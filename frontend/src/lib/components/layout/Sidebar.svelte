<script lang="ts">
	import ProjectIcon from '$components/ui/icons/ProjectIcon.svelte';
	import Icon from '$components/ui/icons/NavbarIcon.svelte';
	import DashboardSVG from '$assets/DashboardIcon.svg?raw';
	import DevicesSVG from '$assets/DevicesIcon.svg?raw';
	import UploadsIcon from '$assets/UploadsIcon.svg?raw';
	import AccountsIcon from '$assets/AccountsIcon.svg?raw';
	import UnknownIcon from '$assets/FilenotfoundIcon.svg?raw';

	type NavItem = {
		label: string;
		svg?: string;
		active: boolean;
	};

	let collapsed = false;

	let navItems = [
		{ label: 'Dashboard', svg: DashboardSVG, active: true },
		{ label: 'Devices', svg: DevicesSVG, active: false },
		{ label: 'Uploads', svg: UploadsIcon, active: false },
		{ label: 'Accounts', svg: AccountsIcon, active: false }
	] satisfies NavItem[];

	function setActive(index: number) {
		navItems = navItems.map((item, i) => ({ ...item, active: i === index }));
	}
</script>

<aside class="sidebar" class:collapsed>
	<div class="logo" aria-label="Toggle sidebar">
		<button class="logo-icon"  on:click={() => (collapsed = !collapsed)} ><ProjectIcon size={39} /></button>
		<span class="logo-text">
			<h1>Argus</h1>
			<small>Command & Control</small>
		</span>
	</div>

	<nav>
		{#each navItems as item, i}
			<button class:item-active={item.active} on:click={() => setActive(i)} title={item.label}>
				<span class="nav-icon">
					{#if item.svg}
						<Icon svg={item.svg} size={39} />
					{:else}
						<Icon svg={UnknownIcon} size={39} />
					{/if}
				</span>
				<span class="nav-label">{item.label}</span>
			</button>
		{/each}
	</nav>

	<div class="user">
		<div class="avatar">AD</div>
		<span class="user-info">
			<strong>Admin</strong>
			<small>admin@c2server.local</small>
		</span>
	</div>
</aside>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	$sidebar-expanded: 250px;
	$sidebar-collapsed: 64px;
	$collapse-duration: 0.25s;
	$collapse-ease: cubic-bezier(0.4, 0, 0.2, 1);

	.sidebar {
		width: $sidebar-expanded;
		min-width: $sidebar-expanded;
		background: $bg-sidebar;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 17px 12px;
        gap: 17px;
		border-right: 1px solid $border;
		transition:
			width $collapse-duration $collapse-ease,
			min-width $collapse-duration $collapse-ease,
            padding $collapse-duration $collapse-ease;
		overflow: hidden;

		&.collapsed {
			width: $sidebar-collapsed;
			min-width: $sidebar-collapsed;
		}
	}

	.logo {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		border: none;
		width: 100%;
		background: transparent;
		border-radius: $radius;
		color: $text-primary;
		text-align: left;
		transition:
                max-width $collapse-duration $collapse-ease,
                width $collapse-duration $collapse-ease,
				color $collapse-duration $collapse-ease;
	}

	.logo-icon {
		display: flex;
		align-items: center;
		flex-shrink: 0;
		width: 39px;
		height: 39px;
        cursor: pointer;
        display: flex;
		background: transparent;
		border: none;

        &:hover :global(.icon) {
            background-color: $second-accent-hover;
        }

        :global(.icon) {
            transition: background-color $collapse-duration $collapse-ease;
        }
	}

	.logo-text {
        margin-left: 10px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		white-space: nowrap;
        gap: 6px;
		max-width: 180px;
		opacity: 1;
		transition:
			max-width $collapse-duration $collapse-ease;

		.collapsed & {
			max-width: 0;
		}

		h1 {
			color: white;
			font-size: 27px;
			font-weight: 600;
			line-height: 1;
			margin: 0;
		}

		small {
			color: $text-muted;
			font-size: 12px;
			line-height: 1;
			white-space: nowrap;
		}
	}

	nav {
		display: flex;
		flex: 1;
		flex-direction: column;
		gap: 4px;
        margin-left: -12px;
        margin-right: -12px;
        padding: 17px 12px;
        border-top: 1px solid $border;
        border-bottom: 1px solid $border;

		button {
			display: flex;
			align-items: center;
			gap: 10px;
			background: transparent;
			border: none;
			color: $text-muted;
			width: 100%;
			border-radius: $radius;
			cursor: pointer;
			transition:
                max-width $collapse-duration $collapse-ease,
                width $collapse-duration $collapse-ease,
				background-color $collapse-duration $collapse-ease,
				color $collapse-duration $collapse-ease;

			&:not(.item-active):hover {
				background-color: $bg-card;
				color: $text-primary;
			}
		}

		.item-active {
			background: $accent;
			color: white;
		}
	}

	.nav-icon {
		display: flex;
		align-items: center;
		flex-shrink: 0;
		width: 18px;
		margin-left: 10px;
	}

	.nav-label {
		white-space: nowrap;
		overflow: hidden;
		font-size: medium;
		max-width: 160px;
        padding-left: 10px;
		opacity: 1;
		transition:
			max-width $collapse-duration $collapse-ease;

		.collapsed & {
			max-width: 0;
		}
	}

	.user {
		display: flex;
		align-items: center;
        transition:
            max-width $collapse-duration $collapse-ease,
            width $collapse-duration $collapse-ease,
            border $collapse-duration $collapse-ease,
            background $collapse-duration $collapse-ease,
            color $collapse-duration $collapse-ease;

		small {
			color: $text-muted;
		}

		strong {
			color: $text-primary;
			display: block;
			font-size: 13px;
		}

        .collapsed & {
			width: 39;
            height: 39;
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
		font-weight: 600;
		flex-shrink: 0;
		color: $text-primary;
	}

	.user-info {
		display: flex;
		flex-direction: column;
		overflow: hidden;
		white-space: nowrap;
		max-width: 160px;
        margin-left: 10px;
		opacity: 1;
		transition:
			max-width $collapse-duration $collapse-ease;

		.collapsed & {
			max-width: 0;
		}
	}
</style>
