<aside class=" left-0 top-0 py-4 items-center flex flex-col absolute w-16 bg-neutral-950 h-screen text-white">
    <button class="p-2 bg-neutral-800 rounded-lg hover:bg-neutral-200" on:mouseover={handleHover} on:mouseout={handleOut}>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="{color}" class="bi bi-house" viewBox="0 0 16 16"> <path fill-rule="evenodd" d="M2 13.5V7h1v6.5a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5V7h1v6.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5zm11-11V6l-2-2V2.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5z"/> <path fill-rule="evenodd" d="M7.293 1.5a1 1 0 0 1 1.414 0l6.647 6.646a.5.5 0 0 1-.708.708L8 2.207 1.354 8.854a.5.5 0 1 1-.708-.708L7.293 1.5z"/> </svg>
    </button>
</aside>

<script>
    import '../app.css'

    let color = "white";

    function handleHover() {
        color = "black"
    }

    function handleOut() {
        color = "white"
    }
</script>
