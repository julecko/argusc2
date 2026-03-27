<script lang="ts">
	import { afterUpdate } from 'svelte';

	export let lines: string[] = [];
	export let loading = false;

	let el: HTMLDivElement;

	afterUpdate(() => {
		if (el) el.scrollTop = el.scrollHeight;
	});
</script>

<div class="terminal" bind:this={el}>
	{#each lines as line}
		<div class="line" class:prompt={line.startsWith('$')} class:output-line={line.startsWith('>')}>
			{line}
		</div>
	{/each}
	{#if loading}
		<div class="line loading">
			<span class="cursor"></span>
		</div>
	{/if}
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.terminal {
		flex: 1;
		background: #000;
		padding: 16px 20px;
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 13px;
		line-height: 1.6;
		overflow-y: auto;
		min-height: 320px;
		max-height: 480px;
	}

	.line {
		color: #4ade80;
		white-space: pre-wrap;
		word-break: break-all;

		&.output-line {
			color: #60a5fa;
		}

		&.loading {
			display: flex;
			align-items: center;
		}
	}

	.cursor {
		display: inline-block;
		width: 8px;
		height: 14px;
		background: #4ade80;
		animation: blink 1s step-end infinite;
		margin-left: 2px;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>
