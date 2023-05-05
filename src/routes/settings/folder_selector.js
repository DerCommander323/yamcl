import { open } from '@tauri-apps/api/dialog';
// Open a selection dialog for directories

 export async function pickDir() {
    const selected = await open({
        directory: true,
        multiple: true,
    });
    if (Array.isArray(selected)) {
        // user selected multiple directories
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        // user selected a single directory
    }
}


