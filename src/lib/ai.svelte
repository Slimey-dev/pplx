<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import rust from 'highlight.js/lib/languages/rust';
	import typescript from 'highlight.js/lib/languages/typescript';
	import 'highlight.js/styles/github.css';
	import rehypeHighlight from 'rehype-highlight';
	import type { Plugin } from 'svelte-exmarkdown';
	import Markdown from 'svelte-exmarkdown';

	let inputText = '';
	let apiResponses: { message: string; isUser: boolean }[] = [];

	async function callAiRequest(): Promise<void> {
		apiResponses = [...apiResponses, { message: inputText, isUser: true }];
		const result = await invoke('ai_request', { input: inputText });
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
	}

	const plugins: Plugin[] = [
		{
			rehypePlugin: [rehypeHighlight, { ignoreMissing: true, languages: { typescript, rust } }]
		}
	];
</script>

<div class="message-container">
	{#each apiResponses as response (response.message)}
		<div
			class:is-user={response.isUser}
			class:user-message={response.isUser}
			class:ai-message={!response.isUser}
		>
			<Markdown {plugins} md={response.message} />
		</div>
	{/each}
</div>

<input bind:value={inputText} placeholder="Enter your input here" />

<button on:click={callAiRequest}> Call AI Request </button>

<style>
	.message-container {
		max-height: 100%; /* adjust as needed */
		overflow-y: auto;
	}

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
