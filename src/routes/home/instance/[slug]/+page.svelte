<Topbar text="{currentInstance?.name}">
    <div class="w-full py-3">
        <button on:click={openFolder}>
            ({currentInstance?.path})
        </button>
    </div>
    <button on:click={launch} class="bg-purple-700 m-1 rounded-lg p-2 px-3 hover:underline right">Launch</button>
</Topbar>
<div class="m-2 p-1 bg-[var(--bg-secondary)] rounded-lg text-lg h-full">
    Last played: {currentInstance?.last_played}
</div>

<script>
    import Topbar from "../../../../components/Topbar.svelte"
    import { instanceStore, launchInstance } from "../../../../scripts/instances"
    import { shell } from "@tauri-apps/api"

    if(!$instanceStore.length) location.href='/home'

    const currentId = location.href.split('/').pop()
    const currentInstance = $instanceStore.find(e => {return e.id.toString() == currentId})
    
    function launch() {
        if(currentInstance) launchInstance(currentInstance.path)
    }

    function openFolder() {
        if(currentInstance) shell.open(currentInstance.path)
    }
</script>