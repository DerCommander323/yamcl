<script>
    import { pickDir, changeSetting } from '../../scripts/settings'
    import { getSetting } from '../../scripts/settings'
    import { onMount } from "svelte"

    let path = 'Loading...'

    onMount(async () => {
        let instancePath = await getSetting('instancePath')
        // @ts-ignore
        path = instancePath?? 'Click to set!'
    })

    async function setInstancePath() {
        /**
         * @type String 
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir==null) return
        changeSetting('instancePath',dir)
        path = dir
    }
</script>

<h1 class="text-[20px] font-bold underline ml-2 mt-2">Settings</h1>

<div class="m-3 p-2 bg-[var(--bg-secondary)] rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150">
    <div on:click={setInstancePath} on:keypress={setInstancePath} class="cursor-pointer">
        <p>Path of the Instance Folder</p>
        <p class="m-1 p-0.5 px-1.5 rounded-md bg-[#222]">{ path }</p>
    </div>
</div>

