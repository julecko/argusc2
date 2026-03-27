<script lang="ts">
	type KeylogEntry = { time: string; process: string; text: string };

	let active = false;
	let entries: KeylogEntry[] = [
		{ time: '10:30:15', process: 'chrome.exe', text: 'admin@example.com' },
		{ time: '10:30:28', process: 'chrome.exe', text: 'P@ssw0rd123' },
		{ time: '10:31:45', process: 'notepad.exe', text: 'Important meeting notes' },
		{ time: '10:32:10', process: 'slack.exe', text: 'Can you send me the report?' },
		{ time: '10:35:42', process: 'outlook.exe', text: 'Meeting at 3pm tomorrow' },
		{ time: '10:38:19', process: 'chrome.exe', text: 'https://secure-bank.com' },
		{ time: '10:38:45', process: 'chrome.exe', text: 'username: john.doe' },
		{ time: '10:39:02', process: 'chrome.exe', text: 'password: ********' }
	];

	function toggleKeylogger() {
		active = !active;
	}

	function downloadLog() {
		const content = entries.map((e) => `[${e.time}] ${e.process}: "${e.text}"`).join('\n');
		const blob = new Blob([content], { type: 'text/plain' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = 'keylog.txt';
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
		<div class="header-left">
			<div class="panel-title-group">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<rect
						x="1"
						y="4"
						width="14"
						height="9"
						rx="1.5"
						stroke="currentColor"
						stroke-width="1.2"
					/>
					<path
						d="M4 7.5h1M6.5 7.5h1M9 7.5h1M11.5 7.5h1M4 10h8"
						stroke="currentColor"
						stroke-width="1.2"
						stroke-linecap="round"
					/>
				</svg>
				<span class="panel-title">Keylogger</span>
			</div>
			<p class="panel-desc">Monitor and record keystrokes from the device</p>
		</div>
		<div class="btn-group">
			<button class="start-btn" class:stop={active} on:click={toggleKeylogger}>
				{#if active}
					<svg width="11" height="11" viewBox="0 0 12 12" fill="currentColor">
						<rect x="1" y="1" width="10" height="10" rx="1" />
					</svg>
					Stop Keylogger
				{:else}
					<svg width="11" height="11" viewBox="0 0 12 12" fill="currentColor">
						<path d="M2 1l9 5-9 5V1z" />
					</svg>
					Start Keylogger
				{/if}
			</button>
			<button class="dl-btn" on:click={downloadLog} disabled={entries.length === 0}>
				<svg width="12" height="12" viewBox="0 0 16 16" fill="none">
					<path
						d="M8 1v9M5 7l3 3 3-3M2 13h12"
						stroke="currentColor"
						stroke-width="1.3"
						stroke-linecap="round"
					/>
				</svg>
				Download Log
			</button>
		</div>
	</div>

	<div class="status-bar">
		<span class="status-dot" class:active></span>
		<span class="status-text">Status: {active ? 'Active' : 'Inactive'}</span>
	</div>

	<div class="log-terminal">
		{#each entries as entry}
			<div class="log-line">
				<span class="log-time">[{entry.time}]</span>
				<span class="log-process">{entry.process}:</span>
				<span class="log-text">"{entry.text}"</span>
			</div>
		{/each}
		{#if entries.length === 0}
			<p class="empty">No keystrokes recorded yet.</p>
		{/if}
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.tab-panel {
		display: flex;
		flex-direction: column;
	}

	.panel-header {
		padding: 16px 20px 12px;
		border-bottom: 1px solid $border;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.panel-title-group {
		display: flex;
		align-items: center;
		gap: 8px;
		color: $text-muted;
	}

	.panel-title {
		font-size: 13px;
		font-weight: 600;
		color: $text-primary;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.panel-desc {
		font-size: 12px;
		color: $text-muted;
		margin: 0;
	}

	.btn-group {
		display: flex;
		gap: 8px;
		flex-shrink: 0;
	}

	.start-btn {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		padding: 8px 16px;
		background: #16a34a;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all $transition;

		&:hover {
			background: #15803d;
		}

		&.stop {
			background: #dc2626;
			&:hover {
				background: #b91c1c;
			}
		}
	}

	.dl-btn {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		padding: 8px 16px;
		background: rgba(255, 255, 255, 0.06);
		color: $text-primary;
		border: 1px solid $border;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all $transition;

		&:hover:not(:disabled) {
			background: rgba(255, 255, 255, 0.1);
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}

	.status-bar {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: rgba(0, 0, 0, 0.15);
		border-bottom: 1px solid $border;
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #3f3f3f;
		transition: background $transition;

		&.active {
			background: #22c55e;
			box-shadow: 0 0 6px rgba(34, 197, 94, 0.6);
			animation: pulse 1.5s infinite;
		}
	}

	.status-text {
		font-size: 12px;
		color: $text-muted;
	}

	.log-terminal {
		background: #000;
		padding: 16px 20px;
		font-family: 'JetBrains Mono', monospace;
		font-size: 12.5px;
		line-height: 1.7;
		min-height: 300px;
		max-height: 460px;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.log-line {
		display: flex;
		gap: 10px;
		flex-wrap: wrap;
	}

	.log-time {
		color: #6b7280;
		flex-shrink: 0;
	}

	.log-process {
		color: #f59e0b;
		flex-shrink: 0;
	}

	.log-text {
		color: #4ade80;
		word-break: break-all;
	}

	.empty {
		color: $text-muted;
		font-size: 13px;
		text-align: center;
		padding: 40px 0;
		margin: 0;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}
</style>
