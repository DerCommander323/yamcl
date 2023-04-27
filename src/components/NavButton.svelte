<script>
    export let href = '#'
    export let content = 'Empty?'
</script>

<a href="{href}" class="w-full rounded-md p-2 hover:bg-purple-700 duration-200 fill-white hover:fill-black">
    {@html content}
</a>