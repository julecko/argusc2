<script lang="ts">
	import UnknownIcon from "$assets/FilenotfoundIcon.svg?raw"
    export let size: number = 24;
	export let color: string = 'currentColor';
	export let svg: string = '';
</script>

{#if svg}
	<div
		class="nav-icon"
		style="width: {size}px; height: {size}px; color: {color};"
	>
		{@html svg}
	</div>
{:else}
	<div
		class="nav-icon"
		style="width: {size}px; height: {size}px; color: {color};"
	>
        {@html UnknownIcon}
    </div>
{/if}

<style>
	.nav-icon {
        display: inline-block;
    }
	.nav-icon svg {
		width: 100%;
		height: 100%;
		stroke: currentColor;
		fill: none;
		stroke-width: 2;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
