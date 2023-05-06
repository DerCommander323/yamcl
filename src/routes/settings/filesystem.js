import { Dir, readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { appConfigDir } from '@tauri-apps/api/path'
import { parse, stringify } from 'yaml'



export async function readSettings() {
    const settingsDir = await appConfigDir()

    return parse(await readTextFile(`${settingsDir}/settings.yaml`))
}

// @ts-ignore
export async function writeSettings(settings) {
    const settingsDir = await appConfigDir()

    await writeTextFile(`${settingsDir}/settings.yaml`, stringify(settings))
}