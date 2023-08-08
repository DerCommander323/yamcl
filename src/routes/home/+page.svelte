<Topbar text="Instances"> amog </Topbar>
<div id="instanceContainer" class="h-fit bg-black">
    <ol id="instances" class="grid">
        {#each instanceList as instance}
            <InstanceTile name={instance.name} icon={instance.icon?? "src/assets/default_instance.png"}></InstanceTile>
        {/each}
    </ol>
</div>

<script>
    import { invoke, convertFileSrc } from "@tauri-apps/api/tauri"
    import { join } from "@tauri-apps/api/path"
    import { listen } from "@tauri-apps/api/event"
    import { onMount, onDestroy } from "svelte"

    import { getSetting } from "../../scripts/settings"
    import InstanceTile from "../../components/InstanceTile.svelte"
    import Topbar from "../../components/Topbar.svelte"

    const prismIcons = [
        'default', 'bee', 'brick', 'chicken', 'creeper', 'diamond', 'dirt', 'enderman', 'enderpearl', 'flame', 'fox', 'gear', 'herobrine',
        'gold', 'grass', 'iron', 'magitech', 'meat', 'modrinth', 'netherstar', 'planks', 'prismlauncher', 'squarecreeper', 'steve', 
        'stone', 'tnt', 'bee_legacy', 'brick_legacy', 'chicken_legacy', 'creeper_legacy', 'diamond_legacy', 'dirt_legacy',
        'enderman_legacy', 'enderpearl_legacy', 'flame_legacy', 'fox_legacy', 'gear_legacy', 'herobrine_legacy', 'gold_legacy', 'grass_legacy', 
        'iron_legacy', 'magitech_legacy', 'meat_legacy', 'modrinth_legacy', 'netherstar_legacy', 'planks_legacy', 'prismlauncher_legacy',
        'squarecreeper_legacy', 'steve_legacy', 'stone_legacy', 'tnt_legacy'
    ]

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
            let ic = event.payload.icon

            if(ic=='' || prismIcons.includes(ic)) {
                event.payload.icon = null
            } else if(ic == 'flame') {
                event.payload.icon = 'https://static-beta.curseforge.com/images/favicon.ico'
            } else if(ic.startsWith("https://media.forgecdn.net")) {
                //do nothing
            } else if(ic == 'curse:666') {
                event.payload.icon = null
            } else if(ic.startsWith('curse:') && ic != 'curse:666') {
                let apiReqeust = await fetch(`https://curserinth-api.kuylar.dev/v2/project/${ic.split(':')[1]}`)
                console.log(`Fetching Icon for project ID ${ic.split(':')[1]} from CurseRinth...`)
                apiReqeust.json().then(json => {
                    console.log(json.icon_url)
                    event.payload.icon = json.icon_url
                })
            } else {
                event.payload.icon = convertFileSrc(await join(iconPath, ic))
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
