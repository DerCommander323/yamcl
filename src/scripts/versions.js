


/**
 * @type {{ 
 * formatVersion: number, name: string, uid: string, versions: {
 *     recommended: boolean, releasetime: string, requires: {}[], type: string, version: string, sha256: string }[]
 * }}
 */
export let minecraftVersionList



export async function getMinecraftVersions() {
    if(!minecraftVersionList) {
        minecraftVersionList = (await (await fetch('https://meta.prismlauncher.org/v1/net.minecraft/')).json())
    }
    return minecraftVersionList
}