<script>
    export let name = "Fallback Name!"
    export let icon = "src/components/default_instance.png"

    let hover = false

    const enableHover = () => hover = true
    const disableHover = () => hover = false
</script>

<div
    on:mouseover={enableHover} on:focus={enableHover}
    on:mouseleave={disableHover}
    class="m-2 bg-[var(--bg-secondary)] rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150 inline-grid relative"
    >
    <div class="rounded-t-lg">
        <img src={icon} alt="" class="w-full rounded-t-lg bg-[#1d1e21] border-4 border-[var(--bg-secondary)]"/>
    </div>
    <div class="whitespace-nowrap overflow-hidden overflow-ellipsis p-1.5 font-semibold text-xl text-gray-300">
        {name}
    </div>
    
    <button 
    class="rounded-b-md absolute bottom-0 font-medium hover:underline overflow-hidden
        {hover ? "w-full text-xl p-1 duration-150 bg-purple-700" : "text-[0px] h-0"}"
    >
        Play
    </button>
</div>