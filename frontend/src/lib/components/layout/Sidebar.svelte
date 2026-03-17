<script lang="ts">
    import Icon from '$components/ui/Icon.svelte';

	type NavItem = {
		label: string;
		icon?: string;
		active?: boolean;
	};

	const navItems: NavItem[] = [
		{ label: 'Dashboard', active: true },
		{ label: 'Devices' },
		{ label: 'Accounts' },
		{ label: 'Uploads' }
	];
</script>

<aside class="sidebar">
	<div class="logo">
		<Icon size={40} />
		<div>
			<strong>C2 Control</strong>
			<small>Command & Control</small>
		</div>
	</div>

	<nav>
		{#each navItems as item}
			<button class:item-active={item.active}>
				{item.label}
			</button>
		{/each}
	</nav>

	<div class="user">
		<div class="avatar">AD</div>
		<div>
			<strong>Admin</strong>
			<small>admin@c2server.local</small>
		</div>
	</div>
</aside>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.sidebar {
		width: 260px;
		background: $bg-sidebar;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 16px;
		border-right: 1px solid $border;
	}

	.logo {
		display: flex;
		gap: 12px;
		align-items: center;

		small {
			color: $text-muted;
		}
	}

	nav {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: 32px;

		button {
			background: transparent;
			border: none;
			color: $text-muted;
			padding: 10px 12px;
			border-radius: $radius;
			text-align: left;
			cursor: pointer;
			transition: $transition;

			&:hover {
				background: $bg-card;
				color: $text-primary;
			}
		}

		.item-active {
			background: $accent;
			color: white;
		}
	}

	.user {
		display: flex;
		gap: 10px;
		align-items: center;

		small {
			color: $text-muted;
		}
	}

	.avatar {
		background: $bg-card;
		padding: 10px;
		border-radius: 50%;
	}
</style>
