import { getDirName, getFolderSize, getSubfolders, savedSize } from '$lib';
import type { Folder } from '$lib/Models/Folder';
import { get, writable, type Writable } from 'svelte/store';

export const selectedFolderPath: Writable<string | undefined> = writable(undefined);
export const selectedFolder: Writable<Folder> = writable();

export const isLoading: Writable<boolean> = writable(false);

export const currentFolder: Writable<Folder> = writable();
export const currentSubFolders: Writable<Folder[]> = writable();

export const pathToCurrentFolder: Writable<Folder[]> = writable([]);

export const leftPartSize: Writable<number> = writable(16.666667); // Layout
export const selectedElements = writable<Folder[]>([]);

selectedFolderPath.subscribe(async (folderPath) => {
	if (!folderPath) return;

	isLoading.set(true);

	const size = await getFolderSize(folderPath);
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

	// Calculate the size of the current folder
	const size = await getFolderSize(currentFolder.fullPath);
	currentFolder.size = size;

	const subfolders = await getSubfolders(currentFolder.fullPath);

	let subFolders: Folder[] = [];
	currentSubFolders.set(subFolders);

	for (const folder of subfolders) {
		const size = await getFolderSize(folder);

		subFolders.push({
			fullPath: folder,
			name: getDirName(folder),
			size: size
		});

		// Sort folders by size
		subFolders.sort((a, b) => {
			return b.size - a.size;
		});

		currentSubFolders.set(subFolders);
	}

	currentSubFolders.set(subFolders);

	isLoading.set(false);
});

export async function refresh() {
	for (var prop in savedSize) {
		if (savedSize.hasOwnProperty(prop)) {
			delete savedSize[prop];
		}
	}

	let _selectedFolder = get(selectedFolder);
	_selectedFolder.size = await getFolderSize(_selectedFolder.fullPath);

	selectedFolder.set(_selectedFolder);
	currentFolder.set(get(currentFolder));
}
