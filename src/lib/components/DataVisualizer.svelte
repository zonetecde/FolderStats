<script>
	import { bytesToFormattedString, getDirName, openFolderDialog } from '$lib';
	import {
		currentFolder,
		currentSubFolders,
		isLoading,
		selectedFolder,
		selectedFolderPath
	} from '$lib/stores/GlobalStore';
	import { Dir } from '@tauri-apps/api/fs';
	import { onMount } from 'svelte';
	import DirList from './DirList.svelte';
</script>

<div class="w-full h-full flex py-4 flex-col items-center">
	<h1 class="text-2xl">
		Directory : <button on:click={openFolderDialog}
			><u>{getDirName($selectedFolderPath ?? '')}</u></button
		>
	</h1>

	{#if $currentFolder && $currentSubFolders}
		<p class="text-xl mt-1">Size : {bytesToFormattedString($selectedFolder.size)}</p>

		<DirList />

		{#if !$isLoading}
			<div class="flex gap-x-10 mt-5">
				<div class="flex flex-row gap-x-2 items-center">
					<section class="w-20 h-10 bg-[#473046] border border-black" />
					<p class="text-base">File size compared <br />to selected folder</p>
				</div>
				<div class="flex flex-row gap-x-2 items-center">
					<section class="w-20 h-10 bg-[#6d4569] border border-black" />
					<p class="text-base">File size compared <br />to current folder</p>
				</div>
			</div>
		{/if}
	{/if}

	{#if $isLoading}
		<div class="flex items-center justify-center mt-3 bg-[#00000085] px-10 py-2 rounded-full">
			<svg
				class="animate-spin h-5 w-5 text-gray-500"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
			>
				<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
				></circle>
				<path
					class="opacity-75"
					fill="currentColor"
					d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l1.732-1A5.99 5.99 0 012 12H0c0 1.657.66 3.154 1.732 4.268l1.732-1zM12 20a8 8 0 100-16 8 8 0 000 16zm0-2a6 6 0 100-12 6 6 0 000 12z"
				></path>
			</svg>
			<p class="text-xl mt-1 ml-2">Loading...</p>
		</div>
	{/if}
</div>
