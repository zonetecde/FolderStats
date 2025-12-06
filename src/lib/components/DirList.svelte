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

<div class="flex flex-col gap-3">
	<div
		class="flex items-center gap-3 rounded-2xl border border-[var(--border)] bg-[var(--panel-strong)] px-4 py-3 shadow-[0_18px_55px_rgba(0,0,0,0.3)]"
	>
		<button
			class="h-11 w-11 rounded-xl border border-[var(--border)] bg-white/5 flex items-center justify-center hover:border-[var(--accent)]"
			on:click={() => {
				if ($isLoading) return;

				if ($pathToCurrentFolder.length > 1) {
					const updatedPath = $pathToCurrentFolder.slice(0, $pathToCurrentFolder.length - 1);
					pathToCurrentFolder.set(updatedPath);
					currentFolder.set(updatedPath[updatedPath.length - 1]);
				} else {
					pathToCurrentFolder.set([]);
					selectedFolderPath.set('');
				}
			}}
		>
			<img src="back.svg" alt="back" class="h-6 w-6" />
		</button>

		<div class="flex flex-wrap items-center gap-2 text-sm text-[var(--muted)]">
			{#each $pathToCurrentFolder as folder, i}
				<button
					class="px-2.5 py-1 rounded-lg bg-white/5 hover:text-[var(--accent)]"
					on:click={() => {
						if ($isLoading) return;
						const updatedPath = $pathToCurrentFolder.slice(0, i + 1);
						pathToCurrentFolder.set(updatedPath);
						currentFolder.set(folder);
					}}
				>
					{folder.name}
				</button>
				{#if i < $pathToCurrentFolder.length - 1}
					<span class="text-[var(--border)]">/</span>
				{/if}
			{/each}

			<span
				class="ml-1 text-xs px-2 py-1 rounded-full border border-[var(--border)] bg-white/5 text-[var(--muted)]"
			>
				{bytesToFormattedString($currentFolder.size)}
			</span>
		</div>

		<button
			class="ml-auto inline-flex items-center gap-2 px-3 py-2 rounded-xl border border-[var(--border)] text-sm text-[var(--muted)] hover:border-[var(--accent)]"
			on:click={async () => {
				for (const prop in savedSize) {
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
				class="size-5"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
				/>
			</svg>
			Rafraîchir
		</button>
	</div>

	<div
		class="rounded-2xl border border-[var(--border)] bg-[var(--panel)] overflow-hidden shadow-[0_20px_60px_rgba(0,0,0,0.3)] divide-y divide-[var(--border)]"
	>
		{#if $currentSubFolders.length === 0}
			<p class="px-4 py-6 text-center text-[var(--muted)] text-sm">Aucun sous-dossier détecté.</p>
		{:else}
			{#each $currentSubFolders as subfolder, i}
				<FolderSize {subfolder} {i} />
			{/each}
		{/if}
	</div>
</div>
