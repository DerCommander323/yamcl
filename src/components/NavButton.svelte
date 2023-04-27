<script>
    export let href = '#'
    export let text = 'Nothing here?'
    export let show = false

    import '../app.css'
</script>

<div class="w-full cursor-pointer rounded-md p-2 hover:bg-purple-700 duration-200 fill-white hover:fill-black">
    <a href="{href}" class="fill-inherit">
        <slot /> 
        {#if show}
        {text}
        {/if}
    </a>
</div>