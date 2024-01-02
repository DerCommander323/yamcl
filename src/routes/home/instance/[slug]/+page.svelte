<Topbar text="{currentInstance?.name}">
    <div class="w-full my-3 overflow-ellipsis overflow-hidden">
        <button on:click={openFolder} class="break-all">
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
        if(currentInstance) {
            let { path, name, mc_version, id } = currentInstance
            launchInstance(path, name, mc_version, id)
        }
    }

    function openFolder() {
    if (currentInstance && currentInstance.path) {
        const folderPath = currentInstance.path.replace(/\\/g, '/'); // Replace backslashes with forward slashes
        shell.open(folderPath);
    }
}
</script>