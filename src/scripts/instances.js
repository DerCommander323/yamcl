import { invoke } from "@tauri-apps/api"
import { getSetting } from "./settings"
import { emit, listen } from "@tauri-apps/api/event"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { join } from "@tauri-apps/api/path"
import { writable } from "svelte/store"
import { createNotification, finishNotification } from "./notificationSystem"
import { getJavaForVersion } from "./javas"

/**
 * @type {import("svelte/store").Writable<{name: string, icon: string, path: string, id: number, last_played: Date, modloader: {name: string, version: string, typ: string}, mc_version: string}[]>}
 */
export const instanceStore = writable([])
/**
 * @type {{name: string, icon: string, path: string, id: number, last_played: Date, modloader: {name: string, version: string, typ: string}, mc_version: string}[]}
 */
let instanceList = []
export let instancesFinished = false

const prismIcons = [
    'default', 'bee', 'brick', 'chicken', 'creeper', 'diamond', 'dirt', 'enderman', 'enderpearl', 'flame', 'fox', 'gear', 'herobrine',
    'gold', 'grass', 'iron', 'magitech', 'meat', 'modrinth', 'netherstar', 'planks', 'prismlauncher', 'squarecreeper', 'steve', 
    'stone', 'tnt', 'bee_legacy', 'brick_legacy', 'chicken_legacy', 'creeper_legacy', 'diamond_legacy', 'dirt_legacy',
    'enderman_legacy', 'enderpearl_legacy', 'flame_legacy', 'fox_legacy', 'gear_legacy', 'herobrine_legacy', 'gold_legacy', 'grass_legacy', 
    'iron_legacy', 'magitech_legacy', 'meat_legacy', 'modrinth_legacy', 'netherstar_legacy', 'planks_legacy', 'prismlauncher_legacy',
    'squarecreeper_legacy', 'steve_legacy', 'stone_legacy', 'tnt_legacy'
]

export async function gatherInstances() {
    let instancePath = await getSetting('instancePath')
    let iconPath = await getSetting('iconPath')

    if(iconPath) await invoke('unlock_icons', { path: iconPath })

    createNotification('instance_gather', 'Gathering Instances...')

    if(!instancePath) {
        finishNotification('instance_gather', 'Instance gathering failed: The instance folder is unset! Go to the Settings to set it.', 'error')
        return 
    } else if(!await invoke('file_exists', { path: instancePath })) {
        finishNotification('instance_gather', 'Instance gathering failed: The instance folder does not exist!', 'error')
        return
    }
    
    await listen('instance_create', async (event) => {
        //Icon handling
        const default_icon = 'default_instance.png'
        let ic = event.payload.icon

        if(ic==='' || ic==='curse:666' || prismIcons.includes(ic)) {
            event.payload.icon = default_icon
        } else if(ic.startsWith("https://media.forgecdn.net")) {
            //do nothing
        } else if(ic.startsWith('curse:')) {
            console.log(`Fetching Icon for project ID ${ic.split(':')[1]} from CurseRinth...`)
            try {
                let apiReqeust = await fetch(`https://curserinth-api.kuylar.dev/v2/project/${ic.split(':')[1]}`)

                apiReqeust.json().then(json => {
                    console.log(json.icon_url)
                    event.payload.icon = json.icon_url
                })
            } catch (e) {
                console.error(`Failed to make icon request: ${e}`)
            }
        } else {
            event.payload.icon = iconPath ? convertFileSrc(await join(iconPath, ic)) : default_icon
        }

        //Date handling
        event.payload.last_played = new Date(event.payload.last_played['Epoch'] ?? event.payload.last_played['String'])
        
        instanceList.push(event.payload)
    })

    invoke('get_instances', { path: instancePath})
}

listen('instance_finish', async (event) => {
    if(instancesFinished) return
    if(instanceList.length == event.payload) {
        //Sort by last played (needs to be updated once multiple sorting options are added (Soon™))
        instanceList = instanceList.sort((a,b) => b.last_played.getTime() - a.last_played.getTime())
        instanceStore.set(instanceList)
        finishNotification('instance_gather', `Finished gathering <b class="font-semibold mx-1">${event.payload}</b> Instances!`, 'success')
        instancesFinished = true
    } else {
        setTimeout(() => {
            emit('instance_finish', event.payload)
        }, 50)
    }
})

/**
 * @param {String} mcPath
 * @param {String} instanceName
 * @param {String} mcVer
 * @param {String} loaderVersion
 * @param {String} loader
 * @param {Number} instanceId
 */
export async function launchInstance(mcPath, instanceName, mcVer, instanceId, loaderVersion, loader) {
    console.log(`Launching instance: ${instanceName}...`)
    createNotification(`instance_launch_${instanceId}`, `Launching '${instanceName}'...`)
    getJavaForVersion(mcVer).then(async java => {
        console.log(`Using java path: ${java.path}, with args ${java.args}`)
        const unlisten = await listen(`notification_${instanceId}_status`, event => {
            console.warn(event)
            finishNotification(`instance_launch_${instanceId}`, event.payload.text, event.payload.status)
        })
        await invoke('launch_instance', {
            minecraftPath: mcPath, versionId: mcVer, javaPath: java.path, additionalArgs: java.args, instanceId, loaderVersion, loader
        })
    }).catch(e => {
        finishNotification(`instance_launch_${instanceId}`, `Failed to get Java version for Minecraft version ${mcVer}!`, 'error')
        console.error(e)
    })
}

