<h1 class="text-[20px] font-bold underline ml-2 mt-2">Settings</h1>

<div class="m-3 p-2 bg-[var(--bg-secondary)] rounded-xl text-lg">
    <div on:click={setInstancePath} on:keypress={setInstancePath} class="cursor-pointer">
        <p>Path of the Instance Folder</p>
        <p class="m-1 p-0.5 rounded-md bg-[#222]">{ settings.instancePath?? 'Click to set!' }</p>
    </div>
</div>




<script>
    let settings = {}
    readSettings().then(data => {settings=data})

    import { pickDir } from './folder_selector'
    import { readSettings, writeSettings } from './filesystem'

    function setInstancePath() {
        pickDir().then(p => {
            // @ts-ignore
            if(p!=null) settings.instancePath=p
            updateSettings()
        })
    }

    // @ts-ignore
    function updateSettings() {
        if(settings) writeSettings(settings)
    }

    readSettings()
    
</script>
