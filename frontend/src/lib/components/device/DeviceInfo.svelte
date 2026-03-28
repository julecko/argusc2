<script lang="ts">
	import type { Device } from "$lib/types";

    export let device: Device;

	let collapsed = false;
</script>

<div class="info-card">
	<button class="card-header" on:click={() => (collapsed = !collapsed)}>
		<div class="header-left">
			<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
				<circle cx="8" cy="8" r="6.5" stroke="currentColor" stroke-width="1.5" />
				<path
					d="M8 7.5v3M8 5.5v.01"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
				/>
			</svg>
			<span>Device Information</span>
		</div>
		<svg
			class="chevron"
			class:flipped={collapsed}
			width="14"
			height="14"
			viewBox="0 0 16 16"
			fill="none"
		>
			<path
				d="M4 6L8 10L12 6"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</svg>
	</button>

	{#if !collapsed}
		<div class="info-grid">
			<div class="info-section">
				<h3 class="section-title">Identity</h3>
				<div class="rows">
					<div class="row">
						<span class="label">Hostname</span><span class="val mono">{device.hostname}</span>
					</div>
					<div class="row">
						<span class="label">Username</span><span class="val mono">{device.username}</span>
					</div>
					<div class="row">
						<span class="label">Program ID</span><span class="val mono">{device.programId}</span>
					</div>
				</div>
			</div>

			<div class="info-section">
				<h3 class="section-title">Network</h3>
				<div class="rows">
					<div class="row">
						<span class="label">Internal IP</span><span class="val mono">{device.internalIp}</span>
					</div>
					<div class="row">
						<span class="label">External IP</span><span class="val mono">{device.externalIp}</span>
					</div>
					<div class="row">
						<span class="label">MAC Address</span><span class="val mono">{device.macAddress}</span>
					</div>
				</div>
			</div>

			<div class="info-section">
				<h3 class="section-title">System</h3>
				<div class="rows">
					<div class="row"><span class="label">OS</span><span class="val">{device.os}</span></div>
					<div class="row">
						<span class="label">Architecture</span><span class="val">{device.architecture}</span>
					</div>
					<div class="row">
						<span class="label">Elevated</span>
						<span class="val" class:elevated={device.elevated}
							>{device.elevated ? 'Yes' : 'No'}</span
						>
					</div>
				</div>
			</div>

			<div class="info-section">
				<h3 class="section-title">Hardware</h3>
				<div class="rows">
					<div class="row"><span class="label">CPU</span><span class="val">{device.cpu}</span></div>
					<div class="row">
						<span class="label">Cores</span><span class="val">{device.cores}</span>
					</div>
					<div class="row"><span class="label">RAM</span><span class="val">{device.ram}</span></div>
					<div class="row">
						<span class="label">Disk</span><span class="val">{device.disk}</span>
					</div>
				</div>
			</div>

			<div class="info-section">
				<h3 class="section-title">Location</h3>
				<div class="rows">
					<div class="row">
						<span class="label">Country</span><span class="val">{device.country}</span>
					</div>
					<div class="row">
						<span class="label">City</span><span class="val">{device.city}</span>
					</div>
					<div class="row">
						<span class="label">Timezone</span><span class="val">{device.timezone}</span>
					</div>
				</div>
			</div>

			<div class="info-section">
				<h3 class="section-title">Activity</h3>
				<div class="rows">
					<div class="row">
						<span class="label">First Seen</span><span class="val mono">{device.firstSeen}</span>
					</div>
					<div class="row">
						<span class="label">Last Seen</span><span class="val mono">{device.lastSeen}</span>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.info-card {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		overflow: hidden;
	}

	.card-header {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 14px 20px;
		background: none;
		border: none;
		border-bottom: 1px solid $border;
		cursor: pointer;
		color: $text-muted;
		font-size: 12px;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		transition: background $transition;

		&:hover {
			background: rgba(255, 255, 255, 0.02);
		}

		.header-left {
			display: flex;
			align-items: center;
			gap: 8px;
		}
	}

	.chevron {
		transition: transform $transition;
		&.flipped {
			transform: rotate(-90deg);
		}
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
	}

	.info-section {
		padding: 20px;
		border-right: 1px solid $border;
		border-bottom: 1px solid $border;

		&:last-child {
			border-right: none;
		}
	}

	.section-title {
		font-size: 10px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 1.2px;
		color: white;
		margin: 0 0 14px 0;
	}

	.rows {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.row {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.label {
		font-size: 11px;
		color: $text-primary;
		letter-spacing: 0.3px;
	}

	.val {
		font-size: 13px;
		color: $text-muted;
		word-break: break-all;

		&.mono {
			font-family: 'JetBrains Mono', monospace;
			font-size: 12px;
		}

		&.elevated {
			color: $accent;
			font-weight: 700;
		}
	}
</style>
