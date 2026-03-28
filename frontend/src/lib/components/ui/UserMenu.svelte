<script lang="ts">
	import { goto } from '$app/navigation';

	export let username: string = 'Admin';
	export let email: string = 'admin@c2server.local';
	export let collapsed: boolean = false;

	let menuOpen = false;

	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	function handleOutsideClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('.user-menu-wrap')) {
			menuOpen = false;
		}
	}

	async function handleLogout() {
		try {
			await fetch('/api/auth/logout', {
				method: 'POST',
			});
		} finally {
			goto('/login');
		}
	}

	$: initials = username
		.split(/[\s_-]/)
		.map((w) => w[0]?.toUpperCase() ?? '')
		.slice(0, 2)
		.join('');
</script>

<svelte:window on:click={handleOutsideClick} />

<div class="user-menu-wrap">
	{#if menuOpen}
		<div class="user-popup" class:user-popup--collapsed={collapsed}>
			<div class="user-popup-info">
				<span class="popup-username">{username}</span>
				<span class="popup-email">{email}</span>
			</div>
			<div class="user-popup-divider"></div>
			<button class="popup-item popup-item--danger" on:click={handleLogout}>
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
					<polyline points="16 17 21 12 16 7" />
					<line x1="21" y1="12" x2="9" y2="12" />
				</svg>
				Sign out
			</button>
		</div>
	{/if}

	<button
		class="user-btn"
		class:user-btn--active={menuOpen}
		on:click|stopPropagation={toggleMenu}
		aria-label="Account menu"
		title={collapsed ? username : undefined}
	>
		<div class="avatar">{initials}</div>
		{#if !collapsed}
			<span class="user-info">
				<strong>{username}</strong>
				<small>{email}</small>
			</span>
			<svg
				class="chevron"
				class:chevron--open={menuOpen}
				width="12"
				height="12"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<polyline points="18 15 12 9 6 15" />
			</svg>
		{/if}
	</button>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	$collapse-duration: 0.25s;
	$collapse-ease: cubic-bezier(0.4, 0, 0.2, 1);

	.user-menu-wrap {
		position: relative;
		width: 100%;
	}

	.user-btn {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		box-sizing: border-box;
		background: transparent;
		border: 1px solid transparent;
		border-radius: $radius;
		cursor: pointer;
		transition:
			background $transition,
			border-color $transition;

		&:hover,
		&--active {
			background: $bg-card;
			border-color: $border;
		}
	}

	.avatar {
		background: $bg-card;
		width: 39px;
		height: 39px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		font-weight: 700;
		flex-shrink: 0;
		color: $text-primary;
		border: 1px solid $border;
		transition: border-color $transition;

		.user-btn--active & {
			border-color: $accent;
		}
	}

	.user-info {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		white-space: nowrap;
		flex: 1;
		text-align: left;
		gap: 2px;

		strong {
			color: $text-primary;
			font-size: 13px;
			font-weight: 600;
			line-height: 1;
			overflow: hidden;
			text-overflow: ellipsis;
		}

		small {
			color: $text-muted;
			font-size: 11px;
			line-height: 1;
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}

	.chevron {
		color: $text-muted;
		flex-shrink: 0;
		transition: transform $transition;
		margin-right: 5px;

		&--open {
			transform: rotate(180deg);
		}
	}

	.user-popup {
		position: absolute;
		bottom: calc(100% + 8px);
		z-index: 10;
		left: 0;
		right: 0;
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		box-shadow: 0 -8px 32px rgba(0, 0, 0, 0.4);
		overflow: hidden;
		animation: pop-up 0.15s cubic-bezier(0.16, 1, 0.3, 1);

		&--collapsed {
			left: 0;
			right: auto;
			width: 200px;
		}
	}

	@keyframes pop-up {
		from {
			opacity: 0;
			transform: translateY(6px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.user-popup-info {
		display: flex;
		flex-direction: column;
		gap: 3px;
		padding: 12px 14px;

		.popup-username {
			font-size: 13px;
			font-weight: 600;
			color: $text-primary;
		}

		.popup-email {
			font-size: 11px;
			color: $text-muted;
			overflow: hidden;
			text-overflow: ellipsis;
			white-space: nowrap;
		}
	}

	.user-popup-divider {
		height: 1px;
		background: $border;
	}

	.popup-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		background: none;
		border: none;
		padding: 10px 14px;
		font-size: 13px;
		cursor: pointer;
		text-align: left;
		transition: background $transition;
		color: $text-primary;

		svg {
			color: $text-muted;
			flex-shrink: 0;
		}

		&:hover {
			background: $bg-main;
		}

		&--danger {
			color: #f87171;
			svg {
				color: #f87171;
			}
			&:hover {
				background: rgba(#f87171, 0.08);
			}
		}
	}
</style>
