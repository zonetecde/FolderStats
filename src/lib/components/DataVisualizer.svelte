<script>
	import { bytesToFormattedString, getDirName, openFolderDialog } from '$lib';
	import {
		currentFolder,
		currentSubFolders,
		isLoading,
		selectedFolder,
		selectedFolderPath
	} from '$lib/stores/GlobalStore';
	import DirList from './DirList.svelte';
</script>

<div class="relative w-full h-full overflow-hidden">
	<div class="pointer-events-none absolute inset-0 opacity-70">
		<div
			class="absolute w-[460px] h-[460px] bg-[var(--accent)]/18 blur-[130px] -left-28 -top-28"
		></div>
		<div
			class="absolute w-[560px] h-[560px] bg-[var(--accent-2)]/12 blur-[150px] -right-32 bottom-[-220px]"
		></div>
	</div>

	<div class="relative max-w-6xl w-full mx-auto px-6 py-8 md:py-12 flex flex-col gap-6">
		<div class="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
			<div class="space-y-2">
				<p class="text-xs uppercase tracking-[0.3em] text-[var(--muted)]">Dossier ouvert</p>
				<div class="flex flex-wrap items-center gap-3">
					<span
						class="inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-[var(--border)] bg-[var(--panel-strong)] text-[var(--accent)]"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.8"
							stroke="currentColor"
							class="size-5"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M3.75 5.25h4.493c.414 0 .81.172 1.09.464l1.125 1.172c.28.293.676.464 1.09.464H19.5A1.5 1.5 0 0 1 21 8.85v8.4a1.5 1.5 0 0 1-1.5 1.5h-15a1.5 1.5 0 0 1-1.5-1.5v-12a1.5 1.5 0 0 1 1.5-1.5Z"
							/>
						</svg>
					</span>
					<button
						class="text-2xl md:text-3xl font-semibold hover:text-[var(--accent)] transition-colors"
						on:click={() => {
							if ($isLoading) return;
							openFolderDialog();
						}}
					>
						{getDirName($selectedFolderPath ?? '')}
					</button>
				</div>
				<p class="text-sm text-[var(--muted)] break-all md:break-words">
					{$selectedFolderPath}
				</p>
			</div>

			<div class="flex flex-wrap items-center gap-3">
				<div class="rounded-2xl border border-[var(--border)] bg-[var(--panel)] px-4 py-3">
					<p class="text-xs uppercase tracking-[0.2em] text-[var(--muted)]">Taille totale</p>
					<p class="text-xl font-semibold">
						{bytesToFormattedString($selectedFolder?.size ?? 0)}
					</p>
					<p class="text-xs text-[var(--muted)]">Calculée sur le dossier sélectionné.</p>
				</div>
				<button
					class="px-4 py-3 rounded-2xl border border-[var(--accent)] text-[var(--accent)] hover:bg-[var(--accent)] hover:text-slate-900 font-semibold bg-[var(--panel)]"
					on:click={() => {
						if ($isLoading) return;
						openFolderDialog();
					}}
				>
					Changer de dossier
				</button>
			</div>
		</div>

		{#if $currentFolder && $currentSubFolders}
			<div class="grid gap-4 md:grid-cols-3">
				<div class="rounded-2xl border border-[var(--border)] bg-[var(--panel-strong)] p-4">
					<p class="text-xs uppercase tracking-[0.2em] text-[var(--muted)] mb-1">Sous-dossiers</p>
					<p class="text-2xl font-semibold">{$currentSubFolders.length}</p>
					<p class="text-sm text-[var(--muted)]">Triés par taille décroissante.</p>
				</div>
				<div class="rounded-2xl border border-[var(--border)] bg-[var(--panel)] p-4">
					<p class="text-xs uppercase tracking-[0.2em] text-[var(--muted)] mb-1">Taille en vue</p>
					<p class="text-2xl font-semibold">{bytesToFormattedString($currentFolder.size)}</p>
					<p class="text-sm text-[var(--muted)]">
						{#if $currentFolder.fullPath !== $selectedFolder.fullPath}
							Par rapport au dossier courant.
						{:else}
							Correspond au dossier sélectionné.
						{/if}
					</p>
				</div>
				<div class="rounded-2xl border border-[var(--border)] bg-[var(--panel)] p-4">
					<p class="text-xs uppercase tracking-[0.2em] text-[var(--muted)] mb-1">Astuce</p>
					<p class="text-base">Faites glisser le séparateur vertical pour ajuster la largeur des noms.</p>
				</div>
			</div>

			<div
				class="bg-[var(--panel)] border border-[var(--border)] rounded-3xl shadow-[0_25px_80px_rgba(0,0,0,0.35)] p-4 md:p-6 space-y-4"
			>
				<div class="flex items-center justify-between gap-3 flex-wrap">
					<div>
						<p class="text-lg font-semibold">Répartition des dossiers</p>
						<p class="text-sm text-[var(--muted)]">Navigation fluide avec clic droit pour plus d'options.</p>
					</div>
					{#if $isLoading}
						<div class="inline-flex items-center gap-2 px-3 py-2 rounded-full bg-white/5 border border-[var(--border)]">
							<span class="h-2 w-2 rounded-full bg-[var(--accent)] animate-pulse"></span>
							<p class="text-sm text-[var(--muted)]">Mise à jour en cours...</p>
						</div>
					{/if}
				</div>

				<DirList />

				{#if !$isLoading}
					<div class="flex flex-wrap items-center gap-4 pt-2 text-sm text-[var(--muted)]">
						<div class="flex items-center gap-2">
							<span
								class="w-10 h-3 rounded-full bg-[var(--accent)]/80 border border-[var(--border)]"
							></span>
							<span>Comparaison avec le dossier sélectionné</span>
						</div>
						{#if $currentFolder.fullPath !== $selectedFolder.fullPath}
							<div class="flex items-center gap-2">
								<span
									class="w-10 h-3 rounded-full bg-[var(--accent-2)]/80 border border-[var(--border)]"
								></span>
								<span>Comparaison avec le dossier courant</span>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
