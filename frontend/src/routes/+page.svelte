<script lang="ts">
	import { onMount } from 'svelte';

	let apiMessage = 'Loading...';

	onMount(async () => {
		try {
			const res = await fetch('/api/hello');
			if (!res.ok) throw new Error(String(res.status));
			apiMessage = await res.text();
		} catch (err) {
			apiMessage = err instanceof Error ? `Failed: ${err.message}` : 'Unknown error';
		}
	});
</script>

<section>
	<h1>Dashboard</h1>
	<p>{apiMessage}</p>
</section>

<style lang="scss">
	section {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
</style>
