<script>
    export let text = "unset";
</script>

<header
class="z-10 sticky bg-[var(--bg-secondary)] top-0 flex flex-row h-12 w-full text-white duration-150 border-[var(--bg-tertiary)] border-b"
>
    <div class="underline text-xl font-bold p-2 whitespace-nowrap">
        {text}
    </div>
    <div class="h-full w-full flex flex-row">
        <slot />
    </div>
</header>