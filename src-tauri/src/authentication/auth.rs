use afire::{Server, Method, Response, Status};
use reqwest::blocking::Client;
use serde_json::json;
use tauri::AppHandle;
use crate::{auth_structs::{MCAccount, MCProfile, Entitlements, MCResponse, XBLResponse, MSAResponse}, notify, NotificationState, authentication::accounts::save_new_account};

const MS_CLIENT_ID: &str = "5431ff2d-20f8-415b-aa2f-5218eba055ea"; // The Yet Another MC Launcher client_id. If you fork this project, please make sure to use your own!
const REDIRECT_PORT: u16 = 32301;

fn get_login_url() -> String {
    String::from_iter([
        "https://login.live.com/oauth20_authorize.srf?client_id=",
        MS_CLIENT_ID,
        "&prompt=select_account",
        //"&cobrandid=8058f65d-ce06-4c30-9559-473c9275a65d",
        "&response_type=code",
        "&scope=XboxLive.signin%20XboxLive.offline_access",
        "&redirect_uri=http%3A%2F%2F127.0.0.1%3A",
        &REDIRECT_PORT.to_string()
    ])
}
fn get_msa_url() -> String {
    String::from("https://login.live.com/oauth20_token.srf")
}
fn get_xbl_url() -> String {
    String::from("https://user.auth.xboxlive.com/user/authenticate")
}
fn get_xsts_url() -> String {
    String::from("https://xsts.auth.xboxlive.com/xsts/authorize")
}
fn get_mc_url() -> String {
    String::from("https://api.minecraftservices.com/launcher/login")
}
fn get_entitlements_url() -> String {
    String::from("https://api.minecraftservices.com/entitlements/license")
}
fn get_mc_profile_url() -> String {
    String::from("https://api.minecraftservices.com/minecraft/profile")
}



#[tauri::command(async)]
pub fn add_account(app_handle: AppHandle) {
    let mut redirect_server = Server::<()>::new("127.0.0.1", REDIRECT_PORT);

    let login_window = tauri::WindowBuilder::new(
        &app_handle,
        "yamcl_microsoft_login",
        tauri::WindowUrl::External(get_login_url().parse().unwrap())
    )
    .title("Microsoft Login")
    .inner_size(500.0, 600.0)
    .center()
    .focused(true)
    .build()
    .unwrap();

    notify(&app_handle, "login_status", "Awaiting login", NotificationState::Running);

    std::thread::spawn(move || {
        let app_handle_clone = app_handle.clone();
        login_window.on_window_event(move |event| {
            if let tauri::WindowEvent::Destroyed = event {
                notify(&app_handle_clone, "login_status", "Login aborted!", NotificationState::Error);
            }
        });

        let app_handle_clone = app_handle.clone();
        redirect_server.route(Method::GET, "/", move |req| {
            if let Some(code) = req.query.get("code") {
                println!("Code obtained!");
                notify(&app_handle, "login_status", "Beginning login process...", NotificationState::Running);
                login_window.close().unwrap();
                add_account_code(code, &app_handle_clone);
                Response::new()
                    .text("You may close this window now.")
                    .status(Status::Ok)
            } else {
                println!("Getting Code failed!");
                notify(&app_handle_clone, "login_status", "Failed getting code from response!", NotificationState::Error);
                Response::new()
                    .text("Failed to get the authentication code!")
                    .status(Status::NotFound)
            }
        });

        println!("Starting auth redirect HTTP server on port {REDIRECT_PORT}...");
        if let Err(e) = redirect_server.start() {
            println!("Starting redirect server failed: {e}")
        };
    });
}

fn add_account_code(code: &str, app_handle: &AppHandle) {
    println!("Started adding new Minecraft account!");
    let client = reqwest::blocking::Client::new();

    println!("Getting Microsoft Auth response...");
    notify(&app_handle, "login_status", "Getting Microsoft Auth reponse...", NotificationState::Running);

    let msa_params = [
        ("client_id", MS_CLIENT_ID),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", &String::from_iter(["http://127.0.0.1:", &REDIRECT_PORT.to_string()])),
        ("scope", "XboxLive.signin XboxLive.offline_access")
    ];
    let msa_response: MSAResponse = client.post(get_msa_url())
        .form(&msa_params)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{:#?}", msa_response);
    println!("Getting Xbox Live Auth response...");
    notify(&app_handle, "login_status", "Getting Xbox Live Auth reponse...", NotificationState::Running);

    let xbl_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": &String::from_iter(["d=", &msa_response.access_token])
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    let xbl_response: XBLResponse = client.post(get_xbl_url())
        .json(&xbl_json)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{:#?}", xbl_response);
    println!("Getting Xsts Auth response...");
    notify(&app_handle, "login_status", "Getting Xsts Auth reponse...", NotificationState::Running);

    let xsts_json = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [
                xbl_response.token
            ]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    });
    let xsts_response: XBLResponse = client.post(get_xsts_url())
    .json(&xsts_json)
    .send()
    .unwrap()
    .json()
    .unwrap();

    // println!("{:#?}", xsts_response);
    println!("Getting Minecraft Auth response...");
    notify(&app_handle, "login_status", "Getting Minecraft Auth reponse...", NotificationState::Running);

    let mc_json = json!({
        "xtoken": String::from_iter(["XBL3.0 x=", &xsts_response.display_claims.xui[0].uhs, ";", &xsts_response.token]),
        "platform": "PC_LAUNCHER"
    });
    let mc_response: MCResponse = client.post(get_mc_url())
    .json(&mc_json)
    .send()
    .unwrap()
    .json()
    .unwrap();

    // println!("{:#?}", mc_response);
    println!("Checking Minecraft ownership...");
    notify(&app_handle, "login_status", "Checking Minecraft ownership...", NotificationState::Running);

    if !has_mc_ownership(&client, &mc_response.access_token) {
        notify(&app_handle, "login_status", "Account does not own Minecraft!", NotificationState::Error);
        return;
    }

    println!("Getting Minecraft account...");
    notify(&app_handle, "login_status", "Getting Minecraft account...", NotificationState::Running);
    let mc_profile = get_mc_profile(&client, &mc_response.access_token);

    // println!("{:#?}", mc_profile);
    let mc_account = MCAccount {
        xsts_response,
        mc_response,
        mc_profile
    };
    let username = mc_account.mc_profile.name.clone();

    //println!("{:#?}", mc_account);
    println!("Saving new Minecraft account...");
    notify(&app_handle, "login_status", "Saving new account...", NotificationState::Running);
    if let Err(e) = save_new_account(mc_account) {
        println!("Error occured while saving new account: {e}")
    }

    notify(&app_handle, "login_status", &String::from_iter(["Successfully added account \"", &username, "\"!"]), NotificationState::Success);
    println!("Successfully added new account.");
}


fn has_mc_ownership(client: &Client, access_token: &str) -> bool {
    let entitlements_response: Entitlements = client.get(
        String::from_iter([&get_entitlements_url(), "?requestId=", &uuid::Uuid::new_v4().to_string()])
    )
    .header("Authorization", String::from_iter(["Bearer ", &access_token]))
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .send()
    .unwrap()
    .json()
    .unwrap();
    
    // println!("{:#?}", entitlements_response);
    entitlements_response.items.iter().any(|item| 
        item.name.eq_ignore_ascii_case("product_minecraft") || item.name.eq_ignore_ascii_case("game_minecraft")
    )
}

fn get_mc_profile(client: &Client, access_token: &str) -> MCProfile {
    let mcprofile_response: MCProfile = client.get(get_mc_profile_url())
    .header("Authorization", String::from_iter(["Bearer ", access_token]))
    .send()
    .unwrap()
    .json()
    .unwrap();

    mcprofile_response
}
