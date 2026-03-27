<script lang="ts">
	import TerminalOutput from '$lib/components/device/TerminalOutput.svelte';

	let command = '';
	let loading = false;
	let sessionActive = true;

	let lines = [
		'Microsoft Windows [Version Windows 10 Pro]',
		' (c) Microsoft Corporation. All rights reserved.',
		'C:\\Users\\admin>'
	];

	async function send() {
		if (!command.trim() || loading || !sessionActive) return;
		const cmd = command.trim();
		command = '';
		lines = [...lines, cmd, '> Processing...'];
		loading = true;

		await new Promise((r) => setTimeout(r, 700));
		lines = [...lines.slice(0, -1), '[Simulated output from command]', 'C:\\Users\\admin>'];
		loading = false;
	}

	function killShell() {
		sessionActive = false;
		lines = [...lines, '\n[Session terminated]'];
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') send();
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
		<div class="header-left">
			<div class="panel-title-group">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<rect
						x="1.5"
						y="1.5"
						width="13"
						height="13"
						rx="2"
						stroke="currentColor"
						stroke-width="1.2"
					/>
					<path
						d="M4 5.5l3 2.5-3 2.5M8.5 10.5H12"
						stroke="currentColor"
						stroke-width="1.2"
						stroke-linecap="round"
					/>
				</svg>
				<span class="panel-title">Shell Session</span>
			</div>
			<p class="panel-desc">Start an interactive shell session with persistent state</p>
		</div>
		<button class="kill-btn" on:click={killShell} disabled={!sessionActive}>
			<svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor">
				<rect x="2" y="2" width="12" height="12" rx="2" />
			</svg>
			Kill Shell
		</button>
	</div>

	<TerminalOutput {lines} {loading} />

	<div class="input-row">
		<input
			class="cmd-input"
			type="text"
			placeholder="Enter shell command..."
			bind:value={command}
			on:keydown={onKeydown}
			disabled={loading || !sessionActive}
		/>
		<button
			class="send-btn"
			on:click={send}
			disabled={loading || !command.trim() || !sessionActive}
		>
			<svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
				<path d="M3 2l12 6-12 6V2z" />
			</svg>
			Send
		</button>
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

	.kill-btn {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		padding: 8px 16px;
		background: $accent;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all $transition;
		flex-shrink: 0;

		&:hover:not(:disabled) {
			background: darken(#e7000b, 5%);
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}

	.input-row {
		display: flex;
		gap: 10px;
		padding: 14px 20px;
		border-top: 1px solid $border;
		background: rgba(0, 0, 0, 0.2);
	}

	.cmd-input {
		flex: 1;
		background: rgba(0, 0, 0, 0.4);
		border: 1px solid $border;
		border-radius: 8px;
		padding: 10px 16px;
		color: $text-primary;
		font-size: 13px;
		font-family: 'JetBrains Mono', monospace;
		outline: none;
		transition: border-color $transition;

        &::placeholder {
            color: $text-muted;
        }
		&:focus {
			border-color: rgba(231, 0, 11, 0.4);
		}
		&:disabled {
			opacity: 0.5;
		}
	}

	.send-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: $accent;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all $transition;

		&:hover:not(:disabled) {
			background: darken(#e7000b, 5%);
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}
</style>
