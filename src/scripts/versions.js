


/**
 * @type {{
 *      latest: {
 *          release: string,
 *          snapshot: string
 *      },
 *      versions: {
 *          "id": string,
 *          "type": "snapshot" | "release" | "old_beta" | "old_alpha",
 *          "url": string,
 *          "time": string,
 *          "releaseTime": string,
 *          "sha1": string,
 *          "complianceLevel": number
 *      }[]
 * }}
 */
export let minecraftVersionList



export async function getMinecraftVersions() {
    if(!minecraftVersionList) {
        minecraftVersionList = (await (await fetch('https://piston-meta.mojang.com/mc/game/version_manifest_v2.json')).json())
    }
    return minecraftVersionList
}