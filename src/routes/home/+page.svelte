<Topbar text="Instances">
    <div class="py-3 w-full">
        ({instanceList.length})
    </div>
    <div class="p-3">
        <input class="" type="range" min="3" max="27" bind:value={instanceSize} on:change={adjustSize}>
    </div>
</Topbar>
<div id="instanceContainer" class="h-fit bg-black">
    <ol id="instances" class="grid">
        {#each instanceList as instance}
            <InstanceTile name={instance.name} path={instance.path} icon={instance.icon} lastPlayed={instance.last_played} />
        {/each}
    </ol>
</div>

<script>
    import { invoke, convertFileSrc } from "@tauri-apps/api/tauri"
    import { join } from "@tauri-apps/api/path"
    import { listen } from "@tauri-apps/api/event"
    import { onMount, onDestroy } from "svelte"

    import { getSetting, changeSetting } from "../../scripts/settings"
    import InstanceTile from "../../components/InstanceTile.svelte"
    import Topbar from "../../components/Topbar.svelte"
  import { intros } from "svelte/internal";

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
    let unlistenIAdd
    /**
     * @type {[{name:string,icon:string,path:string,last_played:Date,last_played_epoch:0,last_played_string:string}]}
     */
    //@ts-ignore
    let instanceList = []
    let instanceSize = 20

    //Main Code goes in here
    onMount(async () => {
        getSetting('instanceSize').then(v => {
            if(v) instanceSize = v
            adjustSize()
        })

        let instancePath = await getSetting('instancePath')
        let iconPath = await getSetting('iconPath')

        if(iconPath) invoke('unlock_icons', { path: iconPath })
        
        unlistenIAdd = await listen('instance_create', async (event) => {
            const default_icon = 'default_instance.png'
            let ic = event.payload.icon

            if(ic=='' || prismIcons.includes(ic)) {
                event.payload.icon = default_icon
            } else if(ic.startsWith("https://media.forgecdn.net")) {
                //do nothing
            } else if(ic == 'curse:666') {
                event.payload.icon = default_icon
            } else if(ic.startsWith('curse:') && ic != 'curse:666') {
                let apiReqeust = await fetch(`https://curserinth-api.kuylar.dev/v2/project/${ic.split(':')[1]}`)
                console.log(`Fetching Icon for project ID ${ic.split(':')[1]} from CurseRinth...`)
                apiReqeust.json().then(json => {
                    console.log(json.icon_url)
                    event.payload.icon = json.icon_url
                })
            } else {
                event.payload.icon = iconPath ? convertFileSrc(await join(iconPath, ic)) : default_icon
            }

            event.payload.last_played = new Date(event.payload.last_played_epoch>0 ? event.payload.last_played_epoch : event.payload.last_played_string)
            
            // @ts-ignore
            instanceList = [...instanceList, event.payload]
            //needs to be improved
            instanceList = instanceList.sort((a,b) => b.last_played.getTime() - a.last_played.getTime())
            
            console.log(event.payload.last_played)
            
            console.log(new Date(event.payload.last_played))
        })

        if (instancePath!=null) {
            invoke('get_instances', { path: instancePath})
        }
    })

    //Remove event listener on unload
    onDestroy(async () => {
        unlistenIAdd()
        changeSetting('instanceSize', instanceSize)
    })

    //Adjust CSS Grid Columns to window width
    function adjustSize() {
        const width = window.innerWidth-40
        const e = document.getElementById('instances')
        // @ts-ignore
        if (e) e.style.gridTemplateColumns = `repeat(${Math.ceil((width*(30-instanceSize))/2500)}, minmax(0, 1fr))`
    }

    //Adjust CSS Grid Columns on resize
    window.addEventListener('resize', () => {
        adjustSize()
    })

    /*
    window.addEventListener('keypress', key => {
        console.log(key)
        if(key.ctrlKey && key.shiftKey && key.code=="KeyE")
    })
    */

</script>
