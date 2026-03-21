<script lang="ts">
	export let title: string;
	export let description: string;

	let searchOpen = false;
	let searchQuery = '';
	let notifOpen = false;
	let searchInput: HTMLInputElement;

	function openSearch() {
		searchOpen = true;
		setTimeout(() => searchInput?.focus(), 0);
	}

	function closeSearch() {
		searchOpen = false;
		searchQuery = '';
	}

	function handleOverlayClick(e: MouseEvent) {
		if ((e.target as HTMLElement).classList.contains('search-overlay')) {
			closeSearch();
		}
	}

	function handleOutsideClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('.notif-wrap')) notifOpen = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (searchOpen) closeSearch();
			if (notifOpen) notifOpen = false;
		}
		if (e.ctrlKey && e.key === 'k') {
			e.preventDefault();
			openSearch();
		}
	}

	const notifications = [
		{ id: 1, text: 'DESKTOP-WIN10-01 came online', time: '2m ago', unread: true },
		{ id: 2, text: 'New file uploaded by UBUNTU-SERVER-02', time: '14m ago', unread: true },
		{ id: 3, text: 'LAPTOP-CORP-04 went offline', time: '1h ago', unread: false }
	];

	$: unreadCount = notifications.filter((n) => n.unread).length;
</script>

<svelte:window on:click={handleOutsideClick} on:keydown={handleKeydown} />

{#if searchOpen}
	<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
	<div class="search-overlay" on:click={handleOverlayClick}>
		<div class="search-modal">
			<div class="search-modal-input-row">
				<svg
					class="search-modal-icon"
					width="18"
					height="18"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
				</svg>
				<input
					bind:this={searchInput}
					class="search-modal-input"
					type="text"
					placeholder="Search devices, accounts, uploads…"
					bind:value={searchQuery}
				/>
			</div>
			{#if searchQuery.length === 0}
				<div class="search-modal-hint">
					<span>Start typing to search</span>
					<span class="search-modal-tip"><kbd>Ctrl + k</kbd> to open · <kbd>Esc</kbd> to close</span>
				</div>
			{/if}
		</div>
	</div>
{/if}

<div class="page-header">
	<div class="page-header-left">
		<h1>{title}</h1>
		<p>{description}</p>
	</div>

	<div class="page-header-right">
		<button class="icon-btn" on:click|stopPropagation={openSearch} aria-label="Search">
			<svg
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
			</svg>
		</button>

		<div class="notif-wrap">
			<button
				class="icon-btn"
				class:icon-btn--active={notifOpen}
				on:click|stopPropagation={() => {
					notifOpen = !notifOpen;
				}}
				aria-label="Notifications"
			>
				<svg
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
					<path d="M13.73 21a2 2 0 0 1-3.46 0" />
				</svg>
				{#if unreadCount > 0}
					<span class="notif-badge">{unreadCount}</span>
				{/if}
			</button>

			{#if notifOpen}
				<div class="dropdown notif-dropdown">
					<div class="dropdown-header">
						<span>Notifications</span>
						<button class="mark-read">Mark all read</button>
					</div>
					{#each notifications as n (n.id)}
						<div class="notif-item" class:notif-item--unread={n.unread}>
							{#if n.unread}<span class="notif-dot"></span>{/if}
							<div class="notif-body">
								<p>{n.text}</p>
								<small>{n.time}</small>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.page-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 24px;
	}

	.page-header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;

		h1 {
			color: $text-primary;
			font-size: 30px;
			font-weight: 600;
			margin: 0;
			white-space: nowrap;
		}

		p {
			color: $text-muted;
			font-size: $text-muted-size;
			margin: 0;
		}
	}

	.page-header-right {
		display: flex;
		align-items: center;
		gap: 8px;
		flex-shrink: 0;
		padding-top: 4px;
	}

	.icon-btn {
		position: relative;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		color: $text-muted;
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		flex-shrink: 0;
		transition:
			background $transition,
			color $transition,
			border-color $transition;

		&:hover,
		&--active {
			border-color: $accent;
			color: $text-primary;
		}
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
		animation: overlay-in 0.15s ease;
	}

	@keyframes overlay-in {
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
		animation: modal-in 0.15s cubic-bezier(0.16, 1, 0.3, 1);
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

	.search-modal-input-row {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 16px;
		border-bottom: 1px solid $border;
	}

	.search-modal-icon {
		color: $text-muted;
		flex-shrink: 0;
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

	.notif-wrap {
		position: relative;
	}

	.notif-badge {
		position: absolute;
		top: 5px;
		right: 5px;
		background: $accent;
		color: white;
		font-size: 9px;
		font-weight: 700;
		width: 13px;
		height: 13px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		line-height: 1;
	}

	.dropdown {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
		z-index: 100;
		overflow: hidden;
	}

	.dropdown-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 10px 14px;
		border-bottom: 1px solid $border;
		font-size: 11px;
		font-weight: 600;
		color: $text-muted;
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}

	.mark-read {
		background: none;
		border: none;
		color: $accent;
		font-size: 11px;
		font-weight: 400;
		text-transform: none;
		letter-spacing: 0;
		cursor: pointer;
		padding: 0;

		&:hover {
			text-decoration: underline;
		}
	}

	.notif-dropdown {
		min-width: 300px;
	}

	.notif-item {
		display: flex;
		align-items: flex-start;
		gap: 10px;
		padding: 10px 14px;
		border-bottom: 1px solid $border;
		transition: background $transition;

		&:last-child {
			border-bottom: none;
		}
		&:hover {
			background: $bg-main;
		}
		&--unread {
			background: rgba($accent, 0.04);
		}
	}

	.notif-dot {
		flex-shrink: 0;
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: $accent;
		margin-top: 5px;
	}

	.notif-body {
		display: flex;
		flex-direction: column;
		gap: 3px;

		p {
			margin: 0;
			font-size: 12px;
			color: $text-primary;
			line-height: 1.4;
		}

		small {
			font-size: 11px;
			color: $text-muted;
		}
	}
</style>
