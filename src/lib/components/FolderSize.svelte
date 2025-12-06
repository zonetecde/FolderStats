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
	import ContextMenu, { Item } from 'svelte-contextmenu';

	export let subfolder: Folder;
	export let i: number;
	let truncatText = true;

	let myMenu: ContextMenu;

	$: pourcentComparedWithCurrentFolder = (subfolder.size / $currentFolder.size) * 100;
	$: pourcentComparedWithSelectedFolder = (subfolder.size / $selectedFolder.size) * 100;
</script>

<div class="relative flex flex-col md:flex-row w-full overflow-hidden bg-white/0 group">
	<button
		class="relative text-left px-4 py-3 md:py-4 flex items-center gap-3 bg-white/5 hover:bg-white/10 transition-colors border-r border-[var(--border)]"
		style={`width: ${$leftPartSize}%`}
		on:click={() => {
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
		<span
			class="inline-flex items-center justify-center h-7 w-7 rounded-xl bg-[var(--panel-strong)] text-xs text-[var(--accent)] border border-[var(--border)]"
		>
			{i + 1}
		</span>
		<p class="text-base font-semibold text-white leading-tight text-nowrap" class:truncate={truncatText}>
			{subfolder.name}
		</p>
	</button>

	<div class="relative flex-1 min-h-[72px] bg-gradient-to-r from-white/5 to-transparent">
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		<section
			aria-roledescription="separator"
			aria-orientation="vertical"
			role="separator"
			class="absolute top-0 left-0 h-full w-1 bg-white/10 hover:bg-[var(--accent)] transition-colors cursor-e-resize z-10"
			on:mousedown={(e) => {
				let initialWidth = e.clientX;
				let initialSize = $leftPartSize;

				//@ts-ignore
				const onMouseMove = (e) => {
					let newWidth = initialSize + ((e.clientX - initialWidth) / window.innerWidth) * 200;
					if (newWidth < 0) newWidth = 0;
					if (newWidth > 200) newWidth = 200;

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
			class="bg-[var(--accent-2)]/30 h-full absolute left-0 top-0"
		></div>

		<div
			style="width: {pourcentComparedWithSelectedFolder}%"
			class="bg-[var(--accent)]/35 h-full absolute left-0 top-0"
		></div>

		<div class="relative flex flex-col md:flex-row items-start md:items-center justify-between px-4 py-3 gap-2">
			<div class="flex flex-col">
				<span class="text-sm text-[var(--muted)]">Poids</span>
				<span class="text-lg font-semibold">
					{bytesToFormattedString(subfolder.size)}
				</span>
			</div>
			<div class="flex flex-wrap items-center gap-2 md:gap-4 text-xs text-[var(--muted)]">
				<span
					class="inline-flex items-center gap-2 px-2 py-1 rounded-full border border-[var(--border)] bg-white/5"
				>
					<span class="h-2 w-6 rounded-full bg-[var(--accent)]/80"></span>
					{pourcentComparedWithSelectedFolder.toFixed(2)}% du dossier sélectionné
				</span>
				{#if $currentFolder.fullPath !== $selectedFolder.fullPath}
					<span
						class="inline-flex items-center gap-2 px-2 py-1 rounded-full border border-[var(--border)] bg-white/5"
					>
						<span class="h-2 w-6 rounded-full bg-[var(--accent-2)]/90"></span>
						{pourcentComparedWithCurrentFolder.toFixed(2)}% du dossier courant
					</span>
				{/if}
			</div>
		</div>
	</div>
</div>

<ContextMenu bind:this={myMenu}>
	<Item
		on:click={() => {
			openFolderInExplorer(subfolder.fullPath);
		}}>Open in file explorer</Item
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
