import { invoke } from "@tauri-apps/api"
import { getSetting } from "./settings"
import { emit, listen } from "@tauri-apps/api/event"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { join } from "@tauri-apps/api/path"
import { writable } from "svelte/store"

/**
 * @type {import("svelte/store").Writable<{name:string,icon:string,path:string,last_played:Date,last_played_epoch:0,last_played_string:string}[]>}
 */
//@ts-ignore
export const instanceStore = writable([])
/**
 * @type {{name:string,icon:string,path:string,id:0,last_played:Date,last_played_epoch:0,last_played_string:string}[]}
 */
//@ts-ignore
let instanceList = []
/**
 * @type {{name:string,icon:string,path:string,id:0,last_played:Date,last_played_epoch:0,last_played_string:string}}
 */
export let selectedInstance

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
    
    await listen('instance_create', async (event) => {
        //Icon handling
        const default_icon = 'default_instance.png'
        let ic = event.payload.icon

        if(ic=='' || prismIcons.includes(ic)) {
            event.payload.icon = default_icon
        } else if(ic.startsWith("https://media.forgecdn.net")) {
            //do nothing
        } else if(ic == 'curse:666') {
            event.payload.icon = default_icon
        } else if(ic.startsWith('curse:') && ic != 'curse:666') {
            let apiReqeust = await fetch(`https://curserinth-api.kuylar.dev/v2/project/${ic.split(':')[1]}`)
            console.log(`Fetching Icon for project ID ${ic.split(':')[1]} from CurseRinth...`)
            apiReqeust.json().then(json => {
                console.log(json.icon_url)
                event.payload.icon = json.icon_url
            })
        } else {
            event.payload.icon = iconPath ? convertFileSrc(await join(iconPath, ic)) : default_icon
        }

        //Date handling
        event.payload.last_played = new Date(event.payload.last_played_epoch>0 ? event.payload.last_played_epoch : event.payload.last_played_string)
        
        instanceList.push(event.payload)
    })

    if (instancePath!=null) {
        await invoke('get_instances', { path: instancePath})
    }
}

listen('instance_finish', async (event) => {
    let unfinished = true
    if(instanceList.length == event.payload) {
        //Sort by last played (needs to be updated once multiple sorting options are added (Soon™))
        instanceList = instanceList.sort((a,b) => b.last_played.getTime() - a.last_played.getTime())
        instanceStore.set(instanceList)
        unfinished = false
    } else {
        setTimeout(() => {
            emit('instance_finish', event.payload)
        }, 20);
    }
})