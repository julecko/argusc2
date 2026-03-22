<script lang="ts">
	import IconSearch from '$components/ui/icons/SearchIcon.svelte';
	import IconBell from '$components/ui/icons/BellIcon.svelte';
	import SearchModal from '$lib/components/ui/modals/SearchModal.svelte';
	import NotificationPanel from '$lib/components/ui/NotificationPanel.svelte';
	import type { Notification } from '$lib/types';

	export let title: string;
	export let description: string;

	let searchOpen = false;
	let notifOpen = false;
	let searchModal: SearchModal;
	let notifications: Notification[] = [
		{
			id: 1,
			type: 'device',
			title: 'New Device Connected',
			text: 'DESKTOP-WIN10-01 has connected from 192.168.1.105',
			time: 'about 2 hours ago',
			unread: true
		},
		{
			id: 2,
			type: 'device',
			title: 'Device Offline',
			text: 'LAPTOP-CORP-04 went offline',
			time: '1 day ago',
			unread: false
		},
		{
			id: 3,
			type: 'upload',
			title: 'Upload Completed',
			text: 'payload_v2.exe uploaded successfully',
			time: '1 day ago',
			unread: true
		},
		{
			id: 4,
			type: 'system',
			title: 'System Update',
			text: 'C2 Server v2.3.1 is now running',
			time: '3 days ago',
			unread: false
		}
	];

	$: unreadCount = notifications.filter((n) => n.unread).length;

	function handleKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			searchModal?.show();
		}
		if (e.key === 'Escape') {
			searchOpen = false;
			notifOpen = false;
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

<SearchModal bind:this={searchModal} bind:open={searchOpen} />
<NotificationPanel bind:open={notifOpen} bind:notifications />

<div class="page-header">
	<div class="page-header-top">
		<div class="page-header-left">
			<h1>{title}</h1>
			<p>{description}</p>
		</div>

		<div class="page-header-right">
			<button class="icon-btn" on:click={() => searchModal.show()} aria-label="Search">
				<IconSearch size={16} />
			</button>
			<button
				class="icon-btn"
				class:icon-btn--active={notifOpen}
				on:click={() => (notifOpen = !notifOpen)}
				aria-label="Notifications"
			>
				<IconBell size={16} />
				{#if unreadCount > 0}
					<span class="notif-badge">{unreadCount}</span>
				{/if}
			</button>
		</div>
	</div>

	{#if $$slots.actions}
		<div class="page-header-actions">
			<slot name="actions" />
		</div>
	{/if}
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.page-header {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.page-header-top {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 24px;
	}

	.page-header-actions {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 8px;
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
</style>
