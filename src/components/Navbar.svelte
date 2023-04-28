<aside 
    style="background-color:var(--bg-secondary);" 
    class="left-0 top-0 py-4 items-center flex flex-col fixed h-screen text-white ease-in-out duration-150 p-1 {expanded ? "w-36" : "w-14"}"
    >

    <div class="w-full h-auto" on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}>
        <NavButton text="Home" show={expanded}>
            <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" class="bi bi-house" viewBox="0 0 16 16"> <path fill-rule="evenodd" d="M2 13.5V7h1v6.5a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5V7h1v6.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5zm11-11V6l-2-2V2.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5z"/> <path fill-rule="evenodd" d="M7.293 1.5a1 1 0 0 1 1.414 0l6.647 6.646a.5.5 0 0 1-.708.708L8 2.207 1.354 8.854a.5.5 0 1 1-.708-.708L7.293 1.5z"/> </svg>
            
        </NavButton>
        <NavButton text="Settings" show={expanded}>
       <svg height="30px" width="30px" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
	 viewBox="0 0 488.189 488.189" xml:space="preserve">
        <g>
	    <g>
		<path d="M326.9,67.166c6.7-14.5,13.5-28.9,20.6-44.2c-78.6-33.1-155.2-31.5-228.4,12.1C48.3,77.166,9.9,141.166,0,223.566
			c16.8,1.5,32.7,2.9,48.6,4.2C57.3,103.266,194.9,7.766,326.9,67.166z"/>
		<path d="M484.7,202.466c-15.8,2.7-31.5,5.5-47.9,8.3c8.5,57-3.3,108.4-38.1,153.4c-34.6,44.8-81,69.4-137.7,75.7
			c1.4,16.1,2.8,31.8,4.1,47.5C383.8,483.166,511,363.966,484.7,202.466z"/>
		<path d="M47.9,262.066c-15.1,1.3-30.8,2.7-46.5,4c3.7,99.5,90.5,211.5,221.4,222.1c1.4-16.1,2.8-31.9,4.1-47.2
			C124.5,423.866,65.2,364.566,47.9,262.066z"/>
		<path d="M398.9,125.566c12.2,15.8,21.6,33.2,28.9,52.4c15.2-5.5,30.2-11,45.9-16.7c-18.1-48.2-47.6-86.6-89.5-116.7
			c-9.3,13.3-18.5,26.3-28,39.9C372.4,96.366,386.8,109.866,398.9,125.566z"/>
		<path d="M267.2,384.166v-19.1c16.3-3.1,32.1-9.6,46.3-19.4l13.6,13.6c5.1,5,13.2,5,18.3,0l14.1-14.1c5-5.1,5-13.2,0-18.3
			l-13.8-13.8c9.6-14.2,15.9-30,18.9-46.2h19.6c7.1,0,12.9-5.8,12.9-12.9v-20c0-7.1-5.8-12.9-12.9-12.9h-19.7
			c-3.1-16.2-9.5-31.8-19.1-45.9l14-14c5-5.1,5-13.2,0-18.3l-14.3-14.2c-5.1-5-13.2-5-18.3,0l-14,14c-14.1-9.6-29.8-16-46-19.1
			v-19.7c0-7.1-5.8-12.9-12.9-12.9h-20c-7.1,0-12.9,5.8-12.9,12.9v19.6c-16.2,3-32,9.4-46.2,19l-13.7-13.6c-5.1-5-13.2-5-18.3,0
			l-14.1,14.2c-5,5.1-5,13.2,0,18.3l13.6,13.6c-9.8,14.2-16.2,30-19.3,46.3h-19.1c-7.1,0-12.9,5.8-12.9,12.9v20
			c0,7.1,5.8,12.9,12.9,12.9h19c3.1,16.4,9.5,32.3,19.2,46.5l-13.2,13.5c-5,5.1-5,13.2,0,18.3l14.1,14.1c5.1,5,13.2,5,18.3,0
			l13.4-13.4c14.3,9.7,30.2,16.1,46.6,19.2v19c0,7.1,5.8,12.9,12.9,12.9h20C261.4,397.066,267.2,391.266,267.2,384.166z
			 M197,291.266c-26-26-26.1-68.2-0.1-94.2s68.2-26.1,94.2-0.1s26.1,68.2,0.1,94.2C265.3,317.166,223.1,317.266,197,291.266z"/>
	</g>
</g>
</svg> 
        </NavButton>

<NavButton text="Create" show={expanded}>
        <svg version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
	 width="30px" height="30px" viewBox="0 0 349.03 349.031"
	 xml:space="preserve">
<g>
	<path d="M349.03,141.226v66.579c0,5.012-4.061,9.079-9.079,9.079H216.884v123.067c0,5.019-4.067,9.079-9.079,9.079h-66.579
		c-5.009,0-9.079-4.061-9.079-9.079V216.884H9.079c-5.016,0-9.079-4.067-9.079-9.079v-66.579c0-5.013,4.063-9.079,9.079-9.079
		h123.068V9.079c0-5.018,4.069-9.079,9.079-9.079h66.579c5.012,0,9.079,4.061,9.079,9.079v123.068h123.067
		C344.97,132.147,349.03,136.213,349.03,141.226z"/>
</g>
</svg>
    </NavButton>
    </div>
     

</aside>

<script>
    import '../app.css'
    import NavButton from './NavButton.svelte'

    let expanded = false

    function handleButtonHover() {
        expanded = true

    }
    function handleButtonOut() {
        expanded = false
    }

    //on:mouseover={handleButtonHover} on:mouseout={handleButtonOut} on:focus={handleButtonHover} on:blur={handleButtonOut}
</script>
