<script lang="ts">
	import IconSearch from '$components/ui/icons/SearchIcon.svelte';

	export let open: boolean = false;

	let searchInput: HTMLInputElement;
	let searchQuery = '';

	export function show() {
		open = true;
		setTimeout(() => searchInput?.focus(), 0);
	}

	export function hide() {
		open = false;
		searchQuery = '';
	}

	function handleOverlayClick(e: MouseEvent) {
		if ((e.target as HTMLElement).classList.contains('search-overlay')) hide();
	}
</script>

{#if open}
	<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
	<div class="search-overlay" on:click={handleOverlayClick}>
		<div class="search-modal">
			<div class="search-modal-input-row">
				<IconSearch size={18} color="var(--icon-muted)" />
				<input
					bind:this={searchInput}
					class="search-modal-input"
					type="text"
					placeholder="Search devices, accounts, uploads…"
					bind:value={searchQuery}
					on:keydown={(e) => e.key === 'Escape' && hide()}
				/>
				<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
				<kbd class="search-modal-esc" on:click={hide}>Esc</kbd>
			</div>

			{#if searchQuery.length === 0}
				<div class="search-modal-hint">
					<span>Start typing to search</span>
					<span class="search-modal-tip">
						<kbd>⌘K</kbd> to open · <kbd>Esc</kbd> to close
					</span>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style lang="scss">
	@use '$lib/styles/variables' as *;

	:root {
        --icon-muted: #{$text-muted};
    }

	.search-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 200;
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 15vh;
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

	.search-modal {
		width: 100%;
		max-width: 560px;
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
		overflow: hidden;
		animation: modal-drop 0.15s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modal-drop {
		from {
			opacity: 0;
			transform: translateY(-12px) scale(0.97);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.search-modal-input-row {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 16px;
		border-bottom: 1px solid $border;
	}

	.search-modal-input {
		flex: 1;
		background: transparent;
		border: none;
		outline: none;
		font-size: 16px;
		color: $text-primary;

		&::placeholder {
			color: $text-muted;
		}
	}

	.search-modal-esc {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 4px;
		color: $text-muted;
		font-size: 11px;
		padding: 2px 7px;
		cursor: pointer;
		flex-shrink: 0;
		font-family: monospace;

		&:hover {
			border-color: $accent;
			color: $text-primary;
		}
	}

	.search-modal-hint {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		font-size: 12px;
		color: $text-muted;
	}

	.search-modal-tip {
		display: flex;
		align-items: center;
		gap: 4px;

		kbd {
			background: $bg-card;
			border: 1px solid $border;
			border-radius: 4px;
			font-size: 10px;
			padding: 1px 5px;
			font-family: monospace;
			color: $text-muted;
		}
	}
</style>
