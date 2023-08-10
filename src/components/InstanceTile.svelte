<script>
    export let name = "Fallback Name!"
    export let icon = "src/resources/default_instance.png"
    export let lastPlayed = new Date(1)

    /**
     * @type {String}
     */
    export let path

    let hover = false
    let buttonHover = false


    const openInstance = () => {
        if(!buttonHover) console.log(`Opening ${path}`)
    }

    const launchInstance = () => {
        console.log(`Launching ${path}`)
    }

    const enableHover = () => hover = true
    const disableHover = () => hover = false
    const enableButtonHover = () => buttonHover = true
    const disableButtonHover = () => buttonHover = false
</script>

<div
    on:mouseover={enableHover} on:focus={enableHover}
    on:mouseleave={disableHover}
    on:click={openInstance} on:keydown={openInstance}
    class="m-1.5 bg-[var(--bg-secondary)] rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150 inline-grid relative cursor-pointer"
    >
    <div class="rounded-t-lg">
        <img src={icon} alt="" class="w-full rounded-t-lg bg-[#1d1e21] border-4 border-[var(--bg-secondary)]"/>
    </div>
    <div class="whitespace-nowrap overflow-hidden overflow-ellipsis p-1 px-2 font-semibold text-xl text-gray-300">
        {name}
    </div>
    <!-- This time display is temporary, will be moved inside instances later -->
    <div class="whitespace-nowrap overflow-hidden text-sm">
        {lastPlayed}
    </div>
    
    <button 
    on:mouseover={enableButtonHover} on:focus={enableButtonHover}
    on:mouseleave={disableButtonHover}
    on:click={launchInstance} on:keydown={launchInstance}
    class="rounded-b-md absolute bottom-0 font-medium hover:underline overflow-hidden
        {hover ? "w-full text-xl p-1 duration-150 bg-purple-700" : "text-[0px] h-0"}"
    >
        Play
    </button>
</div>