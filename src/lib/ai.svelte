<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let inputText = '';
	let apiResponses: { message: string; isUser: boolean }[] = [];

	async function callAiRequest(): Promise<void> {
		apiResponses = [...apiResponses, { message: inputText, isUser: true }];
		const result = await invoke('ai_request', { input: inputText });
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
	}
</script>

{#each apiResponses as response (response.message)}
	<pre
		class:is-user={response.isUser}
		class:user-message={response.isUser}
		class:ai-message={!response.isUser}>{response.message}</pre>
{/each}

<input bind:value={inputText} placeholder="Enter your input here" />

<button on:click={callAiRequest}> Call AI Request </button>

<style>
	.user-message {
		text-align: right;
		background-color: #d1e7dd; /* light green */
		color: black;
		margin: 10px;
		padding: 10px;
		border-radius: 10px;
	}

	.ai-message {
		text-align: left;
		background-color: #d3d3d3; /* light gray */
		color: black;
		margin: 10px;
		padding: 10px;
		border-radius: 10px;
	}
</style>
