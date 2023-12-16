<script lang="ts">
	import { CaretDown, ClipboardText } from '@steeze-ui/phosphor-icons';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { invoke } from '@tauri-apps/api/tauri';
	import python from 'highlight.js/lib/languages/python';
	import rust from 'highlight.js/lib/languages/rust';
	import typescript from 'highlight.js/lib/languages/typescript';
	import 'highlight.js/styles/github.css';
	import rehypeHighlight from 'rehype-highlight';
	import { onMount } from 'svelte';
	import type { Plugin } from 'svelte-exmarkdown';
	import Markdown from 'svelte-exmarkdown';
	import { Dropdown, DropdownItem, DropdownMenu, DropdownToggle, Styles } from 'sveltestrap';

	let inputText = '';
	let apiResponses: { message: string; isUser: boolean }[] = [];
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
	let deleteChatHistory = false;
	let exitOnClose = false;
	let apiKey = '';

	interface Config {
		model: string;
		prevent_exit: boolean;
		api_key: string;
	}

	async function callAiRequest(): Promise<void> {
		const currentInput = inputText;
		inputText = '';
		apiResponses = [...apiResponses, { message: currentInput, isUser: true }];
		const result = await invoke('async_command', {
			selectedModel: selectedModel,
			input: currentInput,
			deleteChatHistory: deleteChatHistory,
			apiKey: apiKey
		});
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
		deleteChatHistory = false;
		highlightCode();
	}

	async function configLoader(): Promise<void> {
		const loadConfig = await invoke('async_config_loader', {
			model: selectedModel,
			preventExit: exitOnClose,
			apiKey: apiKey
		});
		const loadedConfig = JSON.parse(loadConfig as string) as Config;
		selectedModel = loadedConfig.model;
		exitOnClose = loadedConfig.prevent_exit;
		apiKey = loadedConfig.api_key;
	}
	configLoader();

	async function configSaver(): Promise<void> {
		await invoke('async_config_saver', {
			model: selectedModel,
			preventExit: exitOnClose,
			apiKey: apiKey
		});
	}

	function deleteHistory(): void {
		apiResponses = [];
		deleteChatHistory = true;
	}

	const plugins: Plugin[] = [
		{
			rehypePlugin: [
				rehypeHighlight,
				{ ignoreMissing: true, languages: { typescript, rust, python } }
			]
		}
	];

	$: {
		invoke('set_prevent_exit', { setPreventExit: exitOnClose })
			.then(() => console.log('set_prevent_exit command invoked successfully'))
			.catch((error) => console.error('Failed to invoke set_prevent_exit command:', error));
		console.log('exitOnClose', exitOnClose);
	}

	let codeBlocks;

	async function highlightCode(): Promise<void> {
		console.log('Waiting for a second before running highlightCode');
		await new Promise((resolve) => setTimeout(resolve, 1000));
		console.log('highlightCode function called');
		let preBlocks = document.querySelectorAll('pre');
		console.log(`Found ${preBlocks.length} pre blocks`);
		preBlocks.forEach((block, index) => {
			console.log(`Processing block ${index + 1}`);
			let button = document.createElement('button');
			let icon = new Icon({
				target: button,
				props: {
					src: ClipboardText,
					size: '24'
				}
			});
			button.classList.add('absolute', 'top-7', 'right-0', 'p-2', 'text-white');
			button.addEventListener('click', () => {
				let codeBlock = block.querySelector('code');
				if (codeBlock && codeBlock.textContent) {
					console.log('Copy button clicked, copying text to clipboard');
					navigator.clipboard.writeText(codeBlock.textContent);
				} else {
					console.log('Copy button clicked, but block has no text content');
				}
			});
			console.log('Inserting copy button into DOM');
			const wrapper = document.createElement('div');
			wrapper.style.position = 'relative';
			if (block.parentNode) {
				block.parentNode.replaceChild(wrapper, block);
				wrapper.appendChild(block);
				wrapper.appendChild(button);
			} else {
				console.log('Block has no parent node, cannot insert copy button');
			}
		});
	}
</script>

<Styles />

<div>
	<button class="absolute top-5 right-5" on:click={() => (showSettings = !showSettings)}
		><img src="/gear-six.svg" alt="settings" /></button
	>
	<button class="absolute top-5 left-5" on:click={() => deleteHistory()}
		><img src="/trash.svg" alt="trash" /></button
	>
	{#if showSettings}
		<div class="bg-white">
			<div class="">
				<div class="pt-20 pl-4 flex flex-col gap-3">
					<div class="flex flex-row gap-3">
						<input type="text" bind:value={apiKey} class="border-2 border-gray-500 p-2 w-[29rem]" />
					</div>
					<div class="">
						<input type="checkbox" bind:checked={exitOnClose} class="" />
						<span>Exit on Close</span>
					</div>
					<Dropdown>
						<DropdownToggle class="">
							<div class="flex flex-row">
								<div class="">{selectedModel}</div>
								<Icon src={CaretDown} size="24" />
							</div>
						</DropdownToggle>
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
				<button
					class="fixed bottom-0 border-2 bg-gray-500 border-gray-500 py-2 text-white w-full"
					on:click={() => configSaver()}>Save Config</button
				>
			</div>
		</div>
	{/if}

	{#if !showSettings}
		<div class="pt-20 px-14 max-w-full pb-28">
			{#each apiResponses as response (response.message)}
				<div class="px-20 w-full flex {response.isUser ? 'justify-end' : 'justify-start'} p-2">
					<div
						class:is-user={response.isUser}
						class={response.isUser
							? 'text-right flex flex-col bg-blue-500 text-white pt-4 pb-4 px-4 items-center rounded-lg prose'
							: 'text-left flex flex-col bg-gray-300 text-black pt-2 pb-2 px-4 rounded-lg prose'}
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
	{/if}
</div>
