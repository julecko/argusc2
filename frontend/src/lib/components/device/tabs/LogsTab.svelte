<script lang="ts">
	type LogLevel = 'info' | 'warn' | 'error';
	type LogEntry = { timestamp: string; message: string; level: LogLevel };

	const logs: LogEntry[] = [
		{ timestamp: '2026-03-27 10:32:15', message: 'Agent started successfully', level: 'info' },
		{
			timestamp: '2026-03-27 10:32:16',
			message: 'Established connection to C2 server',
			level: 'info'
		},
		{ timestamp: '2026-03-27 10:35:22', message: 'Heartbeat sent', level: 'info' },
		{ timestamp: '2026-03-27 10:40:22', message: 'Heartbeat sent', level: 'info' },
		{ timestamp: '2026-03-27 10:45:22', message: 'Heartbeat sent', level: 'info' },
		{ timestamp: '3/27/2026, 10:03:56 AM', message: 'Keylogger started', level: 'info' },
		{ timestamp: '3/27/2026, 10:03:57 AM', message: 'Keylogger stopped', level: 'warn' },
		{ timestamp: '3/27/2026, 10:04:14 AM', message: 'Shell session started', level: 'info' },
		{ timestamp: '3/27/2026, 10:04:16 AM', message: 'Shell session terminated', level: 'warn' },
		{ timestamp: '3/27/2026, 10:04:17 AM', message: 'Shell session started', level: 'info' },
		{ timestamp: '3/27/2026, 10:04:26 AM', message: 'Shell command: hello', level: 'info' }
	];

	let filter: 'all' | LogLevel = 'all';

	$: filtered = filter === 'all' ? logs : logs.filter((l) => l.level === filter);

	function levelColor(l: LogLevel) {
		return l === 'error' ? '#f87171' : l === 'warn' ? '#fbbf24' : '#6b7280';
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
		<div class="header-left">
			<div class="panel-title-group">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<rect
						x="2"
						y="2"
						width="12"
						height="12"
						rx="2"
						stroke="currentColor"
						stroke-width="1.2"
					/>
					<path
						d="M5 5.5h6M5 8h6M5 10.5h4"
						stroke="currentColor"
						stroke-width="1.2"
						stroke-linecap="round"
					/>
				</svg>
				<span class="panel-title">Logs</span>
			</div>
			<p class="panel-desc">View all device activity and command history</p>
		</div>

		<div class="filters">
			{#each ['all', 'info', 'warn', 'error'] as f}
				<button
					class="filter-btn"
					class:active={filter === f}
					on:click={() => (filter = f as typeof filter)}>{f}</button
				>
			{/each}
		</div>
	</div>

	<div class="log-terminal">
		{#each filtered as entry}
			<div class="log-line">
				<span class="ts">[{entry.timestamp}]</span>
				<span class="msg" style="color: {levelColor(entry.level)}">{entry.message}</span>
			</div>
		{/each}
		{#if filtered.length === 0}
			<p class="empty">No log entries.</p>
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
		flex-wrap: wrap;
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

	.filters {
		display: flex;
		gap: 4px;
	}

	.filter-btn {
		padding: 5px 12px;
		border-radius: 6px;
		border: 1px solid $border;
		background: transparent;
		color: $text-muted;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		cursor: pointer;
		transition: all $transition;

		&:hover {
			background: rgba(255, 255, 255, 0.04);
			color: $text-primary;
		}
		&.active {
			background: rgba(231, 0, 11, 0.12);
			border-color: rgba(231, 0, 11, 0.3);
			color: $text-primary;
		}
	}

	.log-terminal {
		background: #000;
		padding: 16px 20px;
		font-family: 'JetBrains Mono', monospace;
		font-size: 12.5px;
		line-height: 1.8;
		min-height: 320px;
		max-height: 480px;
		overflow-y: auto;
	}

	.log-line {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.ts {
		color: #374151;
		flex-shrink: 0;
	}

	.msg {
		word-break: break-all;
	}

	.empty {
		color: $text-muted;
		font-size: 13px;
		text-align: center;
		padding: 40px 0;
		margin: 0;
	}
</style>
