<script>
    export let text = "unset";
</script>

<header
class="z-10 bg-[var(--bg-secondary)] top-0 p-2 px-4 flex flex-row h-12 w-full text-white duration-150 border-[var(--bg-tertiary)] border-b"
>
    <div class="underline text-xl font-bold ">
        {text}
    </div>
    <div class=" how do i move it to the end">
        <slot />
    </div>
</header>