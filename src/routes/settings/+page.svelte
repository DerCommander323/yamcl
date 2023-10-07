<Topbar text="Settings"></Topbar>
<div class="h-full bg-[var(--bg-primary)]">
    <!-- Instance Folder Path -->
    <div class="settingsElement">
        <div on:click={setInstancePath} on:keypress={setInstancePath} class="cursor-pointer">
            <p>Path of the Instance Folder</p>
            <p class="m-1 py-0.5 px-1.5 rounded-md bg-[#222]">{ instancePath }</p>
        </div>
    </div>
    <!-- Icon Folder Path -->
    <div class="settingsElement">
        <div on:click={setIconPath} on:keypress={setIconPath} class="cursor-pointer">
            <p>Path of the Icon Folder</p>
            <p class="m-1 py-0.5 px-1.5 rounded-md bg-[#222]">{ iconPath }</p>
        </div>
    </div>
    <!-- Java Path List -->
    <div class="settingsElement">
        <button on:click={toggleJavaDropdown} class="w-full h-8 text-left flex flex-row row hover:underline { javaDropdownExtended ? "underline" : ""}">
            <p class="w-7 duration-200 {javaDropdownExtended ? "rotate-90" : ""}"> <IconArrow /> </p>
            <p class="w-full"> Java Settings </p>
        </button>
        {#if javaDropdownExtended}
            <ul transition:slide id="javaList" class="px-2 py-0.5 m-1 rounded-md bg-[var(--bg-tertiary)]">
                {#each $javaStore as java, index}
                    <button on:click={() => { java.extended = !java.extended; java.mcExtended = false; }} class="my-1.5 flex flex-row w-full hover:underline { java.extended ? "underline" : ""}">
                        <p class="w-7 duration-200 {java.extended ? "rotate-90" : ""}"> <IconArrow /> </p>
                        {java.label}
                    </button>
                    {#if java.extended}
                        <li transition:slide class="p-0.5 my-0.5 flex flex-col border border-[var(--bg-secondary)] hover:border-purple-600 rounded-md duration-150 bg-[var(--bg-secondary)]">
                            <div class="flex flex-row">
                                <div on:click={()=>setJavaPath(index)} on:keydown={()=>setJavaPath(index)} class="flex flex-row w-fit cursor-pointer">
                                    <p class="p-1"> Path: </p>
                                    <p class="bg-[#222] py-0.5 my-0.5 px-1.5 rounded-md"> {java.path} </p>
                                </div>
                                <button class="ml-auto w-6 -mt-2 duration-150 hover:text-red-600" on:click={() => deleteJavaSetting(index)}> <IconMinus /> </button>
                            </div>
                            <div class="flex flex-row">
                                <p class="p-1"> Java Version: </p>
                                <p class="bg-[#222] py-0.5 my-0.5 px-1.5 rounded-md"> {java.version?? 'Checking...'} </p>
                                <button on:click={()=>testJavaVersion(index)} class="bg-purple-700 py-0.5 px-1.5 my-0.5 mx-1 rounded-md"> Test </button>
                            </div>
                            <div class="flex flex-row">
                                <p class="p-1"> Label: </p>
                                <input type="text" bind:value={java.label} on:change={saveJavaSettings} class="bg-[#222] py-0.5 my-0.5 px-1.5 rounded-md">
                            </div>
                            <div class="flex flex-row">
                                <p class="p-1"> Max. Memory Allocation (-Xmx): </p>
                                <input type="number" bind:value={java.xmx} on:change={saveJavaSettings} required min="16" step="16" class="bg-[#222] w-20 my-0.5 text-right rounded-md">
                                <p class="p-1 pl-0">MiB;</p>
                                <p class="p-1 pl-4"> Min. Memory Allocation (-Xms): </p>
                                <input type="number" bind:value={java.xms} on:change={saveJavaSettings} required min="16" step="16"  class="bg-[#222] w-20 my-0.5 text-right rounded-md">
                                <p class="p-1 pl-0">MiB</p>
                            </div>
                            <div class="flex flex-row">
                                <p class="p-1 pr-2 w-min"> Additional Arguments: </p>
                                <textarea bind:value={java.args} on:change={saveJavaSettings} class="bg-[#222] py-0.5 my-0.5 px-1.5 rounded-md w-full resize-y"></textarea>
                            </div>
                            <div class="flex flex-row">
                                <p class="p-1"> Minecraft Version List: </p>
                                <MinecraftList
                                    text="From..."
                                    selected={java.mcVersions?.min} 
                                    dateFilter={(v)=>(java.mcVersions?.max.releaseTime ? new Date(v.releaseTime) < new Date(java.mcVersions?.max.releaseTime) : true)}
                                    on:clicked={(e)=>{updateJavaMcVersions(index, 'min', e.detail.ver)}}
                                />
                                <p class="p-1"> - </p>
                                <MinecraftList
                                    text="To..."
                                    selected={java.mcVersions?.max} 
                                    dateFilter={(v)=>(new Date(v.releaseTime) > new Date(java.mcVersions?.min.releaseTime ?? 0))}
                                    on:clicked={(e)=>{updateJavaMcVersions(index, 'max', e.detail.ver)}}
                                />
                            </div>
                        </li>
                    {/if}
                {/each}
                <li>
                    <button on:click={addJavaSetting} class="my-2 flex flex-row w-full hover:underline"> 
                        <p class="w-7"> <IconPlus /> </p>
                        Add Java Version
                    </button>
                </li>
            </ul>
        {/if}
    </div>
</div>



<script>
    import Topbar from '../../components/Topbar.svelte'
    import MinecraftList from '../../components/MinecraftList.svelte'
    import { pickDir, changeSetting, getSetting } from '../../scripts/settings'
    import { javaStore, getJavaSettings, saveJavaSettings, testJavaVersion, setJavaPath, addJavaSetting, deleteJavaSetting, updateJavaMcVersions } from '../../scripts/javas'
    import { getMinecraftVersions, minecraftVersionList } from '../../scripts/versions'

    import IconArrow from 'svelte-icons/md/MdChevronRight.svelte'
    import IconPlus from 'svelte-icons/md/MdAdd.svelte'
    import IconMinus from 'svelte-icons/md/MdRemoveCircleOutline.svelte'
    import { slide } from 'svelte/transition'
    import { onMount, onDestroy } from "svelte"

    let instancePath = 'Loading...'
    let iconPath = 'Loading...'

    /**
     * @type {typeof minecraftVersionList}
     */
    let mcVersions

    if(!$javaStore || !$javaStore.length) getJavaSettings()

    let javaDropdownExtended = false


    onMount(async () => {
        // idk where to use this yet :) console.error("You like crashing, dont you? \n⠀⢸⠂⠀⠀⠀⠘⣧⠀⠀⣟⠛⠲⢤⡀⠀⠀⣰⠏⠀⠀⠀⠀⠀⢹⡀\n⠀⡿⠀⠀⠀⠀⠀⠈⢷⡀⢻⡀⠀⠀⠙⢦⣰⠏⠀⠀⠀⠀⠀⠀⢸⠀\n⠀⡇⠀⠀⠀⠀⠀⠀⢀⣻⠞⠛⠀⠀⠀⠀⠻⠀⠀⠀⠀⠀⠀⠀⢸⠀\n⠀⡇⠀⠀⠀⠀⠀⠀⠛⠓⠒⠓⠓⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀\n⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠀\n⠀⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⣀⠀⠀⢀⡟⠀\n⠀⠘⣇⠀⠘⣿⠋⢹⠛⣿⡇⠀⠀⠀⠀⣿⣿⡇⠀⢳⠉⠀⣠⡾⠁⠀\n⣦⣤⣽⣆⢀⡇⠀⢸⡇⣾⡇⠀⠀⠀⠀⣿⣿⡷⠀⢸⡇⠐⠛⠛⣿⠀\n⠹⣦⠀⠀⠸⡇⠀⠸⣿⡿⠁⢀⡀⠀⠀⠿⠿⠃⠀⢸⠇⠀⢀⡾⠁⠀\n⠀⠈⡿⢠⢶⣡⡄⠀⠀⠀⠀⠉⠁⠀⠀⠀⠀⠀⣴⣧⠆⠀⢻⡄⠀⠀\n⠀⢸⠃⠀⠘⠉⠀⠀⠀⠠⣄⡴⠲⠶⠴⠃⠀⠀⠀⠉⡀⠀⠀⢻⡄⠀\n⠀⠘⠒⠒⠻⢦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⠞⠛⠒⠛⠋⠁⠀\n⠀⠀⠀⠀⠀⠀⠸⣟⠓⠒⠂⠀⠀⠀⠀⠀⠈⢷⡀⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⠀⠀⠙⣦⠀⠀⠀⠀⠀⠀⠀⠀⠈⢷⠀⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⠀⠀⣼⣃⡀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣆⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⠀⠀⠉⣹⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠀⠀⠀\n⠀⠀⠀⠀⠀⠀⠀⠀⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡆⠀⠀⠀⠀⠀\n")

        // @ts-ignore
        instancePath = await getSetting('instancePath')?? 'Click to set!'
        iconPath = await getSetting('iconPath')?? 'Click to set!'

        mcVersions = await getMinecraftVersions()

        //console.log(mcVersions.versions)
    })

    onDestroy(() => {
        javaDropdownExtended = false
    })

    async function setInstancePath() {
        /**
         * @type String 
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir==null) return
        changeSetting('instancePath', dir)
        instancePath = dir
    }

    async function setIconPath() {
        /**
         * @type String
        */
        //@ts-ignore
        let dir = await pickDir()

        if(dir==null) return
        changeSetting('iconPath', dir)
        iconPath = dir
    }

    function toggleJavaDropdown() {
        javaDropdownExtended = !javaDropdownExtended
        if(javaDropdownExtended) $javaStore.forEach(e => {e.extended = false; e.mcExtended = false})
    }

    
</script>

<style>
    .settingsElement {
        margin: 0.5rem 0.75rem;
        padding: 0.5rem;
        background-color: var(--bg-secondary);
        border-radius: 0.5rem;
        font-size: 18px;
        border-width: 1px;
        border-color: var(--bg-tertiary);
        transition-duration: 150ms;
    }
    .settingsElement:hover {
        border-color: rgb(126 34 206)
    }
</style>
