<div class="relative inline-block">
    <button on:click={() => { expanded = !expanded }} class="bg-[#222] p-1 flex flex-row { expanded ? "rounded-b-md " : "rounded-md" }" >
        <p class="w-7 duration-200 { expanded ? "-rotate-90" : ""}"> <IconArrow /> </p>
        <p class="hover:underline { expanded ? "underline" : "" }"> { selected.version ?? text } </p>
    </button>
    <div class="absolute bottom-full { expanded ? "border" : "" } border-[var(--bg-tertiary)] bg-[var(--bg-secondary)] hover:border-purple-700 w-full min-w-[6.9rem]">
        {#if expanded}
            <div class="filters text-sm">
                <p> <input type="checkbox" bind:checked={filters.releases} class="mx-1"> Releases </p>
                <p> <input type="checkbox" bind:checked={filters.snapshots} class="mx-1"> Snapshots </p>
                <p> <input type="checkbox" bind:checked={filters.experiments} class="mx-1"> Experiments </p>
                <p> <input type="checkbox" bind:checked={filters.betas} class="mx-1"> Betas </p>
                <p> <input type="checkbox" bind:checked={filters.alphas} class="mx-1"> Alphas </p>
                <div class="p-1">
                    <!-- svelte-ignore a11y-autofocus -->
                    <input type="text" placeholder="Filter..." bind:value={filters.name} autofocus class="bg-[#222] w-full p-0.5">
                </div>
            </div>
            <ol class="bg-[#222] h-56 overflow-y-scroll w-full">
                {#if loaded}
                    {#each filteredVersions as v, i}
                        <li>
                            <p on:click={()=>select(i)} on:keydown={()=>select(i)} class="hover:bg-[#333] cursor-pointer break-words"> {v.version} </p>
                        </li>
                    {/each}
                {/if}
            </ol>
        {/if}
    </div>
</div>

<script>
    import IconArrow from 'svelte-icons/md/MdChevronRight.svelte'
    import { createEventDispatcher } from "svelte"
    import { getMinecraftVersions, minecraftVersionList } from "../scripts/versions"

    export let dateFilter = (/** @type {typeof minecraftVersionList.versions[0]} */ _) => true
    /**
     * @type {typeof minecraftVersionList.versions[0]}
     */
    // @ts-ignore
    export let selected = {}
    export let text = 'Click to select!'

    /**
     * @type {typeof minecraftVersionList}
     */
    let mcVersions
    /**
     * @type {typeof minecraftVersionList.versions}
     */
    let filteredVersions = []
    let filters = {
        name: '',
        releases: true,
        snapshots: false,
        experiments: false,
        betas: false,
        alphas: false
    }
    let loaded = false
    let expanded = false
    let dispatch = createEventDispatcher()

    ;(async () => { 
        mcVersions = await getMinecraftVersions()
        filteredVersions = mcVersions.versions
        loaded = true
    })()

    $: {
        if(mcVersions) {
            filteredVersions = mcVersions.versions.filter(v => {
                return (
                    v.version.toLowerCase().includes(filters.name.toLowerCase()) &&
                    dateFilter(v) &&
                    (
                        (filters.releases && v.type=='release') ||
                        (filters.snapshots && v.type=='snapshot') ||
                        (filters.snapshots && v.type=='old_snapshot') ||
                        (filters.experiments && v.type=='experiment') ||
                        (filters.betas && v.type=='old_beta') ||
                        (filters.alphas && v.type=='old_alpha')
                    )
                )
            })
        }
    }

    /**
   * @param {number} i
   */
    function select(i) {
        selected = filteredVersions[i]
        expanded = false
        send(i)
    }

    /**
   * @param {number} i Index
   */
    function send(i) {
        dispatch('clicked', { ver: selected })
    }
</script>