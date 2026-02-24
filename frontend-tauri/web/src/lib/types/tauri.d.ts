// Type declarations for Tauri API
declare global {
	interface Window {
		__TAURI__?: {
			core: {
				invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
			};
		};
	}
}

export {};
