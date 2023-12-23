<script>
    import { launchInstance } from "../scripts/instances"

    /**
     * @type {SimpleInstance}
     */
    export let instance

    let { id, name, mc_version, modloader, icon_path } = instance

    let hover = false
    let buttonHover = false

    let errorCount = 0

    const launch = () => {
        launchInstance(instance)
    }
    
    const onError = (/** @type {any} */ e) => {
        errorCount++
        if (errorCount > 2) return
        e.target.src="default_instance.png"
    }

    const enableHover = () => hover = true
    const disableHover = () => hover = false
    const enableButtonHover = () => buttonHover = true
    const disableButtonHover = () => buttonHover = false
</script>

<a
    href="{buttonHover? "" : `/home/instance/${id}`}"
    on:mouseover={enableHover} on:focus={enableHover}
    on:mouseleave={disableHover}
    class="m-1.5 bg-[var(--bg-secondary)] rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150 inline-grid relative cursor-pointer"
>
    <div class="rounded-t-lg">
        <img on:error={onError} src={icon_path} alt="Instance Icon" class="w-full rounded-t-lg bg-[#1d1e21] border-4 border-[var(--bg-secondary)]"/>
    </div>
    <div class="p-1 text-gray-300 whitespace-nowrap overflow-hidden">
        <p class="text-xl font-semibold w-full overflow-ellipsis overflow-hidden"> { name } </p>
        <div class="flex text-sm w-full">
            <p> { mc_version } </p>
            <p class="pr-1">,</p>
            <p> { modloader.name } </p>
        </div>
    </div>
    
    <button 
        on:mouseover={enableButtonHover} on:focus={enableButtonHover}
        on:mouseleave={disableButtonHover}
        on:click={launch} on:keydown={launch}
        class="rounded-b-md absolute bottom-0 font-medium hover:underline overflow-hidden w-full bg-purple-700 p-1 duration-150
        {hover ? "opacity-90" : "opacity-0" }"
    >
        Play
    </button>
</a>