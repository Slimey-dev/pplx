<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let inputText = '';
	let apiResponse: string = '';

	async function callAiRequest(): Promise<void> {
		const result = await invoke('ai_request', { input: inputText });
		apiResponse = result as string;
	}
</script>

<input bind:value={inputText} placeholder="Enter your input here" />

<button on:click={callAiRequest}> Call AI Request </button>

<pre>{apiResponse}</pre>
