import { invoke } from "@tauri-apps/api";
import { confirm } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";
import { writable } from "svelte/store";
import { createNotification, finishNotification } from "./notificationSystem";



export const accountStore = writable([])
export const accountState = writable('loading')
export const skinIndex = writable(-1)
export const selectedIndex = writable(-1)
export const previewIndex = writable(-1)

export function addAccount() {
    invoke('add_account')
}

/**
 * @param {Number} index
 */
export async function setSelectedIndex(index) {
    await invoke('set_selected_account', { index })
    selectedIndex.set(index)
}

/**
 * @param {Number} index
 */
export function setPreviewIndex(index) {
    previewIndex.set(index)
    skinIndex.set(0)
}

/**
 * @param {Number} index
 * @param {Number} length 
 */
export function setSkinIndex(index, length) {
    if(index > length-1 || index < 0) return
    skinIndex.set(index)
}

/**
 * @param {Number} index
 * @param {String} name
 */
export async function removeAccount(index, name) {
    let confirmation = await confirm(
        `Are you sure you want to delete the account '${name}'?\nThis Action is irreversible, however you can re-add it at any time.`,
        { title: 'Confirm Deletion', type: 'warning'}
    )
    if(confirmation) invoke('remove_account', { index }).then(loadAccounts)
}

export function loadAccounts() {
    console.log('Loading accounts...')
    invoke('get_selected_account')
        .then(i => {
            selectedIndex.set(i)
            setPreviewIndex(i)
        })
    invoke('load_accounts')
        .catch(e => {
            console.error('Error occured while loading accounts: ' + e)
            accountState.set('errored')
        })
        .then(accs => {
            accountStore.set(accs)
            accountState.set('success')
        })
}

listen('login_status', event => {
    if(event.payload === 'Successfully added new account!') {
        finishNotification('login_status', event.payload, 'success')
        loadAccounts()
    } else {
        createNotification('login_status', `Logging in: ${event.payload}`)
    }
})