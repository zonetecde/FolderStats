<script lang="ts">
	import { openFolderDialog } from '$lib';
</script>

<div class="w-full h-full flex items-center justify-center">
	<button class="outline-dashed px-10 py-3 rounded-lg" on:click={openFolderDialog}
		>Select a folder</button
	>
</div>
