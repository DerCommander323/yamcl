<Topbar text="Instances">
    <div class="py-3 w-full">
        ({!instancesFinished ? "Scanning..." : $instanceStore.length})
    </div>
    <div class="p-3">
        <input class="" type="range" min="3" max="27" bind:value={instanceSize} on:change={adjustSize}>
    </div>
</Topbar>
<div id="instanceContainer" class="h-fit bg-[var(--bg-primary)]">
    <ol id="instances" class="grid opacity-0">
        {#key $instanceStore}
            {#if instancesFinished}
                {#each $instanceStore as instance}
                    <InstanceTile {instance} />
                {/each}
            {/if}
        {/key}
    </ol>
</div>

<script>
    import { onMount } from "svelte"

    import { getSetting, changeSetting } from "../../scripts/settings"
    import { instanceStore, gatherInstances, instancesFinished } from "../../scripts/instances"
    import InstanceTile from "../../components/InstanceTile.svelte"
    import Topbar from "../../components/Topbar.svelte"

    let instanceSize = 18

    //Main Code goes in here
    onMount(async () => {
        await getSetting('instance_size').then(v => {
            if(v) instanceSize = v
            adjustSize()
        }).catch(console.warn)

        if(!instancesFinished) gatherInstances()
    })

    //Adjust CSS Grid Columns to window width
    function adjustSize() {
        const width = window.innerWidth-40
        const e = document.getElementById('instances')
        // @ts-ignore
        if (e) {
            e.style.gridTemplateColumns = `repeat(${Math.ceil((width*(30-instanceSize))/2500)}, minmax(0, 1fr))`
            e.style.opacity = "100"
        }
        changeSetting('instance_size', instanceSize)
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
