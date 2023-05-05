// @ts-ignore
import { open } from '@tauri-apps/api/dialog';
// Open a selection dialog for directories

 export async function pickDir() {
    const selected = await open({
        directory: true,
        multiple: false,
    })
    return selected
}


