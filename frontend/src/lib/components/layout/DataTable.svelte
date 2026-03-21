<script lang="ts">
	import type { Column, Row, BadgeVariant } from '$lib/types';

	export let columns: Column[] = [];
	export let rows: Row[] = [];
	export let statusKey: string = 'status';
	export let statusColors: Record<string, BadgeVariant> = {};

	function badgeVariant(status: unknown): BadgeVariant {
		if (typeof status === 'string' && status in statusColors) {
			return statusColors[status];
		}
		return 'gray';
	}

	function stringify(value: unknown): string {
		if (value === null || value === undefined) return '—';
		return String(value);
	}
</script>

<div class="dt-wrapper">
	<table class="dt-table">
		<thead>
			<tr>
				{#each columns as col (col.key)}
					<th>
						{col.label}
					</th>
				{/each}
				{#if $$slots.actions}
					<th class="dt-actions-head" aria-label="Actions"></th>
				{/if}
			</tr>
		</thead>

		<tbody>
			{#each rows as row, i (i)}
				<tr class="dt-row">
					{#each columns as col (col.key)}
						<td>
							{#if col.key === statusKey}
								<span class="dt-badge dt-badge--{badgeVariant(row[col.key])}">
									{stringify(row[col.key])}
								</span>
							{:else if $$slots.cell}
								<slot name="cell" {row} {col} value={row[col.key]} />
							{:else}
								<span class="dt-cell-text">{stringify(row[col.key])}</span>
							{/if}
						</td>
					{/each}

					{#if $$slots.actions}
						<td class="dt-actions-cell">
							<slot name="actions" {row} />
						</td>
					{/if}
				</tr>
			{/each}

			{#if rows.length === 0}
				<tr>
					<td colspan={columns.length + ($$slots.actions ? 1 : 0)} class="dt-empty">
						No data available
					</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>

<style lang="scss">
    @use '$styles/variables' as *;
	$bg-surface: $bg-sidebar;
	$bg-row-hover: #22222a;
	$border-color: #2e2e38;
    $text-dim: $text-primary;
	$text-primary: #d4d4dc;
	$text-muted: #6b6b7e;
	$font-ui: 'Inter', 'Segoe UI', system-ui, sans-serif;
	$radius: 8px;
	$transition: 140ms ease;

	$badge-green-bg: #0d2b1a;
	$badge-green-text: #34d87a;
	$badge-green-ring: #1a5c35;

	$badge-amber-bg: #2b1f04;
	$badge-amber-text: #f5a623;
	$badge-amber-ring: #7a4f08;

	$badge-gray-bg: #1e1e26;
	$badge-gray-text: #6b6b7e;
	$badge-gray-ring: #2e2e3e;

	.dt-wrapper {
		width: 100%;
		overflow-x: auto;
		border-radius: $radius;
		background: $bg-surface;
		border: 1px solid $border-color;
		scrollbar-width: thin;
		scrollbar-color: $border-color transparent;

		&::-webkit-scrollbar {
			height: 6px;
		}
		&::-webkit-scrollbar-track {
			background: transparent;
		}
		&::-webkit-scrollbar-thumb {
			background: $border-color;
			border-radius: 3px;
		}
	}

	.dt-table {
		width: 100%;
		border-collapse: collapse;
		font-family: $font-ui;
		font-size: 13px;
		table-layout: auto;
	}

	thead th {
		padding: 10px 16px;
		text-align: left;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: $text-dim;
		border-bottom: 1px solid $border-color;
		background: rgba(0, 0, 0, 0.18);
		white-space: nowrap;
		user-select: none;

		&.dt-actions-head {
			width: 40px;
		}
	}

	tbody {
		.dt-row {
			border-bottom: 1px solid $border-color;
			transition: background $transition;

			&:last-child {
				border-bottom: none;
			}
			&:hover {
				background: $bg-row-hover;
			}
		}

		td {
			padding: 12px 16px;
			color: $text-primary;
			vertical-align: middle;
			white-space: nowrap;
		}

		.dt-actions-cell {
			width: 40px;
			text-align: right;
			padding-right: 12px;
		}

		.dt-empty {
			text-align: center;
			color: $text-muted;
			padding: 36px 16px;
			font-size: 13px;
		}
	}

	.dt-cell-text {
		color: $text-primary;
		font-variant-numeric: tabular-nums;
	}

	.dt-badge {
		display: inline-flex;
		align-items: center;
		padding: 3px 10px;
		border-radius: 20px;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.04em;
		text-transform: lowercase;
		border: 1px solid transparent;
		transition: opacity $transition;

		&--green {
			background: $badge-green-bg;
			color: $badge-green-text;
			border-color: $badge-green-ring;
		}
		&--amber {
			background: $badge-amber-bg;
			color: $badge-amber-text;
			border-color: $badge-amber-ring;
		}
		&--gray {
			background: $badge-gray-bg;
			color: $badge-gray-text;
			border-color: $badge-gray-ring;
		}
	}
</style>
