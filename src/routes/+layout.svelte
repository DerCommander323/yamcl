<Navbar />
<div class="content">
    <main>
        <slot />
    </main>
</div>

<style>
    .sidebar {
        color: #EEE;
        position: fixed;
        height: 100%;
        width: 70px;
    }
</style>

<script>
    let sidebar = document.querySelector('#sidebar')
    console.log(sidebar) //logs null
    import '../app.css';
    import Navbar from "../components/Navbar.svelte";
</script>
