
<Topbar text="Accounts"></Topbar>

<div class="inline-flex h-full">
    <div class="w-1/2 h-full">
        <p class="text-lg m-1 underline"> Stored accounts: </p>
        <div class="bg-[var(--bg-secondary)] border border-[var(--bg-tertiary)] m-1 rounded-md">
            {#if $accountStore.length == 0}
                <p class="m-1 italic"> Empty </p>
            {:else}
                {#each $accountStore as account, index}
                    <div on:click={() => setPI(index)} on:keydown={() => setPI(index)} 
                        class="p-1 rounded-md hover:bg-[var(--bg-tertiary)] inline-flex w-full cursor-pointer
                            { $previewIndex === index ? "bg-[var(--bg-tertiary)]" : ""}"
                        >
                        <button class="h-5 w-5 duration-150 hover:text-green-500 " on:click={() => setSelectedIndex(index)}> <SelectIcon /> </button>
                        <button class="h-5 w-5 mx-1 duration-150 hover:text-red-600" on:click={() => removeAccount(index, account.name)}> <RemoveIcon /> </button>
                        <p class="{ $selectedIndex === index ? "underline font-medium" : "" }"> { account.name } </p>
                    </div>
                {/each}
            {/if}
        </div>
        <div class="p-1">
            <button on:click={addAccount} class="bg-[var(--bg-secondary)] border border-[var(--bg-tertiary)] rounded-md w-full font-medium text-lg hover:underline" > Add account </button>
        </div>
    </div>
    <div class="w-1/2 h-full border-4 border-[var(--bg-tertiary)] rounded-xl">
        {#if $accountState === 'loading'}
            <p> Loading accounts... </p>
        {:else if $accountState === 'errored'}
            <p> Loading accounts failed! </p>
        {:else if $accountState === 'success'}
            {#if $accountStore.length > 0}
                <AccountDisplay> </AccountDisplay>
            {:else}
                <p> No accounts found! </p>
                <p> Use the 'Add account' button to add one! </p>
            {/if}
        {/if}
    </div>
</div>


<script>
    import { onMount } from "svelte"
    import { accountState, accountStore, previewIndex, addAccount, loadAccounts, removeAccount, selectedIndex, setPreviewIndex, setSelectedIndex } from "../../scripts/accounts"

    import SelectIcon from 'svelte-icons/md/MdCheckCircle.svelte'
    import RemoveIcon from 'svelte-icons/md/MdRemoveCircleOutline.svelte'
    import Topbar from "../../components/Topbar.svelte"
    import AccountDisplay from "../../components/AccountDisplay.svelte"

    onMount(async () => {
        if($accountState === 'loading') loadAccounts()
    })

    /**
   * @param {number} index
   */
    async function setPI(index) {
        setPreviewIndex(index)
    }
    
</script>