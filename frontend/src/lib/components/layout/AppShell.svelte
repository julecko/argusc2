<script lang="ts">
	import Sidebar from './Sidebar.svelte';
    import { page } from '$app/stores';
</script>

<div class="app">
	<Sidebar />

	<div class="app-main">
        {#key $page.url.pathname}
            <div style="margin: 20px 10px 20px 10px">
                <slot />
            </div>
        {/key}
    </div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.app {
		display: flex;
		height: 100vh;
		background: $bg-main;
	}

	.app-main {
		padding: 24px;
		flex: 1;
		overflow-y: auto;
	}
</style>
