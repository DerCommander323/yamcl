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

export async function gatherInstances() {
    createNotification('instance_gather', 'Gathering Instances...')

    await invoke('get_instances').then( /** @param {SimpleInstance[]} instances */ instances => {
        console.warn(instances.length)
        onInstanceFinish(instances)
    }).catch(err => {
        finishNotification('instance_gather', `Failed to read instances: ${err}`, 'error')
    })
}

/**
 * @param {SimpleInstance[]} instances
 */
function onInstanceFinish(instances) {
    instances.forEach(i => {
        if(i.icon_path !== "default_instance.png") i.icon_path = convertFileSrc(i.icon_path)
    })
    finishNotification('instance_gather', `Finished gathering <b class="font-semibold mx-1">${instances.length}</b> Instances!`, 'success')
    instancesFinished = true
    instanceStore.set(instances)
}

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

