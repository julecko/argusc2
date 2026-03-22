<script lang="ts">
	export let open: boolean = false;
	export let title: string = '';
	export let maxWidth: string = '540px';

	function handleOverlay(e: MouseEvent) {
		if ((e.target as HTMLElement).classList.contains('modal-overlay')) {
			open = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && open) open = false;
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
	<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
	<div class="modal-overlay" on:click={handleOverlay}>
		<div class="modal" style="max-width:{maxWidth}">
			<div class="modal-header">
				<h2>{title}</h2>
				<button class="close-btn" on:click={() => (open = false)} aria-label="Close">
					<slot name="close-icon">✕</slot>
				</button>
			</div>

			<div class="modal-body">
				<slot />
			</div>

			{#if $$slots.footer}
				<div class="modal-footer">
					<slot name="footer" />
				</div>
			{/if}
		</div>
	</div>
{/if}

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.65);
		backdrop-filter: blur(4px);
		z-index: 200;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 24px;
		animation: fade-in 0.15s ease;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.modal {
		width: 100%;
		max-height: 90vh;
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		animation: modal-in 0.18s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modal-in {
		from {
			opacity: 0;
			transform: translateY(-12px) scale(0.97);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 20px;
		border-bottom: 1px solid $border;
		flex-shrink: 0;

		h2 {
			margin: 0;
			font-size: 16px;
			font-weight: 600;
			color: $text-primary;
		}
	}

	.close-btn {
		background: none;
		border: none;
		color: $text-muted;
		cursor: pointer;
		padding: 4px;
		border-radius: 6px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition:
			color $transition,
			background $transition;

		&:hover {
			color: $text-primary;
			background: $bg-card;
		}
	}

	.modal-body {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 16px;
		scrollbar-width: thin;
		scrollbar-color: $border transparent;

		&::-webkit-scrollbar {
			width: 4px;
		}
		&::-webkit-scrollbar-track {
			background: transparent;
		}
		&::-webkit-scrollbar-thumb {
			background: $border;
			border-radius: 2px;
		}
	}

	.modal-footer {
		padding: 16px 20px;
		border-top: 1px solid $border;
		flex-shrink: 0;
	}
</style>
