<script lang="ts">
	import IconBell from '$components/ui/icons/BellIcon.svelte';
	import IconX from '$components/ui/icons/XIcon.svelte';
	import IconCheck from '$components/ui/icons/CheckIcon.svelte';
	import IconTrash from '$components/ui/icons/TrashIcon.svelte';
	import IconMonitor from '$components/ui/icons/MonitorIcon.svelte';
	import IconUpload from '$components/ui/icons/UploadIcon.svelte';
	import IconSettings from '$components/ui/icons/SettingsIcon.svelte';
	import type { ComponentType } from 'svelte';
	import type { Notification, NotifType } from '$lib/types';

	export let open: boolean = false;

	export let notifications: Notification[];

	$: unreadCount = notifications.filter((n) => n.unread).length;

	const typeIcon: Record<NotifType, ComponentType> = {
		device: IconMonitor,
		upload: IconUpload,
		system: IconSettings
	};

	const typeColor: Record<NotifType, string> = {
		device: '#f87171',
		upload: '#f5a623',
		system: '#60a5fa'
	};

	function markAllRead() {
		notifications = notifications.map((n) => ({ ...n, unread: false }));
	}

	function clearAll() {
		notifications = [];
	}

	function dismiss(id: number) {
		notifications = notifications.filter((n) => n.id !== id);
	}

	function handleOverlayClick(e: MouseEvent) {
		if ((e.target as HTMLElement).classList.contains('notif-overlay')) open = false;
	}
</script>

{#if open}
	<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
	<div class="notif-overlay" on:click={handleOverlayClick}>
		<aside class="notif-panel">
			<div class="notif-panel-header">
				<div class="notif-panel-title">
					<IconBell size={16} />
					<span>Notifications</span>
					{#if unreadCount > 0}
						<span class="notif-panel-badge">{unreadCount}</span>
					{/if}
				</div>
				<button class="icon-btn" on:click={() => (open = false)} aria-label="Close panel">
					<IconX size={16} />
				</button>
			</div>

			<div class="notif-panel-actions">
				<button class="action-btn" on:click={markAllRead}>
					<IconCheck size={13} />
					Mark all read
				</button>
				<button class="action-btn action-btn--danger" on:click={clearAll}>
					<IconTrash size={13} />
					Clear all
				</button>
			</div>

			<div class="notif-panel-list">
				{#each notifications as n (n.id)}
					<div class="notif-card" class:notif-card--unread={n.unread}>
						<div class="notif-card-icon notif-card-icon--{n.type}">
							<svelte:component this={typeIcon[n.type]} size={16} color={typeColor[n.type]} />
						</div>

						<div class="notif-card-body">
							<p class="notif-card-title">{n.title}</p>
							<p class="notif-card-text">{n.text}</p>
							<div class="notif-card-meta">
								<small>{n.time}</small>
								{#if n.unread}<span class="unread-dot"></span>{/if}
							</div>
						</div>

						<button
							class="dismiss-btn"
							on:click|stopPropagation={() => dismiss(n.id)}
							aria-label="Dismiss notification"
						>
							<IconX size={12} />
						</button>
					</div>
				{:else}
					<p class="notif-empty">No notifications</p>
				{/each}
			</div>
		</aside>
	</div>
{/if}

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.notif-overlay {
		position: fixed;
		inset: 0;
		z-index: 150;
	}

	.notif-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: 360px;
		height: 100vh;
		background: $bg-sidebar;
		border-left: 1px solid $border;
		box-shadow: -8px 0 40px rgba(0, 0, 0, 0.5);
		display: flex;
		flex-direction: column;
		z-index: 151;
		animation: slide-in 0.22s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes slide-in {
		from {
			transform: translateX(100%);
		}
		to {
			transform: translateX(0);
		}
	}

	.notif-panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 20px;
		border-bottom: 1px solid $border;
		flex-shrink: 0;
	}

	.notif-panel-title {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 15px;
		font-weight: 600;
		color: $text-primary;

		:global(svg) {
			color: $accent;
		}
	}

	.notif-panel-badge {
		background: $accent;
		color: white;
		font-size: 10px;
		font-weight: 700;
		padding: 1px 6px;
		border-radius: 20px;
		line-height: 1.6;
	}

	.icon-btn {
		background: none;
		border: none;
		color: $text-muted;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 4px;
		border-radius: 6px;
		transition:
			color $transition,
			background $transition;

		&:hover {
			color: $text-primary;
			background: $bg-card;
		}
	}

	.notif-panel-actions {
		display: flex;
		gap: 8px;
		padding: 12px 20px;
		border-bottom: 1px solid $border;
		flex-shrink: 0;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		color: $text-muted;
		font-size: 12px;
		padding: 6px 12px;
		cursor: pointer;
		transition:
			color $transition,
			border-color $transition,
			background $transition;

		&:hover {
			color: $text-primary;
			border-color: $text-muted;
		}

		&--danger:hover {
			color: #f87171;
			border-color: #f87171;
			background: rgba(#f87171, 0.06);
		}
	}

	.notif-panel-list {
		flex: 1;
		overflow-y: auto;
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

	.notif-empty {
		padding: 48px 20px;
		text-align: center;
		color: $text-muted;
		font-size: 13px;
		margin: 0;
	}

	.notif-card {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 14px 20px;
		border-bottom: 1px solid $border;
		position: relative;
		transition: background $transition;

		&:last-child {
			border-bottom: none;
		}
		&:hover {
			background: $bg-main;
		}
		&--unread {
			background: rgba($accent, 0.03);
		}
	}

	.notif-card-icon {
		width: 36px;
		height: 36px;
		border-radius: $radius;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;

		&--device {
			background: rgba(#f87171, 0.12);
			border: 1px solid rgba(#f87171, 0.25);
		}
		&--upload {
			background: rgba(#f5a623, 0.12);
			border: 1px solid rgba(#f5a623, 0.25);
		}
		&--system {
			background: rgba(#60a5fa, 0.12);
			border: 1px solid rgba(#60a5fa, 0.25);
		}
	}

	.notif-card-body {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 3px;
	}

	.notif-card-title {
		margin: 0;
		font-size: 13px;
		font-weight: 600;
		color: $text-primary;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.notif-card-text {
		margin: 0;
		font-size: 12px;
		color: $text-muted;
		line-height: 1.4;
	}

	.notif-card-meta {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-top: 2px;

		small {
			font-size: 11px;
			color: $text-muted;
		}
	}

	.unread-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: $accent;
		flex-shrink: 0;
	}

	.dismiss-btn {
		background: none;
		border: none;
		color: $text-muted;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2px;
		border-radius: 4px;
		opacity: 0;
		flex-shrink: 0;
		transition:
			opacity $transition,
			color $transition,
			background $transition;

		.notif-card:hover & {
			opacity: 1;
		}
		&:hover {
			color: $text-primary;
			background: $bg-card;
		}
	}
</style>
