<script lang="ts">
	import { goto } from '$app/navigation';
	import ShieldIcon from '$components/ui/icons/ShieldIcon.svelte';
	import IconEye from '$components/ui/icons/EyeIcon.svelte';
	import IconEyeOff from '$components/ui/icons/EyeOffIcon.svelte';

	let username = '';
	let email = '';
	let password = '';
	let confirm = '';
	let loading = false;
	let error = '';

	let showPassword = false;
	let showConfirm = false;

	function validate(): string {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

		if (!username.trim()) return 'Username is required.';
		if (username.length < 3) return 'Username must be at least 3 characters.';
		if (!email.trim()) return 'Email is required.';
		if (!emailRegex.test(email)) return 'Invalid email address.';
		if (!password) return 'Password is required.';
		if (password.length < 8) return 'Password must be at least 8 characters.';
		if (password !== confirm) return 'Passwords do not match.';
		return '';
	}

	async function handleSubmit() {
		error = validate();
		if (error) return;
		loading = true;

		try {
			const res = await fetch('/api/auth/setup', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ username, password, email })
			});
			const data = await res.json();
			if (!res.ok) {
				error = data.error ?? 'Setup failed.';
				loading = false;
				return;
			}

			goto('/login');
		} catch {
			error = 'Could not reach the server.';
			loading = false;
		}
	}

	$: strength = (() => {
		if (!password) return 0;
		let s = 0;
		if (password.length >= 8) s++;
		if (/[A-Z]/.test(password)) s++;
		if (/[0-9]/.test(password)) s++;
		if (/[^A-Za-z0-9]/.test(password)) s++;
		return s;
	})();

	const strengthLabel = ['Weak', 'Weak', 'Fair', 'Good', 'Strong'];
	const strengthClass = ['weak', 'weak', 'fair', 'good', 'strong'];
</script>

<div class="login-page">
	<div class="login-center">
		<div class="login-brand">
			<div class="login-icon">
				<ShieldIcon size={36} color="#ff0000" />
			</div>
			<h1>Initial Setup</h1>
			<p>Create your administrator account to get started</p>
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
						placeholder="Choose an admin username"
						bind:value={username}
						disabled={loading}
                        on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
					/>
				</div>

				<div class="field">
					<label for="email">Email</label>
					<input
						id="email"
						type="email"
						placeholder="Enter your email"
						bind:value={email}
						disabled={loading}
                        on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
					/>
				</div>

				<div class="field">
					<label for="password">Password</label>
					<div class="input-wrap">
						<input
							id="password"
							type={showPassword ? 'text' : 'password'}
							placeholder="Choose a strong password"
							bind:value={password}
							disabled={loading}
                            on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
						/>
						<button
							type="button"
							class="eye-btn"
							on:click={() => (showPassword = !showPassword)}
							aria-label={showPassword ? 'Hide password' : 'Show password'}
						>
							{#if showPassword}
								<IconEye size={15} />
							{:else}
								<IconEyeOff size={15} />
							{/if}
						</button>
					</div>
					<div class="strength-bar">
						{#each [1, 2, 3, 4] as i}
							<div
								class={`strength-segment ${
									i <= strength
										? `strength-segment--active strength-segment--${strengthClass[strength]}`
										: ''
								}`}
							></div>
						{/each}
						<span class={`strength-label strength-label--${strengthClass[strength]}`}>
							{strengthLabel[strength]}
						</span>
					</div>
				</div>

				<div class="field">
					<label for="confirm">Confirm Password</label>
					<div class="input-wrap">
						<input
							id="confirm"
							type={showConfirm ? 'text' : 'password'}
							placeholder="Repeat your password"
							bind:value={confirm}
							disabled={loading}
							on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
						/>
						<button
							type="button"
							class="eye-btn"
							on:click={() => (showConfirm = !showConfirm)}
							aria-label={showConfirm ? 'Hide password' : 'Show password'}
						>
							{#if showConfirm}
								<IconEye size={15} />
							{:else}
								<IconEyeOff size={15} />
							{/if}
						</button>
					</div>
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
						Create Admin Account
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

	.input-wrap {
		position: relative;
		display: flex;
		align-items: center;

		input {
			padding-right: 40px;
		}
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

	.eye-btn {
		position: absolute;
		right: 10px;
		background: none;
		border: none;
		color: $text-muted;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2px;
		border-radius: 4px;
		transition: color $transition;

		&:hover {
			color: $text-primary;
		}
	}

	.strength-bar {
		display: flex;
		align-items: center;
		gap: 4px;
		margin-top: 2px;
	}

	.strength-segment {
		flex: 1;
		height: 3px;
		border-radius: 2px;
		background: $border;
		transition: background $transition;

		&--active {
			&.strength-segment--weak {
				background: #f87171;
			}
			&.strength-segment--fair {
				background: #f5a623;
			}
			&.strength-segment--good {
				background: #60a5fa;
			}
			&.strength-segment--strong {
				background: #34d87a;
			}
		}
	}

	.strength-label {
		font-size: 11px;
		margin-left: 4px;
		min-width: 36px;
		text-align: right;

		&--weak {
			color: #f87171;
		}
		&--fair {
			color: #f5a623;
		}
		&--good {
			color: #60a5fa;
		}
		&--strong {
			color: #34d87a;
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
