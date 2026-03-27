<script lang="ts">
	import TerminalOutput from '$lib/components/device/TerminalOutput.svelte';

	let command = '';
	let output: string[] = ['$ Connected to remote command prompt', '$ Waiting for commands...'];
	let loading = false;

	async function execute() {
		if (!command.trim() || loading) return;
		const cmd = command.trim();
		command = '';
		loading = true;
		output = [...output, `$ ${cmd}`];

		// Simulate response — replace with real API call
		await new Promise((r) => setTimeout(r, 800));
		output = [...output, `> [Output of: ${cmd}]`];
		loading = false;
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') execute();
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
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
					d="M4 6l2.5 2L4 10M8.5 10H12"
					stroke="currentColor"
					stroke-width="1.2"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			</svg>
			<span class="panel-title">Command Prompt</span>
		</div>
		<p class="panel-desc">Execute one-off commands on the remote device</p>
	</div>

	<TerminalOutput lines={output} {loading} />

	<div class="input-row">
		<input
			class="cmd-input"
			type="text"
			placeholder="Enter command (e.g., whoami, dir, ipconfig)..."
			bind:value={command}
			on:keydown={onKeydown}
			disabled={loading}
		/>
		<button class="execute-btn" on:click={execute} disabled={loading || !command.trim()}>
			<svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
				<path d="M3 2l12 6-12 6V2z" />
			</svg>
			Execute
		</button>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.tab-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.panel-header {
		padding: 16px 20px 12px;
		border-bottom: 1px solid $border;
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 12px;
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

	.execute-btn {
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
		white-space: nowrap;

		&:hover:not(:disabled) {
			background: darken(#e7000b, 5%);
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}
</style>
