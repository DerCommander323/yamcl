<script>
    import { slide } from "svelte/transition"
    import LoadingSpinner from "./LoadingSpinner.svelte"
    import SuccessIcon from 'svelte-icons/md/MdCheckCircle.svelte'
    import FailIcon from 'svelte-icons/md/MdError.svelte'

    /**
    * @type {{id: string, contents: string, status: 'running' | 'error' | 'success'}}
    */
    export let notif
</script>

<div transition:slide|local class="px-2 py-1 my-1 bg-[var(--bg-primary)] border rounded-lg duration-1000 text-lg flex max-w-[40vw]
    {notif.status==='running' ? "border-[var(--bg-tertiary)]" : 
        notif.status==='success' ? "border-green-700" : "border-red-700" }"
>

    {#if notif.status === 'running'}
        <p class="w-7 min-w-7 shrink-0"> <LoadingSpinner /> </p>
    {:else if notif.status === 'success'}
        <p class="w-6 min-w-6 m-0.5 text-green-500 shrink-0"> <SuccessIcon /> </p>
    {:else}
        <p class="w-6 min-w-6 m-0.5 text-red-600 shrink-0"> <FailIcon /> </p>
    {/if}

    { @html notif.contents }
</div>