<h1 class="text-[20px] font-bold underline ml-2 mt-2">Instances</h1>
<div id="instanceContainer" class="h-fit bg-black">
    <ol id="instances" class="grid">
        {#each instanceList as instance}
            <InstanceTile name={instance.name}></InstanceTile>
        {/each}
    </ol>
</div>

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event"
    import { onMount, onDestroy } from "svelte"

    import { getSetting } from "../../scripts/settings"
    import InstanceTile from "../../components/InstanceTile.svelte";

    /**
     * @type {import("@tauri-apps/api/event").UnlistenFn}
     */
    let unlisten
    /**
     * @type {[{name:""}]}
     */
    let instanceList = []

    //Main Code goes in here
    onMount(async () => {
        adjustSize()

        let instancePath = await getSetting('instancePath')
        let instances = document.getElementById('instances')
        
        unlisten = await listen('instance_create', (event) => {
            // @ts-ignore
            instanceList = [...instanceList, event.payload]
            console.log(instanceList)
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
        if (e) e.style.gridTemplateColumns = `repeat(${Math.ceil(w/300)}, minmax(0, 1fr))`
    }

    //Adjust CSS Grid Columns on resize
    window.addEventListener('resize', () => {
        adjustSize()
    })
</script>
