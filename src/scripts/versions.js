/**
 * @type {MCVersionList}
 */
export let minecraftVersionList


export async function getMinecraftVersions() {
    if(!minecraftVersionList) {
        minecraftVersionList = (await (await fetch('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json')).json())
    }
    return minecraftVersionList
}