import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// Static adapter for Tauri desktop app - no SSR needed
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			precompress: false,
			strict: true
		}),
		// Tauri uses custom protocol, trust these origins for CSRF
		csrf: {
			trustedOrigins: ['tauri://localhost', 'http://localhost:1420']
		}
	}
};

export default config;
