import { invoke } from '@tauri-apps/api';
import { selectedFolderPath } from '$lib/stores/GlobalStore';
import { open } from '@tauri-apps/api/dialog';

export function openFolderDialog() {
	open({
		multiple: false,
		directory: true
	}).then((result) => {
		if (result === null) return;

		selectedFolderPath.set(result as string); // Path vers le dossier
	});
}

export function openFolderInExplorer(path: string) {
	invoke('open_folder_in_explorer', {
		path: path
	});
}

export async function getFolderSize(path: string): Promise<number> {
	if (savedSize[path]) {
		return savedSize[path];
	}

	const size = await invoke('get_folder_size', {
		path: path
	});

	savedSize[path] = size as number;

	return size as number;
}

export async function getSubfolders(path: string): Promise<string[]> {
	const size = await invoke('get_subfolders_and_files', {
		path: path
	});

	return size as string[];
}

export function bytesToFormattedString(bytes: number): string {
	const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];

	if (bytes === 0) return '0 Byte';

	const i = Math.floor(Math.log(bytes) / Math.log(1024));

	return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + ' ' + sizes[i];
}

export function getDirName(fullPath: string): string {
	return fullPath.split('\\').reverse()[0];
}

export async function deleteFromDisk(path: string) {
	await invoke('delete_from_disk', {
		path: path
	});
}

export let savedSize: { [key: string]: number } = {};
