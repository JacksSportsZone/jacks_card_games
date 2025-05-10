<script lang="ts">
	let gameCode = '';
	let ws: WebSocket | null = null;
	let isCreating = false;
	let error = '';
	let connected = false;

	async function createGame() {
		isCreating = true;
		error = '';

		try {
			const res = await fetch('http://localhost:8080/games/create', {
				method: 'POST'
			});
			const data = await res.json();
			gameCode = data.code;

			startWebSocket(gameCode);
		} catch (e) {
			error = 'Failed to create game.';
		} finally {
			isCreating = false;
		}
	}

	function joinGame() {
		if (!gameCode) {
			error = 'Please enter a game code.';
			return;
		}
		error = '';
		startWebSocket(gameCode);
	}

	function startWebSocket(code: string) {
		ws = new WebSocket(`ws://localhost:8080/ws/game/${code}`);

		ws.onopen = () => {
			connected = true;
			console.log('Connected to game session:', code);
		};

		ws.onmessage = (event) => {
			const msg = JSON.parse(event.data);
			console.log('Message from server:', msg);
			// Update game state here
		};

		ws.onerror = () => {
			error = 'WebSocket error.';
		};

		ws.onclose = () => {
			connected = false;
			console.log('WebSocket connection closed.');
		};
	}
</script>

<main>
	<h1>Gin Rummy</h1>

	{#if !connected}
		<div>
			<button on:click={createGame} disabled={isCreating}>Create Game</button>
			<p>or</p>
			<input bind:value={gameCode} placeholder="Enter game code" />
			<button on:click={joinGame}>Join Game</button>
		</div>
	{:else}
		<p>Connected to game: <strong>{gameCode}</strong></p>
		<!-- You can route to the main game screen or display player hand here -->
	{/if}

	{#if error}
		<p style="color: red">{error}</p>
	{/if}
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem;
		font-family: sans-serif;
	}

	input {
		margin: 0.5rem;
		padding: 0.5rem;
	}
</style>
