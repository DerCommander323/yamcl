<div>
    <p class="text-xl font-semibold w-min mx-auto p-1 underline"> { $accountStore[$previewIndex].name } </p>
    {#if $selectedIndex === $previewIndex}
        <p class="mx-auto w-fit italic mb-1"> Currently active account </p>
    {/if}
    <div class="relative">
        <canvas id="skinCanvas" class="w-full h-full cursor-all-scroll"></canvas>
        <div class="absolute top-0 left-0 z-10">
            <div id="skinSelector" class="inline-flex">
                <button class="h-6 w-6" on:click={() => setSkinIndex($skinIndex-1, $accountStore[$previewIndex].skins.length)}>
                    <ArrowLeft />
                </button>
                <p> Skin: { $skinIndex+1 }/{ $accountStore[$previewIndex].skins.length } </p>
                <button class="h-6 w-6" on:click={() => setSkinIndex($skinIndex+1, $accountStore[$previewIndex].skins.length)}>
                    <ArrowRight />
                </button>
            </div>
        </div>
    </div>
    <p class="mx-auto w-fit m-2">
        <abbr title="Universally Unique IDentifier" class="decoration-dotted"> UUID: </abbr>
        { $accountStore[$previewIndex].id }
    </p>
</div>

<script>
    import { onDestroy, onMount } from "svelte"
    import { accountStore, previewIndex, selectedIndex, setSkinIndex, skinIndex } from "../scripts/accounts"
    import { IdleAnimation, SkinViewer } from "skinview3d"

    import ArrowLeft from 'svelte-icons/md/MdKeyboardArrowLeft.svelte'
    import ArrowRight from 'svelte-icons/md/MdKeyboardArrowRight.svelte'

    

    const unsub = previewIndex.subscribe((val) => {
        updateSkinViewer(val)
    })

    onMount(() => {
        updateSkinViewer($previewIndex)
    })

    onDestroy(unsub)

    $: updateSkinViewer($skinIndex)

    /**
   * @param {number} index
   */
    function updateSkinViewer(index) {
        let canvas = document.getElementById('skinCanvas')
        if(!canvas) return
        let skinViewer = new SkinViewer({
            // @ts-ignore
            canvas,
            skin: $accountStore[index].skins[0].url,
            background: '#444',
            height: canvas?.clientHeight?? 300,
            width: canvas?.clientWidth?? 300,
        })
        skinViewer.animation = new IdleAnimation()
        // this breaks the mouse movement :( skinViewer.playerObject.rotateY(-0.6).rotateX(0.5).rotateZ(-0.3)
        
    }
</script>