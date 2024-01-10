import { invoke } from "@tauri-apps/api";
import { open, confirm } from "@tauri-apps/api/dialog";
import { writable } from "svelte/store";
import { getMinecraftVersions } from "./versions";
import { changeSetting, getSetting } from "./settings";
import { createNotification, finishNotification } from "./notificationSystem";



/**
 * @type {import("svelte/store").Writable<JavaDetails[]>}
 */
export const javaStore = writable([])

/**
 * @type {JavaDetails[]}
 */
let javaSettings = []

export async function getJavaSettings() {
    javaSettings = await getSetting('java_settings') ?? []
    javaStore.set(javaSettings)
}

export function saveJavaSettings() {
    /**
   * @type {{ path: String, label: String, version: String, minecraft_versions: {min: MCVersion | null, max: MCVersion | null}, xmx: Number, xms: Number, args: String }[]}
   */
    let _savedJavaSettings = []
    javaSettings.forEach(e => {
        const { path, label, version, minecraft_versions, args, xmx, xms } = e
        _savedJavaSettings.push({ path, label, version, minecraft_versions, args, xmx, xms })
    })
    changeSetting('java_settings', _savedJavaSettings)
}

export function addJavaSetting() {
    javaSettings.push({
        // @ts-ignore
        path: 'Click to set!', label: 'New Java', mcVersions: {min:{},max:{}}, args: '', xmx: 4096, xms: 2048, extended: true, version: 'Select Java binary path!', mcExtended: false
    })
    javaStore.set(javaSettings)
}

/**
 * @param {Number} index
 */
export async function deleteJavaSetting(index) {
    let confirmation = await confirm(
        `Are you sure you want to delete '${javaSettings[index].label}'?\nThis Action is irreversible, however you can manually re-add it at any time.`,
        { title: 'Confirm Deletion', type: 'warning'}
    )
    if(confirmation) {
        javaSettings.splice(index, index)
        javaStore.set(javaSettings)
        saveJavaSettings()
    }
}

/**
 * @param {Number} index Index of the Java in javaSettings
 * @param {'min' | 'max'} minmax The Property in the javaSetting
 * @param {any} value The new value for the Property
 */
export function updateJavaMcVersions(index, minmax, value) {
    if(!javaSettings[index].minecraft_versions) javaSettings[index].minecraft_versions = {min: null, max: null}
    javaSettings[index].minecraft_versions[minmax] = value
    javaStore.set(javaSettings)
    saveJavaSettings()
}

/**
 * @param {Number} index Index of the Java in javaSettings
 */
function getJavaArgs(index) {
    let j = javaSettings[index]
    return `-Xmx${j.xmx}M -Xms${j.xms}M ${j.args}`
}

/**
* @param {Number} index Index of the Java in javaSettings
*/
export async function testJavaVersion(index) {
    let java = javaSettings[index]
    createNotification(`java_test_${java.path}`, `Testing Java at ${java.path}...`)
    await invoke('get_java_version', { path: java.path, args: getJavaArgs(index)})
        .then(output => {
            let outputArray = output.replaceAll('"','').replaceAll('\n',' ').split(' ')
            java.version = outputArray[2]
            saveJavaSettings()
            finishNotification(`java_test_${java.path}`, `<div class="flex flex-col"> Java Test succeeded: <code class="bg-[var(--bg-secondary)] text-sm p-1 rounded-md"> ${output.replaceAll('\n',' ')} </code> </div>`, 'success')
        })
        .catch(err => {
            console.error(err)
            java.version = 'Invalid Java!'
            finishNotification(`java_test_${java.path}`, `<div class="flex flex-col"> Java Test failed: <code class="bg-[var(--bg-secondary)] text-sm p-1 rounded-md"> ${err} </code> </div>`, 'error')
        })
    javaStore.set(javaSettings)
}

/**
* @param {String} path The Java binary path
* @param {String} args Args to run it with
* @returns {Promise<String>} The 'java -version' output if successful
*/
export async function getJavaVersion(path, args) {
    return new Promise((resolve, reject) => {
        invoke('get_java_version', { path, args })
            .then(res => {
                res = res.replaceAll('"','').replaceAll('\n',' ')
                let array = res.split(' ')
                resolve(`${array[2]}`)
            })
            .catch(_ => reject(`Failed to get java version for ${path}!`))
    })
}

/**
 * @param {String} mcVer The requested Minecraft version
 * @returns {Promise<JavaDetails>} The Java version to use for mc_ver
 */
export async function getJavaForVersion(mcVer) {
    await getJavaSettings()
    let mcVersions = await getMinecraftVersions()
    let releaseTime = new Date(mcVersions.versions.find(v => v.id == mcVer)?.releaseTime ?? 0).getTime()
    if(releaseTime === 0) return Promise.reject("Invalid Minecraft Version!")
    
    let java = javaSettings.find((java) => {
        if (!java.minecraft_versions.max || !java.minecraft_versions.min) return
        let maxTime = new Date(java.minecraft_versions.max.releaseTime).getTime()
        let minTime = new Date(java.minecraft_versions.min.releaseTime).getTime()

        return maxTime >= releaseTime && minTime <= releaseTime
    })
    if(java) {
        return Promise.resolve(java)
    } else {
        return Promise.reject("Could not find Java version to use for this instance!")
    }
}

/**
* @param {Number} index Index of the Java in javaSettings
*/
export async function setJavaPath(index) {
    /**
     * @type {String | null}
    */
    // @ts-ignore
    let dir = await open({
        multiple: false
    })

    if(!dir) return

    javaSettings[index].path = dir
    testJavaVersion(index)
}
