<h1 class="text-[20px] font-bold underline ml-2 mt-2">Instances</h1>

<div id="instances" />

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { emit, listen } from "@tauri-apps/api/event"


    import { getSetting } from "../../scripts/settings"

    async function init() {
        let instancePath = await getSetting('instancePath')
        let instances = document.getElementById('instances');

        
        const unlisten = await listen('instance_create', (event) => {
            console.log(event.payload.name)
        })

        if (instancePath!=null) {
            invoke('get_instances', { path: instancePath})
        }
    }

    init()
</script>
