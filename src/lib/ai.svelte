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
	<pre class:is-user={response.isUser}>{response.message}</pre>
{/each}

<input bind:value={inputText} placeholder="Enter your input here" />

<button on:click={callAiRequest}> Call AI Request </button>
