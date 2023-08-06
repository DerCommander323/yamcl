import { readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { appConfigDir } from '@tauri-apps/api/path'
import { open } from '@tauri-apps/api/dialog'
import { parse, stringify } from 'yaml'


/**
 * Reads Settings {} from File
 * @returns {Promise<{}>} Parsed Settings from File
 */
export async function readSettings() {
    const settingsDir = await appConfigDir()
    return parse(await readTextFile(`${settingsDir}/settings.yaml`))?? {}
}

/**
 * Writes Settings {} to File
 * @param {{}} settings New Settings for File
 */
export async function writeSettings(settings) {
    const settingsDir = await appConfigDir()
    await writeTextFile(`${settingsDir}/settings.yaml`, stringify(settings))
}

/**
 * Changes a specific Setting and saves it
 * @param {String} name Name of the Setting
 * @param {String} data New Value of the Setting
 */
export async function changeSetting(name, data) {
    readSettings().then(settings => {
        // @ts-ignore
        settings[name] = data
        writeSettings(settings)
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
    const selected = await open({
        directory: true,
        multiple: false,
        recursive: true
    })
    return selected
}


