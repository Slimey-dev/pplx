<script lang="ts">
	import { CaretDown } from '@steeze-ui/phosphor-icons';
	import { Icon } from '@steeze-ui/svelte-icon';
	import { invoke } from '@tauri-apps/api/tauri';
	import python from 'highlight.js/lib/languages/python';
	import rust from 'highlight.js/lib/languages/rust';
	import typescript from 'highlight.js/lib/languages/typescript';
	import 'highlight.js/styles/github.css';
	import rehypeHighlight from 'rehype-highlight';
	import type { Plugin } from 'svelte-exmarkdown';
	import Markdown from 'svelte-exmarkdown';
	import { Dropdown, DropdownItem, DropdownMenu, DropdownToggle, Styles } from 'sveltestrap';
	import Switch from '../component/Switch.svelte';

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

	interface Config {
		model: string;
		preventExit: boolean;
	}

	$: {
		invoke('set_prevent_exit', { value: exitOnClose })
			.then(() => console.log('set_prevent_exit command invoked successfully'))
			.catch((error) => console.error('Failed to invoke set_prevent_exit command:', error));
		console.log('exitOnClose', exitOnClose);
	}

	async function callAiRequest(): Promise<void> {
		const currentInput = inputText;
		inputText = '';
		apiResponses = [...apiResponses, { message: currentInput, isUser: true }];
		const result = await invoke('async_command', {
			selectedModel: selectedModel,
			input: currentInput,
			deleteChatHistory: deleteChatHistory
		});
		apiResponses = [...apiResponses, { message: result as string, isUser: false }];
		deleteChatHistory = false;
	}

	async function configLoader(): Promise<void> {
		const loadConfig = await invoke('async_config_loader', {
			model: selectedModel,
			preventExit: exitOnClose
		});
		const loadedConfig = JSON.parse(loadConfig as string) as Config;
		selectedModel = loadedConfig.model;
		exitOnClose = loadedConfig.preventExit;
		console.log(loadConfig);
	}
	configLoader();

	async function configSaver(): Promise<void> {
		await invoke('async_config_saver', {
			model: selectedModel,
			preventExit: exitOnClose
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
</script>

<Styles />

<div class="m-0 p-0 h-full w-full relative">
	<button class="fixed top-4 right-4 z-20" on:click={() => (showSettings = !showSettings)}
		><img src="/gear-six.svg" alt="settings" /></button
	>
	<button class="fixed top-4 left-4 z-20" on:click={() => deleteHistory()}
		><img src="/trash.svg" alt="trash" /></button
	>
	{#if showSettings}
		<div class="absolute top-0 left-0 w-full h-full z-10 bg-white">
			<div class="flex flex-row mx-auto justify-center gap-20 items-center pt-20">
				<div class="flex flex-col justify-center items-center gap-3">
					<span>Exit on Close</span>
					<Switch bind:checked={exitOnClose}></Switch>
				</div>
				<Dropdown>
					<DropdownToggle class="w-60 flex flex-row items-center justify-center">
						<div class="w-full m-0 flex flex-row items-center justify-between">
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
			<button on:click={() => configSaver()}>Save Config</button>
		</div>
	{/if}

	<div class="w-full flex flex-col mt-10 px-14 overflow-y-auto mb-20">
		{#each apiResponses as response (response.message)}
			<div class="px-20 w-full flex {response.isUser ? 'justify-end' : 'justify-start'} p-2">
				<div
					class:is-user={response.isUser}
					class={response.isUser
						? 'text-right flex flex-col bg-blue-500 text-white pt-4 pb-2 px-4 items-center rounded-lg'
						: 'text-left flex flex-col bg-gray-300 text-black pt-4 pb-2 px-4 rounded-lg'}
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
