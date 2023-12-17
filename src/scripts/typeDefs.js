/**
 * InstanceType Enum
 * @typedef { "MultiMC" | "CurseForge" } MCInstanceType
*/

/**
 * ModLoader Struct
 * @typedef {{
 *  name: String,
 *  typ: ModLoaders,
 *  version: String
 * }} ModLoader
*/

/**
 * ModLoaders Enum
 * @typedef { "Vanilla" | "Forge" | "NeoForge" | "Fabric" | "Quilt" | "LiteLoader" | "Rift" } ModLoaders
*/

/**
 * SimpleInstance Struct
 * @typedef {{
 *  name: String,
 *  icon: String,
 *  path: String,
 *  id: Number,
 *  mc_version: String,
 *  modloader: ModLoader,
 *  last_played: String | null,
 *  instance_type: MCInstanceType
 * }} SimpleInstance
 */


/**
 * JavaDetails Struct
 * @typedef {{ 
 *  path: String,
 *  label: String,
 *  version: String,
 *  mcVersions: {
 *      min: MCVersion,
 *      max: MCVersion
 *  },
 *  xmx: Number,
 *  xms: Number,
 *  args: String,
 *  extended: boolean,
 *  mcExtended: boolean 
 * }} JavaDetails
 */

/**
 * MCVersionList Struct
 * @typedef {{
 *  latest: {
 *      release: String,
 *      snapshot: String
 *  },
 *  versions: MCVersion[]
 * }} MCVersionList
 */

/**
 * MCVersionDetails Struct
 * @typedef {{
 *      "id": String,
 *      "type": "snapshot" | "release" | "old_beta" | "old_alpha",
 *      "url": String,
 *      "time": String,
 *      "releaseTime": String,
 *      "sha1": String,
 *      "complianceLevel": Number
 * }} MCVersion
 */