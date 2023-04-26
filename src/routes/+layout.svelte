<aside class="sidebar">
why no color
</aside>

<div class="content">
    <main>
        <slot />
    </main>
</div>


<style>
    .sidebar {
        background-color: #1E1E20;
        color: #EEE;
        position: fixed;
        height: 100%;
        width: 70px;
    }
</style>

<script>
    let sidebar = document.querySelector('#sidebar')
    console.log(sidebar) //logs null
</script>
