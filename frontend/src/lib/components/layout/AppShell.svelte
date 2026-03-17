<script lang="ts">
	import Sidebar from './Sidebar.svelte';
</script>

<div class="app">
	<Sidebar />

	<div class="main">
		<slot />
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.app {
		display: flex;
		height: 100vh;
		background: $bg-main;
	}

	.main {
		padding: 24px;
		flex: 1;
		overflow-y: auto;
	}
</style>
