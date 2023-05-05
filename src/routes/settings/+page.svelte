<h1 class="text-xl">Settings</h1>

<div on:click={setInstancePath}  class="m-4 cursor-pointer bg-gray-800 border-blue-400 border-2">
    <p>Path of the Instance Folder:</p>
    <p class="bg-gray-500 m-2">{instancePath}</p>
</div>



<script>
    //TODO: Add a way for the user to select the instance folder
    import { pickDir } from './folder_selector'

    let instancePath = "Click to set!"

    function setInstancePath() {
        pickDir().then(p => {if(p!=null) instancePath=p})
    }
</script>
