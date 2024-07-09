import { getDirName, getFolderSize, getSubfolders, savedSize } from '$lib';
import type { Folder } from '$lib/Models/Folder';
import { get, writable, type Writable } from 'svelte/store';

export const selectedFolderPath: Writable<string | undefined> = writable(undefined);
export const selectedFolder: Writable<Folder> = writable();

export const isLoading: Writable<boolean> = writable(false);

export const currentFolder: Writable<Folder> = writable();
export const currentSubFolders: Writable<Folder[]> = writable();

export const pathToCurrentFolder: Writable<Folder[]> = writable([]);

selectedFolderPath.subscribe(async (folderPath) => {
	if (!folderPath) return;

	isLoading.set(true);

	const size = savedSize[folderPath] || (await getFolderSize(folderPath));
	const folder = {
		fullPath: folderPath,
		name: getDirName(folderPath),
		size: size
	}; // Set the actual folder here

	selectedFolder.set(folder);
	currentFolder.set(folder);
	pathToCurrentFolder.set([folder]);
});

currentFolder.subscribe(async (currentFolder) => {
	if (!currentFolder) return;

	isLoading.set(true);

	const subfolders = await getSubfolders(currentFolder.fullPath);

	let subFolders: Folder[] = [];
	currentSubFolders.set(subFolders);

	for (const folder of subfolders) {
		const size = savedSize[folder] || (await getFolderSize(folder));

		subFolders.push({
			fullPath: folder,
			name: getDirName(folder),
			size: size
		});

		savedSize[folder] = size;

		// Sort folders by size
		subFolders.sort((a, b) => {
			return b.size - a.size;
		});

		currentSubFolders.set(subFolders);
	}

	currentSubFolders.set(subFolders);

	isLoading.set(false);
});
