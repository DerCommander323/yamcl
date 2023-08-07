<aside 
    style="background-color:var(--bg-secondary);" 
    class="z-20 left-0 top-0 py-4 items-center flex flex-col fixed h-screen text-white ease-in-out duration-150 p-1 {expanded ? "w-36" : "w-14"}"
    >

    <div class="w-full h-auto" on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}>
        <NavButton text="Home" href="/home" show={expanded}>
            <IconHome />
        </NavButton>

        <NavButton text="Settings" href="/settings" show={expanded}>
            <IconSettings />
        </NavButton>

        <NavButton text="Create" href="/create" show={expanded}>
            <IconCreate />
        </NavButton>
    </div>
     

</aside>

<script>
    import '../app.css'
    import NavButton from './NavButton.svelte'

    import IconHome from 'svelte-icons/ti/TiHome.svelte'
    import IconSettings from 'svelte-icons/md/MdSettings.svelte'
    import IconCreate from 'svelte-icons/md/MdAdd.svelte'

    let expanded = false

    function handleButtonHover() {
        expanded = true

    }
    function handleButtonOut() {
        expanded = false
    }

</script>
