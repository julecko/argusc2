<script lang="ts">
	let downloadPath = 'C:\\Users\\admin\\Documents\\file.txt';
	let uploadFile: FileList | null = null;
	let downloadStatus = '';
	let uploadStatus = '';

	async function download() {
		if (!downloadPath.trim()) return;
		downloadStatus = 'Downloading...';
		await new Promise((r) => setTimeout(r, 1200));
		downloadStatus = 'Download started!';
		setTimeout(() => (downloadStatus = ''), 3000);
	}

	async function upload() {
		if (!uploadFile?.length) return;
		uploadStatus = 'Uploading...';
		await new Promise((r) => setTimeout(r, 1200));
		uploadStatus = 'Upload complete!';
		setTimeout(() => (uploadStatus = ''), 3000);
	}
</script>

<div class="tab-panel">
	<div class="panel-header">
		<div class="panel-title-group">
			<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
				<path
					d="M8 1v9M5 7l3 3 3-3M2 12v1a1 1 0 001 1h10a1 1 0 001-1v-1"
					stroke="currentColor"
					stroke-width="1.3"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			</svg>
			<span class="panel-title">File Transfer</span>
		</div>
	</div>

	<div class="transfer-grid">
		<!-- Download -->
		<div class="transfer-card">
			<div class="card-title-row">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<path
						d="M8 1v9M5 7l3 3 3-3"
						stroke="currentColor"
						stroke-width="1.3"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
					<path
						d="M2 12v1a1 1 0 001 1h10a1 1 0 001-1v-1"
						stroke="currentColor"
						stroke-width="1.3"
						stroke-linecap="round"
					/>
				</svg>
				<span>Download File from Device</span>
			</div>
			<p class="card-desc">Download a file from the remote device</p>

			<label class="field-label" for="dl-path">File Path on Device</label>
			<input
				id="dl-path"
				class="text-input"
				type="text"
				bind:value={downloadPath}
				placeholder="C:\Users\admin\Documents\file.txt"
			/>

			{#if downloadStatus}
				<p class="status">{downloadStatus}</p>
			{/if}

			<button class="action-btn action-btn--blue" on:click={download}>
				<svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
					<path
						d="M8 1v9M5 7l3 3 3-3M2 13h12"
						stroke="white"
						stroke-width="1.3"
						stroke-linecap="round"
						fill="none"
					/>
				</svg>
				Download File
			</button>
		</div>

		<!-- Upload -->
		<div class="transfer-card">
			<div class="card-title-row">
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<path
						d="M8 9V1M5 4l3-3 3 3"
						stroke="currentColor"
						stroke-width="1.3"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
					<path
						d="M2 12v1a1 1 0 001 1h10a1 1 0 001-1v-1"
						stroke="currentColor"
						stroke-width="1.3"
						stroke-linecap="round"
					/>
				</svg>
				<span>Upload File to Device</span>
			</div>
			<p class="card-desc">Upload a file to the remote device</p>

			<label class="field-label" for="ul-file">Select File</label>
			<input
				id="ul-file"
				class="file-input"
				type="file"
				on:change={(e) => (uploadFile = (e.target as HTMLInputElement).files)}
			/>

			{#if uploadStatus}
				<p class="status">{uploadStatus}</p>
			{/if}

			<button
				class="action-btn action-btn--purple"
				on:click={upload}
				disabled={!uploadFile?.length}
			>
				<svg width="14" height="14" viewBox="0 0 16 16" fill="none">
					<path
						d="M8 9V1M5 4l3-3 3 3M2 13h12"
						stroke="white"
						stroke-width="1.3"
						stroke-linecap="round"
					/>
				</svg>
				Upload File
			</button>
		</div>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.tab-panel {
		display: flex;
		flex-direction: column;
	}

	.panel-header {
		padding: 16px 20px 12px;
		border-bottom: 1px solid $border;
	}

	.panel-title-group {
		display: flex;
		align-items: center;
		gap: 8px;
		color: $text-muted;
	}

	.panel-title {
		font-size: 13px;
		font-weight: 600;
		color: $text-primary;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.transfer-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0;

		@media (max-width: 700px) {
			grid-template-columns: 1fr;
		}
	}

	.transfer-card {
		padding: 24px;
		display: flex;
		flex-direction: column;
		gap: 12px;
		border-right: 1px solid $border;

		&:last-child {
			border-right: none;
		}
	}

	.card-title-row {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 600;
		color: $text-primary;
	}

	.card-desc {
		font-size: 12px;
		color: $text-muted;
		margin: 0;
	}

	.field-label {
		font-size: 12px;
		color: $text-muted;
		font-weight: 500;
	}

	.text-input {
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid $border;
		border-radius: 8px;
		padding: 10px 14px;
		color: $text-primary;
		font-size: 13px;
		font-family: 'JetBrains Mono', monospace;
		outline: none;
		transition: border-color $transition;
		width: 100%;
		box-sizing: border-box;

		&:focus {
			border-color: rgba(231, 0, 11, 0.4);
		}
	}

	.file-input {
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid $border;
		border-radius: 8px;
		padding: 9px 14px;
		color: $text-muted;
		font-size: 12px;
		width: 100%;
		box-sizing: border-box;
		cursor: pointer;
	}

	.status {
		font-size: 12px;
		color: #22c55e;
		margin: 0;
		font-family: 'JetBrains Mono', monospace;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 11px 0;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all $transition;
		width: 100%;
		margin-top: 4px;

		&--blue {
			background: #2563eb;
			&:hover {
				background: #1d4ed8;
			}
		}

		&--purple {
			background: #7c3aed;
			&:hover {
				background: #6d28d9;
			}
			&:disabled {
				opacity: 0.4;
				cursor: not-allowed;
			}
		}
	}
</style>
