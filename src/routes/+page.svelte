<script lang="ts">
	import DataVisualizer from '$lib/components/DataVisualizer.svelte';
	import ImportFolder from '$lib/components/ImportFolder.svelte';
	import { isLoading, selectedFolderPath } from '$lib/stores/GlobalStore';
	import { onMount } from 'svelte';

	onMount(() => {
		// Disable right click context menu
		document.addEventListener('contextmenu', (event) => event.preventDefault());

		//@ts-ignore
		kofiWidgetOverlay.draw('vzero', {
			type: 'floating-chat',
			'floating-chat.donateButton.text': 'Support me',
			'floating-chat.donateButton.background-color': '#794bc4',
			'floating-chat.donateButton.text-color': '#fff'
		});
	});
</script>

<div class="w-screen h-screen">
	{#if !$selectedFolderPath}
		<ImportFolder />
	{:else}
		<DataVisualizer />
	{/if}
</div>
