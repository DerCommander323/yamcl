<aside class="sidebar bg-neutral-950 h-full text-">
why no color
</aside>

<div class="content">
    <main>
        <slot />
    </main>
</div>


<style>
    .sidebar {
        color: #EEE;
        position: fixed;
        width: 70px;
    }
</style>

<script>
    let sidebar = document.querySelector('#sidebar')
    console.log(sidebar) //logs null
    import '../app.css'
</script>
