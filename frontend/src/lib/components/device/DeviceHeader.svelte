<script lang="ts">
	export let name: string;
	export let status: 'online' | 'offline';
	export let elevated: boolean;
	export let deviceId: string;
</script>

<div class="header">
	<a href="/admin/devices" class="back-link">
		<svg width="16" height="16" viewBox="0 0 16 16" fill="none">
			<path
				d="M10 12L6 8L10 4"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</svg>
		Back to Devices
	</a>

	<div class="title-row">
		<h1 class="device-name">{name}</h1>

		<span class="badge badge--{status}">
			<span class="dot"></span>
			{status}
		</span>

		{#if elevated}
			<span class="badge badge--elevated">
				<svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
					<path d="M8 1L10.5 6H15L11 9.5L12.5 15L8 12L3.5 15L5 9.5L1 6H5.5L8 1Z" />
				</svg>
				ELEVATED
			</span>
		{/if}
	</div>

	<p class="device-id">Device ID: {deviceId}</p>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.header {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: $text-muted;
		text-decoration: none;
		font-size: 13px;
		transition: color $transition;
		width: fit-content;

		&:hover {
			color: $text-primary;
		}
	}

	.title-row {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.device-name {
		font-size: 28px;
		font-weight: 700;
		color: $text-primary;
		letter-spacing: -0.5px;
		margin: 0;
	}

	.badge {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		padding: 3px 10px;
		border-radius: 20px;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.5px;
		text-transform: uppercase;

		&--online {
			background: rgba(34, 197, 94, 0.15);
			color: #22c55e;
			border: 1px solid rgba(34, 197, 94, 0.3);

			.dot {
				width: 6px;
				height: 6px;
				border-radius: 50%;
				background: #22c55e;
				animation: pulse 2s infinite;
			}
		}

		&--offline {
			background: rgba(159, 159, 154, 0.1);
			color: $text-muted;
			border: 1px solid $border;

			.dot {
				width: 6px;
				height: 6px;
				border-radius: 50%;
				background: $text-muted;
			}
		}

		&--elevated {
			background: rgba(231, 0, 11, 0.15);
			color: $accent;
			border: 1px solid rgba(231, 0, 11, 0.3);
		}
	}

	.device-id {
		color: $text-muted;
		font-size: 12px;
		margin: 0;
		font-family: 'JetBrains Mono', monospace;
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
