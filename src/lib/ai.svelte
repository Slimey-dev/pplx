<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import rust from 'highlight.js/lib/languages/rust';
	import typescript from 'highlight.js/lib/languages/typescript';
	import 'highlight.js/styles/github.css';
	import rehypeHighlight from 'rehype-highlight';
	import type { Plugin } from 'svelte-exmarkdown';
	import Markdown from 'svelte-exmarkdown';
	import {
		Button,
		Dropdown,
		DropdownItem,
		DropdownMenu,
		DropdownToggle,
		Styles
	} from 'sveltestrap';
	import Switch from '../component/Switch.svelte';

	let inputText = '';
	let apiResponses: { message: string; isUser: boolean }[] = [];
	let exitOnClose = false;
	let modelSelection = '';
	let showSettings = false;
	let modelList: string[] = [
		'pplx-70b-chat',
		'pplx-7b-online',
		'pplx-70b-online',
		'mistral-7b-instruct',
		'codellama-34b-instruct',
		'llama-2-70b-chat'
	];
	let selectedModel = 'pplx-7b-chat';

	async function callAiRequest(): Promise<void> {
		const currentInput = inputText;
		inputText = '';
		apiResponses = [...apiResponses, { message: currentInput, isUser: true }];
		const result = await invoke('async_command', { input: currentInput });
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
	}

	const plugins: Plugin[] = [
		{
			rehypePlugin: [rehypeHighlight, { ignoreMissing: true, languages: { typescript, rust } }]
		}
	];
</script>

<Styles />

<div class="m-0 p-0 h-full w-full relative">
	<button class="absolute -top-6 right-4 z-20" on:click={() => (showSettings = !showSettings)}
		><img src="/gear-six.svg" alt="settings" /></button
	>
	{#if showSettings}
		<div class="absolute top-0 left-0 w-full h-full z-10 bg-white">
			<div class="flex flex-col justify-center items-center pt-48">
				<Switch bind:checked={exitOnClose}></Switch>
				<Dropdown>
					<DropdownToggle caret>Menu</DropdownToggle>
					<DropdownMenu>
						<DropdownItem
							on:click={() => {
								selectedModel = 'pplx-7b-chat';
								console.log(selectedModel);
							}}>pplx-7b-chat</DropdownItem
						>
						{#each modelList as model}
							<DropdownItem divider />
							<DropdownItem
								on:click={() => {
									selectedModel = model;
									console.log(selectedModel);
								}}>{model}</DropdownItem
							>
						{/each}
					</DropdownMenu>
				</Dropdown>
			</div>
		</div>
	{/if}

	<div class="w-full flex flex-col mt-10 overflow-y-auto mb-20">
		{#each apiResponses as response (response.message)}
			<div class="px-20 w-full flex {response.isUser ? 'justify-end' : 'justify-start'} p-2">
				<div
					class:is-user={response.isUser}
					class={response.isUser
						? 'text-right bg-blue-500 text-white p-4 rounded-lg'
						: 'text-left bg-gray-300 text-black p-4 rounded-lg'}
				>
					<Markdown {plugins} md={response.message} />
				</div>
			</div>
		{/each}
	</div>

	{#if !showSettings}
		<div class="fixed bottom-0 left-0 w-full h-1/12 bg-white shadow-lg shadow-black">
			<form
				class="flex flex-row justify-center items-center m-0 p-0 h-24 w-full"
				on:submit|preventDefault={callAiRequest}
			>
				<input
					class="w-[80%] p-2 border border-gray-300 rounded flex justify-center mx-auto my-5"
					bind:value={inputText}
					placeholder="Enter your input here"
				/>

				<button
					class="mx-auto bg-blue-500 text-white p-2 rounded flex z-0 items-center justify-center"
					type="submit"
				>
					Call AI Request
				</button>
			</form>
		</div>
	{/if}
</div>
