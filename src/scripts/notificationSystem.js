import { writable } from "svelte/store";


/**
 * @type {import("svelte/store").Writable<{id: string, contents: string, status: 'running' | 'error' | 'success'}[]>}
 */
export const notificationStore = writable([])

/**
 * @type {Object.<string,{contents: string, status: 'running' | 'error' | 'success'}>}
 */
let notifs = {}



/**
 * @param {string} id
 * @param {string} contents
 * @param {'running' | 'error' | 'success'} [status='running']
 */
export function createNotification(id, contents, status) {
    notifs[id] = { contents, status: status ?? 'running'}
    updateNotifStore()
}

/**
 * @param {string} id
 * @param {string} newContents
 * @param {'error' | 'success' | undefined} newStatus
 */
export function finishNotification(id, newContents, newStatus) {
    let status = newStatus ?? 'success'
    notifs[id] = { contents: newContents, status }
    updateNotifStore()
    setTimeout(async () => {
        delete notifs[id]
        updateNotifStore()
    }, status === 'success' ? 2500 : 10000)
}

function updateNotifStore() {
    let list = []
    for(let id of Object.keys(notifs)) {
        list.push({id, ...notifs[id]})
    }
    notificationStore.set(list)
}

