
<div class="flex flex-row ml-[56px] bg-[var(--bg-main)] text-white h-screen">
    <Navbar />
    <slot />
</div>


<script>
    import '../app.css';
    import Navbar from "../components/Navbar.svelte";
</script>
