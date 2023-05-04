<Navbar />
<div class="content ml-[56px] bg-[var(--bg-main)] text-white h-screen">
    <main>
        <slot />
    </main>
</div>


<script>
    import '../app.css';
    import Navbar from "../components/Navbar.svelte";
</script>
