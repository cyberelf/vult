import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { readFileSync } from 'fs';
import { join } from 'path';

// Read version from tauri.conf.json
const tauriConfigPath = join(__dirname, '../tauri.conf.json');
const tauriConfig = JSON.parse(readFileSync(tauriConfigPath, 'utf-8'));
const APP_VERSION = tauriConfig.version;

export default defineConfig({
	plugins: [sveltekit()],
	define: {
		'__APP_VERSION__': JSON.stringify(APP_VERSION)
	}
});
