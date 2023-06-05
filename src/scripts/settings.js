import { readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { appConfigDir } from '@tauri-apps/api/path'
import { open } from '@tauri-apps/api/dialog'
import { parse, stringify } from 'yaml'

/**
 * @type {{}}
 */
let settings

export async function getSettings() {
    if (!settings) {settings = await readSettings(); return settings}
    else return settings
}

export async function readSettings() {
    const settingsDir = await appConfigDir()
    return parse(await readTextFile(`${settingsDir}/settings.yaml`))?? {}
}

/**
 * @param {Object} settings
 */
export async function writeSettings(settings) {
    const settingsDir = await appConfigDir()
    await writeTextFile(`${settingsDir}/settings.yaml`, stringify(settings))
}


/**
 * @param {String} name
 * @param {String} data
 */
export async function changeSetting(name, data) {
    readSettings().then(settings => {
        settings[name] = data
        writeSettings(settings).then(() => {
            return
        })
    })
}


// Open a selection dialog for directories
 export async function pickDir() {
    const selected = await open({
        directory: true,
        multiple: false,
    })
    return selected
}


