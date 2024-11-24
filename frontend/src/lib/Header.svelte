<script lang="ts">
	import { socketManager } from '$lib/socket';

	let connected = false;
	let token = '';

	socketManager.subscribeConnection((con: boolean) => (connected = con));

	const updateToken = (event: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		token = event.currentTarget.value;
	};
</script>

<header>
	<div class="logo">
		<h1>shf</h1>
	</div>
	<div class="status-container">
		<div class="status">
			<div class={connected ? 'status-indicator connected' : 'status-indicator disconnected'}></div>
			<span>{connected ? 'Connected' : 'Disconnected'}</span>
		</div>
		<div class="token-entry">
			<input type="text" placeholder="Enter Token" bind:value={token} on:input={updateToken} />
		</div>
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		border: 1px solid #ccc;
		padding: 0.5rem 1rem;
		font-family: Arial, sans-serif;
	}

	.logo h1 {
		margin: 0;
		font-size: 1rem;
	}

	.status-container {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.3rem 0.7rem;
		border: 1px solid #ccc;
		border-radius: 20px;
		font-size: 0.9rem;
		font-weight: bold;
	}

	.status-indicator {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.status-indicator.connected {
		background-color: #28a745;
	}

	.status-indicator.disconnected {
		background-color: #dc3545;
	}

	.token-entry input {
		padding: 0.3rem;
		border-radius: 3px;
		border: 1px solid #ccc;
		font-size: 0.8rem;
	}

	.token-entry input:focus {
		outline: none;
		border-color: #333;
		box-shadow: 0 0 3px rgba(0, 0, 0, 0.2);
	}
</style>
