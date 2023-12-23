import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'


/**
 * Reads AppSettings from the settings.json File
 * @returns {Promise<AppSettings>} Parsed Settings
 */
export async function readSettings() {
    return await invoke('get_settings')
}

/**
 * Writes  AppSettings to the settings.json File
 * @param {AppSettings} settings New Settings for File
 */
export async function writeSettings(settings) {
    await invoke('update_settings', { newSettings: settings })
}

/**
 * Changes a specific Setting and saves it
 * @param {String} name Name of the Setting
 * @param {any} data New Value for the Setting
 */
export async function changeSetting(name, data) {
    readSettings().then(settings => {
        // @ts-ignore
        settings[name] = data
        writeSettings(settings)
    }).catch(err => {
        console.error(`Failed to change settings: ${err}`)
    })
}

/**
 * Returns a specific Setting
 * @param {String} name Name of the Setting
 */
export async function getSetting(name) {
    let settings = await readSettings()
    // @ts-ignore
    return settings[name]
}

/**
 * Opens a Directory Picker Window
 * @returns {Promise<string | string[] | null>} The user-selected Directory
 */
export async function pickDir() {
    return await open({
        directory: true,
        multiple: false,
        recursive: true
    })
}


