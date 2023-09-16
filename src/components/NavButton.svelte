<script>
    export let href = '#'
    export let text = 'Nothing here?'
    export let show = false

    import '../app.css'
</script>

<a {href} class="cursor-pointer">
    <div class="flex flex-row fill-white text-white font-medium text-lg rounded-lg p-2 hover:bg-purple-700 duration-200">
        <div class="h-[32px] w-[32px] flex-shrink-0 m-0">
            <slot />
        </div> 
        <div class="duration-200 m-0.5 {show ? "opacity-100 w-full ml-2" : "opacity-0 w-0 -ml-4"}">
            {#if show}
                {text}
            {/if}
        </div>
    </div>
</a>