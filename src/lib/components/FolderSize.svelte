<script lang="ts">
	import { bytesToFormattedString, deleteFromDisk, openFolderInExplorer } from '$lib';
	import type { Folder } from '$lib/Models/Folder';
	import {
		currentFolder,
		currentSubFolders,
		isLoading,
		pathToCurrentFolder,
		selectedFolder,
		leftPartSize
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
	class={'w-full flex flex-row text-xl min-h-8 border-2 border-r-0 border-[#1a0d13] bg-[#5c30583b] duration-100 overflow-x-hidden ' +
		($currentSubFolders.length - 1 === i ? 'border-b-2 ' : 'border-b-0 ')}
>
	<button
		class="text-sm pt-1.5 px-1 w-2/12 min-w-[150px] hover:overflow-x-auto hover:underline text-left"
		style={`width: ${$leftPartSize}%`}
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
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		<section
			aria-roledescription="separator"
			aria-orientation="vertical"
			role="separator"
			class="absolute top-0 h-full w-0.5 bg-[#1a0d13] z-10 cursor-e-resize"
			on:mousedown={(e) => {
				let initialWidth = e.clientX;
				let initialSize = $leftPartSize;

				//@ts-ignore
				const onMouseMove = (e) => {
					let newWidth = initialSize + ((e.clientX - initialWidth) / window.innerWidth) * 100;
					if (newWidth < 0) newWidth = 0;
					if (newWidth > 100) newWidth = 100;

					leftPartSize.set(newWidth);
				};

				const onMouseUp = () => {
					window.removeEventListener('mousemove', onMouseMove);
					window.removeEventListener('mouseup', onMouseUp);
				};

				window.addEventListener('mousemove', onMouseMove);
				window.addEventListener('mouseup', onMouseUp);
			}}
		></section>

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
