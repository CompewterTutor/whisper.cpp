<script lang="ts">
	import { onMount } from 'svelte';
	import { theme, type Theme } from '$lib/stores/theme';
	import {
		getAppSettings,
		setTheme,
		setDefaultOutputDir,
		setDefaultModelDir,
		setDefaultThreads,
		setDefaultTimeout,
		setDiagnosticsEnabled,
		setStartInBackground,
		setLaunchOnLogin,
		pickDirectory,
		getPttRouting,
		setPttRouting,
		getShortcuts,
		setShortcuts,
		registerGlobalShortcut,
		unregisterGlobalShortcut,
		listAudioDevices,
		selectAudioDevice,
		type AppSettings,
		type PttRouting,
		type ShortcutSettings,
		type AudioDevice
	} from '$lib/services/tauri';

	interface Props {
		onstatus?: (message: string, type: 'ok' | 'error' | 'neutral') => void;
	}

	let { onstatus }: Props = $props();

	// Settings state
	let settings = $state<AppSettings>({
		theme: 'system',
		default_output_dir: '',
		default_model_dir: '',
		default_threads: null,
		default_timeout_ms: null,
		diagnostics_enabled: false,
		start_in_background: false,
		launch_on_login: false
	});

	// PTT routing state
	let pttRouting = $state<PttRouting>({
		copy_to_clipboard: true,
		save_to_file: false,
		type_emulation: false
	});

	// Shortcuts state
	let shortcuts = $state<ShortcutSettings>({
		enabled: false,
		ptt: 'Ctrl+Shift+Space',
		type: 'Ctrl+Shift+T',
		clipboard: 'Ctrl+Shift+C',
		file: 'Ctrl+Shift+F'
	});

	// Audio devices
	let audioDevices = $state<AudioDevice[]>([]);
	let selectedDevice = $state<string>('');
	let pttMode = $state<'hold' | 'toggle'>('hold');

	// Active registered shortcuts (for tracking what's registered)
	let activeRegisteredShortcuts = $state<Set<string>>(new Set());

	// Shortcut conflicts
	let shortcutConflicts = $state<Map<string, { value: string; others: string[] }>>(new Map());

	onMount(async () => {
		await loadSettings();
		await loadPttRoutingSettings();
		await loadShortcutSettings();
		await loadAudioDevices();
	});

	async function loadSettings() {
		try {
			settings = await getAppSettings();
			// Sync theme store with backend
			if (settings.theme) {
				theme.set(settings.theme as Theme);
			}
		} catch (error) {
			console.error('Failed to load settings:', error);
			onstatus?.('Failed to load settings', 'error');
		}
	}

	async function loadPttRoutingSettings() {
		try {
			pttRouting = await getPttRouting();
		} catch (error) {
			console.error('Failed to load PTT routing:', error);
			// Use defaults
			pttRouting = {
				copy_to_clipboard: true,
				save_to_file: false,
				type_emulation: false
			};
		}
	}

	async function loadShortcutSettings() {
		try {
			const stored = localStorage.getItem('frontend-tauri.shortcuts');
			if (stored) {
				shortcuts = JSON.parse(stored);
			}
		} catch (error) {
			console.error('Failed to load shortcuts:', error);
		}
	}

	async function loadAudioDevices() {
		try {
			audioDevices = await listAudioDevices();
			const defaultDevice = audioDevices.find(d => d.is_default);
			if (defaultDevice) {
				selectedDevice = defaultDevice.name;
			}
		} catch (error) {
			console.error('Failed to load audio devices:', error);
		}
	}

	// Theme change handler
	async function handleThemeChange(newTheme: Theme) {
		try {
			await setTheme(newTheme);
			settings.theme = newTheme;
			theme.set(newTheme);
			onstatus?.(`Theme set to ${newTheme}`, 'ok');
		} catch (error) {
			onstatus?.('Failed to save theme setting', 'error');
		}
	}

	// Directory picker handlers
	async function handlePickOutputDir() {
		try {
			const path = await pickDirectory();
			await setDefaultOutputDir(path);
			settings.default_output_dir = path;
			onstatus?.('Default output directory saved', 'ok');
		} catch (error: unknown) {
			const err = error as { code?: string };
			if (err.code !== 'selection_cancelled') {
				onstatus?.('Failed to set default output directory', 'error');
			}
		}
	}

	async function handlePickModelDir() {
		try {
			const path = await pickDirectory();
			await setDefaultModelDir(path);
			settings.default_model_dir = path;
			onstatus?.('Default model directory saved', 'ok');
		} catch (error: unknown) {
			const err = error as { code?: string };
			if (err.code !== 'selection_cancelled') {
				onstatus?.('Failed to set default model directory', 'error');
			}
		}
	}

	// Execution defaults handlers
	async function handleThreadsChange(value: string) {
		const threads = value ? parseInt(value, 10) : null;
		try {
			await setDefaultThreads(threads);
			settings.default_threads = threads;
			onstatus?.('Default threads saved', 'ok');
		} catch {
			onstatus?.('Failed to save default threads', 'error');
		}
	}

	async function handleTimeoutChange(value: string) {
		const timeout = value ? parseInt(value, 10) : null;
		try {
			await setDefaultTimeout(timeout);
			settings.default_timeout_ms = timeout;
			onstatus?.('Default timeout saved', 'ok');
		} catch {
			onstatus?.('Failed to save default timeout', 'error');
		}
	}

	// Startup behavior handlers
	async function handleStartInBackgroundChange(enabled: boolean) {
		try {
			await setStartInBackground(enabled);
			settings.start_in_background = enabled;
			onstatus?.(
				enabled
					? 'Start-in-background enabled for next launch'
					: 'Start-in-background disabled',
				'ok'
			);
		} catch {
			onstatus?.('Failed to save start-in-background setting', 'error');
			settings.start_in_background = !enabled;
		}
	}

	async function handleLaunchOnLoginChange(enabled: boolean) {
		try {
			await setLaunchOnLogin(enabled);
			settings.launch_on_login = enabled;
			onstatus?.(
				enabled ? 'Launch on login enabled' : 'Launch on login disabled',
				'ok'
			);
		} catch {
			onstatus?.('Failed to save launch on login setting', 'error');
			settings.launch_on_login = !enabled;
		}
	}

	// Diagnostics handler
	async function handleDiagnosticsChange(enabled: boolean) {
		try {
			await setDiagnosticsEnabled(enabled);
			settings.diagnostics_enabled = enabled;
			onstatus?.(
				enabled ? 'Diagnostic logging enabled' : 'Diagnostic logging disabled',
				'ok'
			);
		} catch {
			onstatus?.('Failed to save diagnostics setting', 'error');
			settings.diagnostics_enabled = !enabled;
		}
	}

	// Shortcuts handlers
	function detectShortcutConflicts(): Map<string, { value: string; others: string[] }> {
		const shortcutList = [
			{ key: 'ptt', value: shortcuts.ptt, name: 'Push-to-talk' },
			{ key: 'type', value: shortcuts.type, name: 'Type emulation' },
			{ key: 'clipboard', value: shortcuts.clipboard, name: 'Capture-to-clipboard' },
			{ key: 'file', value: shortcuts.file, name: 'Capture-to-file' }
		];

		const valueMap = new Map<string, { key: string; name: string }[]>();
		const conflicts = new Map<string, { value: string; others: string[] }>();

		for (const { key, value, name } of shortcutList) {
			if (!value.trim()) continue;
			if (!valueMap.has(value)) {
				valueMap.set(value, []);
			}
			valueMap.get(value)!.push({ key, name });
		}

		for (const [value, items] of valueMap) {
			if (items.length > 1) {
				for (const item of items) {
					conflicts.set(item.key, {
						value,
						others: items.filter(i => i.key !== item.key).map(i => i.name)
					});
				}
			}
		}

		return conflicts;
	}

	function persistShortcuts() {
		localStorage.setItem('frontend-tauri.shortcuts', JSON.stringify(shortcuts));
		localStorage.setItem('frontend-tauri.shortcutsEnabled', shortcuts.enabled ? 'true' : 'false');
	}

	async function applyShortcutRegistration(enabled: boolean) {
		if (!enabled) {
			for (const shortcut of Array.from(activeRegisteredShortcuts)) {
				try {
					await unregisterGlobalShortcut(shortcut);
				} catch {
					// Ignore unregister errors
				}
			}
			activeRegisteredShortcuts = new Set();
			return;
		}

		const bindings = [shortcuts.ptt, shortcuts.type, shortcuts.clipboard, shortcuts.file]
			.filter(s => s.trim());
		const desired = new Set(bindings);

		// Unregister shortcuts no longer needed
		for (const shortcut of Array.from(activeRegisteredShortcuts)) {
			if (!desired.has(shortcut)) {
				try {
					await unregisterGlobalShortcut(shortcut);
					activeRegisteredShortcuts.delete(shortcut);
				} catch {
					// Ignore
				}
			}
		}

		// Register new shortcuts
		for (const shortcut of bindings) {
			if (!activeRegisteredShortcuts.has(shortcut)) {
				await registerGlobalShortcut(shortcut);
				activeRegisteredShortcuts.add(shortcut);
			}
		}
	}

	async function handleShortcutsEnabledChange(enabled: boolean) {
		persistShortcuts();
		shortcutConflicts = detectShortcutConflicts();

		if (shortcutConflicts.size > 0) {
			onstatus?.('Resolve shortcut conflicts before enabling', 'error');
			return;
		}

		try {
			await applyShortcutRegistration(enabled);
			shortcuts.enabled = enabled;
			onstatus?.('Global shortcut preference updated', 'ok');
		} catch {
			shortcuts.enabled = false;
			persistShortcuts();
			onstatus?.('Global shortcut setup failed', 'error');
		}
	}

	function handleShortcutInput() {
		shortcutConflicts = detectShortcutConflicts();
	}

	async function handleShortcutChange() {
		persistShortcuts();
		shortcutConflicts = detectShortcutConflicts();

		if (shortcutConflicts.size > 0) {
			onstatus?.('Resolve shortcut conflicts before enabling', 'error');
			return;
		}

		if (!shortcuts.enabled) {
			onstatus?.('Shortcut binding saved', 'ok');
			return;
		}

		try {
			await applyShortcutRegistration(true);
			onstatus?.('Shortcut bindings updated and re-registered', 'ok');
		} catch {
			onstatus?.('Shortcut binding update failed', 'error');
		}
	}

	// PTT routing handlers
	async function handlePttRoutingChange() {
		try {
			await setPttRouting(pttRouting);
			onstatus?.('PTT routing settings saved', 'ok');
		} catch {
			onstatus?.('Failed to save PTT routing', 'error');
		}
	}

	// Audio device handler
	async function handleAudioDeviceChange(deviceName: string) {
		try {
			await selectAudioDevice(deviceName);
			selectedDevice = deviceName;
			onstatus?.(`Microphone set to: ${deviceName}`, 'ok');
		} catch {
			onstatus?.('Failed to select microphone', 'error');
		}
	}

	// Helper for checking conflicts
	function hasConflict(key: string): boolean {
		return shortcutConflicts.has(key);
	}
</script>

<div class="section">
	<div class="section-header">Settings</div>

	<!-- Appearance Settings -->
	<div class="card">
		<div class="subsection-header">Appearance</div>
		<div class="settings-grid">
			<div class="field">
				<label for="themeSelect">Theme</label>
				<select
					id="themeSelect"
					value={settings.theme}
					onchange={(e) => handleThemeChange((e.target as HTMLSelectElement).value as Theme)}
				>
					<option value="system">System</option>
					<option value="light">Light</option>
					<option value="dark">Dark</option>
				</select>
			</div>
		</div>
	</div>

	<!-- Default Paths -->
	<div class="card">
		<div class="subsection-header">Default Paths</div>
		<div class="settings-grid">
			<div class="field">
				<label for="defaultOutputDir">Default output directory</label>
				<div class="row">
					<input
						id="defaultOutputDir"
						type="text"
						placeholder="(use file picker each time)"
						value={settings.default_output_dir}
						readonly
					/>
					<button type="button" class="secondary" onclick={handlePickOutputDir}>
						Browse...
					</button>
				</div>
			</div>
			<div class="field">
				<label for="defaultModelDir">Default model directory</label>
				<div class="row">
					<input
						id="defaultModelDir"
						type="text"
						placeholder="(use file picker each time)"
						value={settings.default_model_dir}
						readonly
					/>
					<button type="button" class="secondary" onclick={handlePickModelDir}>
						Browse...
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Execution Defaults -->
	<div class="card">
		<div class="subsection-header">Execution Defaults</div>
		<div class="settings-grid">
			<div class="field">
				<label for="defaultThreads">Default threads <span class="kbd">0 = auto</span></label>
				<input
					id="defaultThreads"
					type="number"
					min="0"
					max="128"
					placeholder="Auto"
					value={settings.default_threads ?? ''}
					onchange={(e) => handleThreadsChange((e.target as HTMLInputElement).value)}
				/>
			</div>
			<div class="field">
				<label for="defaultTimeout">Default timeout (ms)</label>
				<input
					id="defaultTimeout"
					type="number"
					min="1000"
					placeholder="60000"
					value={settings.default_timeout_ms ?? ''}
					onchange={(e) => handleTimeoutChange((e.target as HTMLInputElement).value)}
				/>
			</div>
		</div>
	</div>

	<!-- Startup Behavior -->
	<div class="card">
		<div class="subsection-header">Startup Behavior</div>
		<div class="settings">
			<label class="toggle">
				<input
					type="checkbox"
					checked={settings.start_in_background}
					onchange={(e) => handleStartInBackgroundChange((e.target as HTMLInputElement).checked)}
				/>
				<span>Start in background</span>
			</label>
			<label class="toggle">
				<input
					type="checkbox"
					checked={settings.launch_on_login}
					onchange={(e) => handleLaunchOnLoginChange((e.target as HTMLInputElement).checked)}
				/>
				<span>Launch on login</span>
			</label>
		</div>
		<p class="hint neutral">
			{settings.launch_on_login
				? 'App will launch automatically on login.'
				: 'App hides to tray on close. Enable start-in-background to hide on launch.'}
		</p>
	</div>

	<!-- Global Shortcuts -->
	<div class="card">
		<div class="subsection-header">Global Shortcuts</div>
		<div class="shortcuts-grid">
			<label class="toggle">
				<input
					type="checkbox"
					checked={shortcuts.enabled}
					onchange={(e) => handleShortcutsEnabledChange((e.target as HTMLInputElement).checked)}
				/>
				<span>Enable global shortcuts</span>
			</label>
			<div class="field">
				<label for="pttShortcut">Push-to-talk shortcut</label>
				<input
					id="pttShortcut"
					type="text"
					placeholder="Ctrl+Shift+Space"
					bind:value={shortcuts.ptt}
					oninput={handleShortcutInput}
					onchange={handleShortcutChange}
					class:conflict={hasConflict('ptt')}
				/>
			</div>
			<div class="field">
				<label for="typeShortcut">Type emulation shortcut</label>
				<input
					id="typeShortcut"
					type="text"
					placeholder="Ctrl+Shift+T"
					bind:value={shortcuts.type}
					oninput={handleShortcutInput}
					onchange={handleShortcutChange}
					class:conflict={hasConflict('type')}
				/>
			</div>
			<div class="field">
				<label for="clipboardShortcut">Capture-to-clipboard shortcut</label>
				<input
					id="clipboardShortcut"
					type="text"
					placeholder="Ctrl+Shift+C"
					bind:value={shortcuts.clipboard}
					oninput={handleShortcutInput}
					onchange={handleShortcutChange}
					class:conflict={hasConflict('clipboard')}
				/>
			</div>
			<div class="field">
				<label for="fileShortcut">Capture-to-file shortcut</label>
				<input
					id="fileShortcut"
					type="text"
					placeholder="Ctrl+Shift+F"
					bind:value={shortcuts.file}
					oninput={handleShortcutInput}
					onchange={handleShortcutChange}
					class:conflict={hasConflict('file')}
				/>
			</div>
		</div>
		<p class="hint" class:error={shortcutConflicts.size > 0} class:neutral={shortcutConflicts.size === 0}>
			{shortcutConflicts.size > 0
				? `Conflict detected: "${Array.from(shortcutConflicts.values())[0]?.value}" used by multiple shortcuts`
				: shortcuts.enabled
					? 'Global shortcuts enabled and registered.'
					: 'Shortcut bindings are saved.'}
		</p>
	</div>

	<!-- PTT Audio Settings -->
	<div class="card">
		<div class="subsection-header">PTT Audio Settings</div>
		<div class="settings-grid">
			<div class="field">
				<label for="audioDeviceSelect">Microphone</label>
				<select
					id="audioDeviceSelect"
					value={selectedDevice}
					onchange={(e) => handleAudioDeviceChange((e.target as HTMLSelectElement).value)}
				>
					{#if audioDevices.length === 0}
						<option value="">No microphones found</option>
					{:else}
						{#each audioDevices as device}
							<option value={device.name}>
								{device.is_default ? `${device.name} (Default)` : device.name}
							</option>
						{/each}
					{/if}
				</select>
			</div>
			<div class="field">
				<label for="pttModeSelect">PTT Mode</label>
				<select id="pttModeSelect" bind:value={pttMode}>
					<option value="hold">Hold to talk</option>
					<option value="toggle">Toggle (press to start/stop)</option>
				</select>
			</div>
		</div>
		<p class="hint neutral">
			Select a microphone for push-to-talk transcription. Configure the shortcut in Global Shortcuts above.
		</p>
	</div>

	<!-- PTT Output Routing -->
	<div class="card">
		<div class="subsection-header">PTT Output Routing</div>
		<div class="settings">
			<label class="toggle">
				<input
					type="checkbox"
					checked={pttRouting.copy_to_clipboard}
					onchange={(e) => {
						pttRouting.copy_to_clipboard = (e.target as HTMLInputElement).checked;
						handlePttRoutingChange();
					}}
				/>
				<span>Copy transcript to clipboard</span>
			</label>
			<label class="toggle">
				<input
					type="checkbox"
					checked={pttRouting.save_to_file}
					onchange={(e) => {
						pttRouting.save_to_file = (e.target as HTMLInputElement).checked;
						handlePttRoutingChange();
					}}
				/>
				<span>Save transcript to file</span>
			</label>
			<label class="toggle">
				<input
					type="checkbox"
					checked={pttRouting.type_emulation}
					onchange={(e) => {
						pttRouting.type_emulation = (e.target as HTMLInputElement).checked;
						handlePttRoutingChange();
					}}
				/>
				<span>Type emulation (paste transcript)</span>
			</label>
		</div>
		<p class="hint neutral">
			Configure where PTT transcripts are sent after transcription.
		</p>
	</div>

	<!-- Diagnostics -->
	<div class="card">
		<div class="subsection-header">Diagnostics</div>
		<div class="settings">
			<label class="toggle">
				<input
					type="checkbox"
					checked={settings.diagnostics_enabled}
					onchange={(e) => handleDiagnosticsChange((e.target as HTMLInputElement).checked)}
				/>
				<span>Enable diagnostic logging</span>
			</label>
		</div>
		<p class="hint neutral">
			When enabled, additional diagnostic information is logged for troubleshooting.
		</p>
	</div>
</div>

<style>
	.subsection-header {
		font-size: 0.8rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-muted);
		margin-bottom: var(--space-3);
	}

	.settings-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: var(--space-3);
	}

	@media (max-width: 600px) {
		.settings-grid {
			grid-template-columns: 1fr;
		}
	}

	.settings {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.shortcuts-grid {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.row {
		display: grid;
		grid-template-columns: 1fr auto;
		gap: var(--space-2);
		align-items: end;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.field label {
		font-size: var(--text-sm);
		color: var(--text-secondary);
	}

	.field input,
	.field select {
		font-size: var(--text-sm);
		padding: var(--space-1) var(--space-2);
	}

	.toggle {
		display: flex;
		gap: var(--space-2);
		align-items: center;
		cursor: pointer;
	}

	.toggle input[type='checkbox'] {
		width: auto;
		margin: 0;
		cursor: pointer;
	}

	.toggle span {
		font-size: var(--text-sm);
	}

	.kbd {
		display: inline-block;
		padding: 0.1rem 0.3rem;
		font-size: var(--text-xs);
		font-family: monospace;
		background: var(--bg-tertiary);
		border: 1px solid var(--border-secondary);
		border-radius: var(--radius-sm);
		margin-left: var(--space-1);
	}

	.hint {
		min-height: 1.1rem;
		font-size: var(--text-xs);
		margin-top: var(--space-2);
	}

	.hint.neutral {
		color: var(--text-muted);
	}

	.hint.error {
		color: var(--status-error);
	}

	.conflict {
		border-color: #f59e0b !important;
		background: rgba(245, 158, 11, 0.1) !important;
	}
</style>
