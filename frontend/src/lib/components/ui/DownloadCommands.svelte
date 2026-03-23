<script lang="ts">
	export let filename: string;
	export let file_hash: string;

	$: baseUrl = typeof window !== 'undefined' ? window.location.origin : '';
	$: fileUrl = `${baseUrl}/api/download/programs/${file_hash}`;

	// PowerShell variant toggle
	let psVariant: 'long' | 'short' = 'long';

	$: psLong = `Invoke-WebRequest -Uri "${fileUrl}" -OutFile "${filename}"\n.\\${filename}`;
	$: psShort = `IWR "${fileUrl}" -O "${filename}"; .\\${filename}`;

	$: psCode = psVariant === 'long' ? psLong : psShort;

	let copied: string | null = null;

	async function copy(id: string, text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copied = id;
			setTimeout(() => {
				copied = null;
			}, 2000);
		} catch {}
	}
</script>

<div class="download-commands">
	<!-- PowerShell -->
	<div class="command-block">
		<div class="command-header">
			<span class="command-label">PowerShell (Windows)</span>
			<div class="header-right">
				<!-- Variant toggle -->
				<div class="variant-toggle">
					<button
						class="variant-btn"
						class:variant-btn--active={psVariant === 'long'}
						on:click={() => (psVariant = 'long')}>Long</button
					>
					<button
						class="variant-btn"
						class:variant-btn--active={psVariant === 'short'}
						on:click={() => (psVariant = 'short')}>Short</button
					>
				</div>
				<button
					class="copy-btn"
					class:copy-btn--copied={copied === 'ps'}
					on:click={() => copy('ps', psCode)}
					aria-label="Copy PowerShell command"
				>
					{#if copied === 'ps'}
						<svg
							width="13"
							height="13"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<polyline points="20 6 9 17 4 12" />
						</svg>
						Copied
					{:else}
						<svg
							width="13"
							height="13"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
							<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
						</svg>
						Copy
					{/if}
				</button>
			</div>
		</div>
		<div class="command-body command-body--ps">
			<code class="command-code command-code--ps">{psCode}</code>
		</div>
	</div>

	<!-- cURL -->
	<div class="command-block">
		<div class="command-header">
			<span class="command-label">cURL (Linux/macOS)</span>
			<button
				class="copy-btn"
				class:copy-btn--copied={copied === 'curl'}
				on:click={() =>
					copy('curl', `curl -o ${filename} ${fileUrl} && chmod +x ${filename} && ./${filename}`)}
				aria-label="Copy cURL command"
			>
				{#if copied === 'curl'}
					<svg
						width="13"
						height="13"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Copied
				{:else}
					<svg
						width="13"
						height="13"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
					</svg>
					Copy
				{/if}
			</button>
		</div>
		<div class="command-body command-body--sh">
			<code class="command-code command-code--sh"
				>curl -o {filename} {fileUrl} && chmod +x {filename} && ./{filename}</code
			>
		</div>
	</div>

	<!-- Direct URL -->
	<div class="command-block">
		<div class="command-header">
			<span class="command-label">Direct Download URL</span>
			<button
				class="copy-btn"
				class:copy-btn--copied={copied === 'url'}
				on:click={() => copy('url', fileUrl)}
				aria-label="Copy direct URL"
			>
				{#if copied === 'url'}
					<svg
						width="13"
						height="13"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Copied
				{:else}
					<svg
						width="13"
						height="13"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
					</svg>
					Copy
				{/if}
			</button>
		</div>
		<div class="command-body command-body--url">
			<code class="command-code command-code--url">{fileUrl}</code>
		</div>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.download-commands {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	// ── Command block ─────────────────────────────────────────
	.command-block {
		border: 1px solid $border;
		border-radius: $radius;
		overflow: hidden;
	}

	.command-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 14px;
		background: rgba(0, 0, 0, 0.12);
		border-bottom: 1px solid $border;
	}

	.command-label {
		font-size: 12px;
		font-weight: 500;
		color: $text-muted;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	// ── Variant toggle ────────────────────────────────────────
	.variant-toggle {
		display: flex;
		border: 1px solid $border;
		border-radius: 5px;
		overflow: hidden;
	}

	.variant-btn {
		background: $bg-card;
		border: none;
		color: $text-muted;
		font-size: 10px;
		font-weight: 600;
		padding: 3px 9px;
		cursor: pointer;
		letter-spacing: 0.04em;
		transition:
			background $transition,
			color $transition;

		&:not(:last-child) {
			border-right: 1px solid $border;
		}

		&--active {
			background: $accent;
			color: white;
		}

		&:not(&--active):hover {
			background: $bg-main;
			color: $text-primary;
		}
	}

	// ── Copy button ───────────────────────────────────────────
	.copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 5px;
		background: $bg-card;
		border: 1px solid $border;
		border-radius: 5px;
		color: $text-muted;
		font-size: 11px;
		font-weight: 500;
		padding: 3px 9px;
		cursor: pointer;
		transition:
			color $transition,
			border-color $transition,
			background $transition;

		&:hover {
			color: $text-primary;
			border-color: $text-muted;
		}

		&--copied {
			color: #34d87a;
			border-color: #34d87a;
			background: rgba(#34d87a, 0.08);
		}
	}

	// ── Code blocks ───────────────────────────────────────────
	.command-body {
		padding: 12px 14px;

		&--ps {
			background: rgba(#34d87a, 0.03);
		}
		&--sh {
			background: rgba(#34d87a, 0.03);
		}
		&--url {
			background: rgba(#1f77b4, 0.04);
		}
	}

	.command-code {
		font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
		font-size: 11.5px;
		white-space: pre-wrap;
		word-break: break-all;
		line-height: 1.6;
		display: block;

		&--ps {
			color: #7ee787;
		}
		&--sh {
			color: #7ee787;
		}
		&--url {
			color: #79c0ff;
		}
	}
</style>
