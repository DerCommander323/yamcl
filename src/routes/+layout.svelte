
<div class="flex flex-col ml-[56px] bg-[var(--bg-main)] text-white h-full">
    <Navbar />
    <slot />
</div>


<script>
    import '../app.css';
    import Navbar from "../components/Navbar.svelte";
</script>
