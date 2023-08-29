<aside 
    class="z-20 bg-[var(--bg-secondary)] left-0 top-0 px-1 py-2 items-center flex flex-col fixed h-full text-white ease-in-out duration-200
        border-[var(--bg-tertiary)] border-r {expanded ? "w-36" : "w-14"}"
    >

    <div class="w-full h-auto" on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}>
        <NavButton text="Home" href="/home" show={expanded}>
            <IconHome />
        </NavButton>

        <NavButton text="Create" href="/create" show={expanded}>
            <IconCreate />
        </NavButton>

        <NavButton text="Settings" href="/settings" show={expanded}>
            <IconSettings />
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
