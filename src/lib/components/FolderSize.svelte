<script lang="ts">
	import { bytesToFormattedString, deleteFromDisk, openFolderInExplorer } from '$lib';
	import type { Folder } from '$lib/Models/Folder';
	import {
		currentFolder,
		currentSubFolders,
		isLoading,
		pathToCurrentFolder,
		selectedFolder
	} from '$lib/stores/GlobalStore';
	import ContextMenu, { Item, Divider, Settings } from 'svelte-contextmenu';

	export let subfolder: Folder;
	export let i: number;
	let truncatText = true;

	let myMenu: ContextMenu;

	$: pourcentComparedWithCurrentFolder = (subfolder.size / $currentFolder.size) * 100;
	$: pourcentComparedWithSelectedFolder = (subfolder.size / $selectedFolder.size) * 100;
</script>

<div
	class={'w-full flex flex-row text-xl min-h-8 border-2 divide-x-2 divide-[#1a0d13] border-[#1a0d13] bg-[#5c30583b] duration-100 overflow-x-hidden ' +
		($currentSubFolders.length - 1 === i ? 'border-b-2 ' : 'border-b-0 ')}
>
	<button
		class="text-sm pt-1.5 px-1 w-2/12 min-w-[150px] hover:overflow-x-auto hover:underline text-left"
		on:click={(e) => {
			if ($isLoading) return;

			pathToCurrentFolder.set([...$pathToCurrentFolder, subfolder]);
			currentFolder.set(subfolder);
		}}
		on:contextmenu={(e) => {
			myMenu.createHandler();
			myMenu.show(e);
		}}
		on:mouseenter={() => {
			truncatText = false;
		}}
	>
		<p class="text-nowrap hover:text-clip hover:overflow-auto" class:truncate={truncatText}>
			{subfolder.name}
		</p>
	</button>

	<div class="w-full bg-[#a88bac] relative">
		<div
			style="width: {pourcentComparedWithCurrentFolder}%"
			class="bg-[#6d4569] h-full absolute left-0 top-0"
		></div>

		<div
			style="width: {pourcentComparedWithSelectedFolder}%"
			class="bg-[#473046] h-full absolute left-0 top-0"
		></div>

		<p class="text-sm pt-1.5 px-1 absolute right-1">
			<span class="text-black">
				{bytesToFormattedString(subfolder.size)} ({pourcentComparedWithSelectedFolder.toFixed(
					2
				)}%)</span
			>
			{#if $currentFolder.fullPath !== $selectedFolder.fullPath}
				<span class="text-[#473046]">({pourcentComparedWithCurrentFolder.toFixed(2)}%)</span>
			{/if}
		</p>
	</div>
</div>

<ContextMenu bind:this={myMenu}>
	<Item
		on:click={() => {
			openFolderInExplorer(subfolder.fullPath);
		}}>Open folder in explorer</Item
	>
	<Item
		on:click={async () => {
			if ($currentFolder.fullPath === $selectedFolder.fullPath)
				$selectedFolder.size -= subfolder.size;
			else $currentFolder.size -= subfolder.size;

			await deleteFromDisk(subfolder.fullPath);

			// Force a refresh of the current folder
			currentFolder.set($currentFolder);
		}}>Delete from disk</Item
	>
</ContextMenu>
