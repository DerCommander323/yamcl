<h1 class="text-[20px] font-bold underline ml-2 mt-2">Instances</h1>

<div id="instances" />

<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri";
    import { getSettings } from "../../scripts/settings";

    async function init() {
        let settings = await getSettings();
        let instances = document.getElementById("instances");

        console.log(settings);
        
        if (settings.instancePath!=null) {
            invoke('my_custom_command', { path: settings.instancePath }).then(data => {
                console.log(data)
                let p = document.createElement('p')
                p.innerText=data
                instances.append(p)
            })
        }
        
    }
    init()
</script>
