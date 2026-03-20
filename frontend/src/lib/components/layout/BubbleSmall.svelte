<script>
	export let title;
	export let text;
    export let size;
	export let icon;
	export let iconColor;
</script>

<div class="bubble">
	<div class="bubble__inner">
		<div class="bubble__header">
			<span class="bubble__title">{title}</span>
			<span class="bubble__icon" style="color: {iconColor};width: {size}px; height: {size}px;">
				{@html icon}
			</span>
		</div>
		<div class="bubble__text">{text}</div>
	</div>
</div>

<style>
	.bubble {
		position: relative;
		display: inline-flex;
		width: 180px;
		height: 110px;
		border-radius: 18px;
		background: linear-gradient(145deg, #1e2330, #161a24);
		box-shadow:
			0 4px 24px rgba(0, 0, 0, 0.45),
			0 1px 0 rgba(255, 255, 255, 0.04) inset,
			0 -1px 0 rgba(0, 0, 0, 0.3) inset;
		overflow: hidden;
		cursor: default;
		transition:
			transform 0.2s ease,
			box-shadow 0.2s ease;
	}

	.bubble:hover {
		transform: translateY(-2px) scale(1.02);
		box-shadow:
			0 8px 32px rgba(0, 0, 0, 0.55),
			0 1px 0 rgba(255, 255, 255, 0.06) inset;
	}

	.bubble__inner {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 16px 18px;
		width: 100%;
	}

	.bubble__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 8px;
	}

	.bubble__title {
		font-family: 'DM Sans', 'Segoe UI', sans-serif;
		font-size: 0.72rem;
		font-weight: 500;
		letter-spacing: 0.02em;
		color: #6b9ac4;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.bubble__icon {
		font-size: 1.1rem;
		line-height: 1;
		flex-shrink: 0;
		filter: drop-shadow(0 0 6px currentColor);
		transition: filter 0.3s ease;
	}

	.bubble:hover .bubble__icon {
		filter: drop-shadow(0 0 10px currentColor);
	}

	.bubble__text {
		font-family: 'DM Mono', 'Courier New', monospace;
		font-size: 1.85rem;
		font-weight: 600;
		color: #dce6f0;
		letter-spacing: -0.03em;
		line-height: 1;
	}
</style>
