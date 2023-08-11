<Topbar text="Settings"></Topbar>
<div class="h-full bg-[var(--bg-primary)]">
    <div class="m-3 mb-1 bg-[var(--bg-secondary)] p-2 rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150">
        <div on:click={setInstancePath} on:keypress={setInstancePath} class="cursor-pointer">
            <p>Path of the Instance Folder</p>
            <p class="m-1 p-0.5 px-1.5 rounded-md bg-[#222]">{ instancePath }</p>
        </div>
    </div>
    <div class="m-3 mt-1 bg-[var(--bg-secondary)] p-2 rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150">
        <div on:click={setIconPath} on:keypress={setIconPath} class="cursor-pointer">
            <p>Path of the Icon Folder</p>
            <p class="m-1 p-0.5 px-1.5 rounded-md bg-[#222]">{ iconPath }</p>
        </div>
    </div>
</div>



<script>
    import Topbar from '../../components/Topbar.svelte';
    import { pickDir, changeSetting } from '../../scripts/settings'
    import { getSetting } from '../../scripts/settings'
    import { onMount } from "svelte"

    let instancePath = 'Loading...'
    let iconPath = 'Loading...'

    onMount(async () => {
        // @ts-ignore
        instancePath = await getSetting('instancePath')?? 'Click to set!'
        iconPath = await getSetting('iconPath')?? 'Click to set!'
    })

    async function setInstancePath() {
        /**
         * @type String 
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir==null) return
        changeSetting('instancePath',dir)
        instancePath = dir
    }
    async function setIconPath() {
        /**
         * @type String
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir==null) return
        changeSetting('iconPath',dir)
        iconPath = dir
    }

    
</script>
