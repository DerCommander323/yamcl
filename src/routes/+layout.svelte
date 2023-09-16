
<div class="flex flex-col ml-[56px] bg-[var(--bg-main)] text-white h-full">
    <Navbar />
    <NotificationContainer />
    <slot />
</div>


<script>
    import '../app.css';
    import Navbar from "../components/Navbar.svelte";
    import NotificationContainer from '../components/NotificationContainer.svelte';
</script>
