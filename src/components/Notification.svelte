<script>
  import { slide } from "svelte/transition";

    /**
    * @type {{id: string, contents: string, status: 'running' | 'error' | 'success'}}
    */
    export let notif
</script>

<div transition:slide class="px-2 py-1 bg-[var(--bg-primary)] border rounded-lg duration-300
    {notif.status==='running' ? "border-[var(--bg-tertiary)]" : 
        notif.status==='success' ? "border-green-700" : "border-red-700" }"
>
    { @html notif.contents }
</div>