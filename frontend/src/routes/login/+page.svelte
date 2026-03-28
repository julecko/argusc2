<script lang="ts">
	import { goto } from '$app/navigation';
	import ShieldIcon from '$components/ui/icons/ShieldIcon.svelte';

	let username = '';
	let password = '';
	let loading = false;
	let error = '';

	async function handleSubmit() {
		error = '';
		loading = true;

		try {
			const res = await fetch('/api/auth/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ username, password }),
			});

			if (!res.ok) {
				const data = await res.json().catch(() => ({}));
				error = data.error ?? 'Login failed.';
				loading = false;
				return;
			}

			await goto('/admin/dashboard');
		} catch {
			error = 'Could not reach the server.';
		} finally {
			loading = false;
		}
	}
</script>

<div class="login-page">
	<div class="login-center">
		<div class="login-brand">
			<div class="login-icon">
				<ShieldIcon size={36} color="#ff0000" />
			</div>
			<h1>C2 Control Panel</h1>
			<p>Sign in to access the command center</p>
		</div>

		<div class="login-card">
			<div class="login-fields">
				{#if error}
					<div class="login-error">{error}</div>
				{/if}

				<div class="field">
					<label for="username">Username</label>
					<input
						id="username"
						type="text"
						placeholder="Enter your username"
						bind:value={username}
						disabled={loading}
						on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
					/>
				</div>

				<div class="field">
					<label for="password">Password</label>
					<input
						id="password"
						type="password"
						placeholder="Enter your password"
						bind:value={password}
						disabled={loading}
						on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
					/>
				</div>

				<button
					class="login-btn"
					class:login-btn--loading={loading}
					on:click={handleSubmit}
					disabled={loading}
				>
					{#if loading}
						<span class="spinner"></span>
					{:else}
						Sign In
					{/if}
				</button>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/variables' as *;

	.login-page {
		min-height: 100vh;
		background: $bg-main;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
	}

	.login-center {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 28px;
		width: 100%;
		max-width: 380px;
		padding: 24px;
		animation: fade-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) both;
	}

	@keyframes fade-up {
		from {
			opacity: 0;
			transform: translateY(16px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.login-brand {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		text-align: center;
	}

	.login-icon {
		width: 56px;
		height: 56px;
		background: rgba($accent, 0.15);
		border: 1px solid rgba($accent, 0.3);
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	h1 {
		color: $text-primary;
		font-size: 26px;
		font-weight: 700;
		margin: 0;
		letter-spacing: -0.02em;
	}

	p {
		color: $text-muted;
		font-size: 13px;
		margin: 0;
	}

	.login-card {
		width: 100%;
		background: $bg-sidebar;
		border: 1px solid $border;
		border-radius: $radius;
		overflow: hidden;
	}

	.login-fields {
		display: flex;
		flex-direction: column;
		gap: 16px;
		padding: 24px 24px 20px;
	}

	.login-error {
		background: rgba(#f87171, 0.1);
		border: 1px solid rgba(#f87171, 0.3);
		border-radius: 6px;
		padding: 10px 14px;
		font-size: 13px;
		color: #f87171;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	label {
		font-size: 13px;
		font-weight: 500;
		color: $text-primary;
	}

	input {
		background: $bg-card;
		border: 1px solid $border;
		border-radius: $radius;
		padding: 10px 14px;
		font-size: 14px;
		color: $text-primary;
		outline: none;
		transition: border-color $transition;
		width: 100%;
		box-sizing: border-box;

		&::placeholder {
			color: $text-muted;
		}

		&:focus {
			border-color: $accent;
		}

		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}

	.login-btn {
		width: 100%;
		padding: 11px;
		background: $accent;
		border: none;
		border-radius: $radius;
		color: white;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		transition:
			opacity $transition,
			transform $transition;
		margin-top: 4px;

		&:hover:not(:disabled) {
			opacity: 0.88;
		}
		&:active:not(:disabled) {
			transform: scale(0.98);
		}
		&:disabled {
			cursor: not-allowed;
			opacity: 0.7;
		}

		&--loading {
			pointer-events: none;
		}
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(white, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
