<script>
    import { pickDir, changeSetting } from '../../scripts/settings'
    import { getSettings } from '../../scripts/settings'

    let path = 'Loading...'
    async function init() {
        let settings = await getSettings()
        // @ts-ignore
        path = settings?.instancePath?? 'Click to set!'
        console.log(path)
    }

    async function setInstancePath() {
        /**
         * @type String 
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir!=null) changeSetting('instancePath',dir)
        path = dir
    }

    init()
</script>

<h1 class="text-[20px] font-bold underline ml-2 mt-2">Settings</h1>

<div class="m-3 p-2 bg-[var(--bg-secondary)] rounded-xl text-lg">
    <div on:click={setInstancePath} on:keypress={setInstancePath} class="cursor-pointer">
        <p>Path of the Instance Folder</p>
        <p class="m-1 p-0.5 px-1.5 rounded-md bg-[#222]">{ path }</p>
    </div>
</div>

