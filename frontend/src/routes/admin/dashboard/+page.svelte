<script lang="ts">
	import BubbleSmall from '$lib/components/layout/BubbleSmall.svelte';
	import PageHeader from '$components/layout/PageHeader.svelte';
	import DashboardIcon from '$assets/DashboardIcon.svg?raw';
</script>

<section class="page">
	<PageHeader title="Dashboard" description="Overview of admin panel" />
	<BubbleSmall title="Online Devices" text="2/4" size=24 icon={DashboardIcon} iconColor="#00ff00" />
</section>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	section.page {
		display: flex;
		flex-direction: column;
		gap: 16px;
		background: $bg-main;
	}
</style>
