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
		const result = await invoke('async_command', { input: inputText });
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
	}

	const plugins: Plugin[] = [
		{
			rehypePlugin: [rehypeHighlight, { ignoreMissing: true, languages: { typescript, rust } }]
		}
	];
</script>

<div class="h-full">
	<div class="w-full mt-10 overflow-y-auto mb-20">
		{#each apiResponses as response (response.message)}
			<div class="mx-20 flex justify-{response.isUser ? 'end' : 'start'} m-2">
				<div
					class:is-user={response.isUser}
					class={response.isUser
						? 'text-right bg-blue-500 text-white p-4 max-w-[90%] rounded-lg'
						: 'text-left  bg-gray-300 text-black p-4 max-w-[90%] rounded-lg'}
				>
					<Markdown {plugins} md={response.message} />
				</div>
			</div>
		{/each}
	</div>
	<div class="fixed bottom-0 left-0 w-full bg-white shadow-lg shadow-black">
		<div class="flex flex-row justify-center items-center">
			<input
				class="w-[80%] p-2 border border-gray-300 rounded flex justify-center mx-auto my-5"
				bind:value={inputText}
				placeholder="Enter your input here"
			/>

			<button
				class="mx-auto bg-blue-500 text-white p-2 rounded flex items-center justify-center"
				on:click={callAiRequest}
			>
				Call AI Request
			</button>
		</div>
	</div>
</div>
