<h1 class="text-[20px] font-bold underline ml-2 mt-2">Instances</h1>

<div id="instances" />

<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri";
    import { getSetting } from "../../scripts/settings";

    async function init() {
        let instancePath = await getSetting('instancePath')
        let instances = document.getElementById('instances');

        
        if (instancePath!=null) {
            invoke('get_instances', {path: instancePath})
        }
        
    }
    init()
</script>
