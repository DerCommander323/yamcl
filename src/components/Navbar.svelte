<aside 
    style="background-color:var(--bg-secondary);" 
    class="left-0 top-0 py-4 items-center flex flex-col fixed h-screen text-white ease-in-out duration-150 p-1 {expanded ? "w-36" : "w-14"}"
    >

    <div class="w-full h-auto" on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}>
        <NavButton text="Home" show={expanded}>
            <FaHome />
        </NavButton>

        <NavButton text="Settings" show={expanded}>
            <FaCog />
        </NavButton>

        <NavButton text="Create" show={expanded}>
            <IoIosAddCircle />
        </NavButton>
    </div>
     

</aside>

<script>
    import '../app.css'
    import NavButton from './NavButton.svelte'
    import IoIosAddCircle from 'svelte-icons/io/IoIosAddCircle.svelte'
    import FaCog from 'svelte-icons/fa/FaCog.svelte'
    import FaHome from 'svelte-icons/fa/FaHome.svelte'

    let expanded = false

    function handleButtonHover() {
        expanded = true

    }
    function handleButtonOut() {
        expanded = false
    }

    //on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}
</script>
