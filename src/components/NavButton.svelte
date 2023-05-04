<script>
    export let href = '#'
    export let text = 'Nothing here?'
    export let show = false

    import '../app.css'
</script>

<div class="w-full cursor-pointer rounded-md p-2 hover:bg-purple-700 duration-200 fill-white hover:fill-black text-white hover:text-black">
    <a href="{href}" class="fill-inherit inline-flex font-medium text-lg">
        <div class="h-[32px] w-[32px]">
            <slot />
        </div> 
        <p class="m-0.5 ml-2 duration-300 text-white {show ? "opacity-100" : "opacity-0"}">{#if show}{text}{/if}</p>
    </a>
</div>
