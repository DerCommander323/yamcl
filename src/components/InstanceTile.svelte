<script>
    export let name = "Fallback Name!"
    export let icon = "src/components/default_instance.png"
</script>

<div class="m-2 bg-[var(--bg-secondary)] rounded-lg text-lg border border-[var(--bg-secondary)] hover:border-purple-700 duration-150 inline-grid">
    <div class="rounded-t-lg">
        <img src={icon} alt="" class="w-full rounded-t-lg bg-[#1d1e21] border-4 border-[var(--bg-secondary)]"/>
    </div>
    <div class="whitespace-nowrap overflow-hidden overflow-ellipsis p-1.5 font-semibold text-xl text-gray-300">
        {name}
    </div>
</div>