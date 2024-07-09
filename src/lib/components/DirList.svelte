<script>
	import { bytesToFormattedString, getFolderSize, savedSize } from '$lib';
	import {
		currentFolder,
		currentSubFolders,
		isLoading,
		pathToCurrentFolder,
		selectedFolder,
		selectedFolderPath
	} from '$lib/stores/GlobalStore';
	import { get } from 'svelte/store';
	import FolderSize from './FolderSize.svelte';
</script>

<div class="flex w-10/12 min-h-10 max-h-10 h-10 mt-4 flex-col overflow-y-auto">
	<div class="h-full flex items-center flex-row border-[#1a0d13] border-2 bg-[#5c30583b]">
		<button
			class="h-full pt-1.5 mx-2"
			on:click={() => {
				if ($isLoading) return;

				if ($pathToCurrentFolder.length > 1) {
					pathToCurrentFolder.set($pathToCurrentFolder.splice(0, $pathToCurrentFolder.length - 1));
					currentFolder.set($pathToCurrentFolder[$pathToCurrentFolder.length - 1]);
				}
			}}
		>
			<img src="back.svg" alt="back" class="h-full w-full" />
		</button>
		<p class="truncate">
			{#each $pathToCurrentFolder as folder, i}
				<button
					class="hover:underline"
					on:click={() => {
						if ($isLoading) return;
						pathToCurrentFolder.set($pathToCurrentFolder.splice(0, i + 1));
						currentFolder.set(folder);
					}}>{folder.name}</button
				>{i < $pathToCurrentFolder.length - 1 ? ' / ' : ''}
			{/each}

			({bytesToFormattedString($currentFolder.size)})
		</p>

		<button
			class="ml-auto mr-2"
			on:click={async () => {
				// Refresh the current folder
				for (var prop in savedSize) {
					if (savedSize.hasOwnProperty(prop)) {
						delete savedSize[prop];
					}
				}

				let _selectedFolder = get(selectedFolder);
				_selectedFolder.size = await getFolderSize(_selectedFolder.fullPath);
				selectedFolder.set(_selectedFolder);

				currentFolder.set($currentFolder);
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-6"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
				/>
			</svg>
		</button>
	</div>
</div>

<div class="flex w-10/12 max-h-full min-h-fit mt-4 flex-col overflow-y-auto">
	{#each $currentSubFolders as subfolder, i}
		<FolderSize {subfolder} {i} />
	{/each}
</div>
