import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

let settingsWindow: WebviewWindow | null = null;

export async function openSettingsWindow() {
    if (settingsWindow) {
        try {
            await settingsWindow.show();
            await settingsWindow.setFocus();
            return;
        } catch (e) {
            settingsWindow = null
        }
    }
    settingsWindow = new WebviewWindow('settings', {
        url: 'settings.html',
        title: 'Settings',
        width: 600,
        height: 500,
        minWidth: 400,
        minHeight: 300,
        decorations: true,
        resizable: false,
        center: true,
    });

    settingsWindow.once('tauri://created', () => {
        console.log('Settings window created');
    });

    settingsWindow.once('tauri://error', (error) => {
        console.error('Error creating settings window', {
            message: error?.event,
            stack: error?.id,
            payload: error.payload
        });
    });

    settingsWindow.once('close', () => {
        console.log("Settings window closed");
        settingsWindow = null;
    });
}
