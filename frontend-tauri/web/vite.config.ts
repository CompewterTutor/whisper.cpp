import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	// Tauri expects a fixed port for development
	server: {
		port: 1420,
		strictPort: true,
		// Tauri uses custom protocol, allow all origins
		cors: true
	},

	// Build configuration for Tauri
	build: {
		// Don't inline small assets - Tauri needs them as files
		assetsInlineLimit: 0,
		// Produce sourcemaps for debugging
		sourcemap: true,
		// Target modern browsers for smaller bundle
		target: 'esnext'
	},

	// Clear console on rebuild for cleaner output
	clearScreen: false
});
