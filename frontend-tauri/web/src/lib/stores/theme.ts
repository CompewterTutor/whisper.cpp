import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'system' | 'light' | 'dark';

const STORAGE_KEY = 'careless.theme';

function getSystemTheme(): 'light' | 'dark' {
	if (!browser) return 'dark';
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function getStoredTheme(): Theme {
	if (!browser) return 'system';
	const stored = localStorage.getItem(STORAGE_KEY);
	if (stored === 'light' || stored === 'dark' || stored === 'system') {
		return stored;
	}
	return 'system';
}

function applyTheme(theme: Theme): void {
	if (!browser) return;

	const effectiveTheme = theme === 'system' ? getSystemTheme() : theme;
	document.documentElement.setAttribute('data-theme', effectiveTheme);
}

function createThemeStore() {
	const { subscribe, set, update } = writable<Theme>(getStoredTheme());

	// Apply initial theme on client
	if (browser) {
		applyTheme(getStoredTheme());

		// Listen for system theme changes
		window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
			const current = getStoredTheme();
			if (current === 'system') {
				applyTheme('system');
			}
		});
	}

	return {
		subscribe,
		set: (theme: Theme) => {
			if (browser) {
				localStorage.setItem(STORAGE_KEY, theme);
				applyTheme(theme);
			}
			set(theme);
		},
		toggle: () => {
			update((current) => {
				const next: Theme = current === 'dark' ? 'light' : current === 'light' ? 'system' : 'dark';
				if (browser) {
					localStorage.setItem(STORAGE_KEY, next);
					applyTheme(next);
				}
				return next;
			});
		}
	};
}

export const theme = createThemeStore();
