<h1 class="text-[20px] font-bold underline ml-2 mt-2">Instances</h1>
<div id="instanceContainer" class="h-fit bg-black">
    <ol id="instances" class="grid">
        {#each instanceList as instance}
            <InstanceTile name={instance.name} icon={instance.icon?? "src/components/default_instance.png"}></InstanceTile>
        {/each}
    </ol>
</div>

<script>
    import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
    import { join } from "@tauri-apps/api/path";
    import { listen } from "@tauri-apps/api/event"
    import { onMount, onDestroy } from "svelte"

    import { getSetting } from "../../scripts/settings"
    import InstanceTile from "../../components/InstanceTile.svelte";


    /**
     * @type {import("@tauri-apps/api/event").UnlistenFn}
     */
    let unlisten
    /**
     * @type {[{name:"",icon:""}]}
     */
    //@ts-ignore
    let instanceList = []

    //Main Code goes in here
    onMount(async () => {
        adjustSize()

        let instancePath = await getSetting('instancePath')
        let iconPath = await getSetting('iconPath')
        let instances = document.getElementById('instances')

        invoke('unlock_icons', { path: iconPath })
        
        unlisten = await listen('instance_create', async (event) => {
            switch (event.payload.icon) {
                case "": event.payload.icon = null; break;
                case "default": break;
                default: event.payload.icon = convertFileSrc(await join(iconPath, event.payload.icon)); break;
            }
            // @ts-ignore
            instanceList = [...instanceList, event.payload]
        })

        if (instancePath!=null) {
            invoke('get_instances', { path: instancePath})
        }
    })

    //Remove event listener on unload
    onDestroy(async () => {
        unlisten()
    })

    //Adjust CSS Grid Columns to window width
    function adjustSize() {
        const w = window.innerWidth-40
        const e = document.getElementById('instances')
        if (e) e.style.gridTemplateColumns = `repeat(${Math.ceil(w/250)}, minmax(0, 1fr))`
    }

    //Adjust CSS Grid Columns on resize
    window.addEventListener('resize', () => {
        adjustSize()
    })
</script>
