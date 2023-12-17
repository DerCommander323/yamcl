import { invoke } from "@tauri-apps/api"
import { getSetting } from "./settings"
import { emit, listen } from "@tauri-apps/api/event"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { join } from "@tauri-apps/api/path"
import { writable } from "svelte/store"
import { createNotification, finishNotification } from "./notificationSystem"
import { getJavaForVersion } from "./javas"

/**
 * @type {import("svelte/store").Writable<SimpleInstance[]>}
 */
export const instanceStore = writable([])
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
    

    await listen('instance_create', /** @param {import("@tauri-apps/api/event").Event<SimpleInstance>} event */ async (event) => {
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
                event.payload.icon = default_icon
            }
        } else {
            event.payload.icon = iconPath ? convertFileSrc(await join(iconPath, ic)) : default_icon
        }
        
        instanceStore.update(val => [...val, event.payload])
    })

    await invoke('get_instances', { path: instancePath})
}

listen('instance_finish', async (event) => {
    if(instancesFinished) return

    instanceStore.update(instances => {
        if(instances.length == event.payload) {
            finishNotification('instance_gather', `Finished gathering <b class="font-semibold mx-1">${event.payload}</b> Instances!`, 'success')
            instancesFinished = true
            return instances.sort((a,b) => !a.last_played || !b.last_played ? -1 : new Date(b.last_played).getTime() - new Date(a.last_played).getTime())
        } else {
            setTimeout(() => {
                emit('instance_finish', event.payload)
            }, 50)
            return instances
        }
    })

    //if(instanceList.length == event.payload) {
    //    //Sort by last played (needs to be updated once multiple sorting options are added (Soon™))
//
    //} else {
//
    //}
})

/**
 * @param {SimpleInstance} instance
 */
export async function launchInstance(instance) {
    let { name, id, mc_version } = instance
    console.log(`Launching instance: ${name}...`)
    createNotification(`instance_launch_${id}`, `Launching '${name}'...`)
    getJavaForVersion(mc_version).then(async (java) => {
        console.log(`Using java path: ${java.path}, with args ${java.args}`)
        const unlisten = await listen(`notification_${id}_status`, event => {
            console.warn(event)
            finishNotification(`instance_launch_${id}`, event.payload.text, event.payload.status)
        })
        await invoke('launch_instance',
            { instance, java }
        ).catch(e => {
            finishNotification(`instance_launch_${id}`, `Failed to launch instance ${name}: ${e}!`, 'error')
            console.error(e)
        })
    }).catch(e => {
        finishNotification(`instance_launch_${id}`, `Failed to get Java version for Minecraft version ${mc_version}!`, 'error')
        console.error(e)
    })
}

