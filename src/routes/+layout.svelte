<aside id="sidebar">
A
</aside>

<div id="content">
    <slot />
</div>


<style>
    #sidebar {
        background-color: gray;
        position: fixed;
        height: 100%;
        width: auto;
    }
</style>

<script>
    let sidebar = document.querySelector('#sidebar')
    console.log(sidebar) //logs null
</script>
