<script lang="ts">
	import UnknownIcon from "$assets/FilenotfoundIcon.svg?raw"
    export let size: number = 24;
	export let color: string = 'currentColor';
	export let svg: string = UnknownIcon;
</script>

<div
    class="nav-icon"
    style="width: {size}px; height: {size}px; color: {color};"
>
    {@html svg}
</div>

<style>
	.nav-icon {
        display: inline-block;
    }
	.nav-icon :global(svg) {
		width: 100%;
		height: 100%;
		stroke: currentColor;
		fill: none;
		stroke-width: 2;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
