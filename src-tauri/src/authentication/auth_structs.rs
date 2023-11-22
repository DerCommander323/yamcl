use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountList {
    pub accounts: Vec<MCAccount>,
    pub selected_index: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCAccount {
    pub xsts_response: XBLResponse,
    pub mc_response: MCResponse,
    pub mc_profile: MCProfile
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCProfile {
    pub id: String,
    pub name: String,
    pub skins: Vec<MCSkin>,
    pub capes: Vec<MCCape>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MCSkin {
    pub id: String,
    pub state: String,
    pub url: String,
    pub variant: String,
    pub texture_key: String,
    pub alias: Option<String>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MCCape {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MSAResponse {
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XBLResponse {
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
    pub display_claims: XBLDisplayClaims
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MCResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub username: String,
    pub token_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entitlements {
    pub items: Vec<Entitlement>,
    pub signature: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Entitlement {
    pub name: String,
    pub source: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XBLDisplayClaims {
    pub xui: Vec<XBLXUIClaims>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct XBLXUIClaims {
    pub uhs: String
}