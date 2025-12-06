<script lang="ts">
	import { openFolderDialog } from '$lib';
</script>

<div class="relative w-full h-full overflow-hidden">
	<div class="pointer-events-none absolute inset-0">
		<div
			class="absolute w-[520px] h-[520px] bg-[var(--accent)]/15 blur-[120px] -left-32 -top-20"
		></div>
		<div
			class="absolute w-[440px] h-[440px] bg-[var(--accent-2)]/14 blur-[120px] right-6 bottom-[-140px]"
		></div>
	</div>

	<section
		class="relative max-w-5xl mx-auto h-full flex flex-col items-center justify-center gap-8 px-6 py-14 text-center"
	>
		<p class="uppercase tracking-[0.32em] text-xs text-[var(--muted)]">Folderstats</p>

		<div
			class="bg-[var(--panel)] border border-[var(--border)] shadow-[0_30px_90px_rgba(0,0,0,0.35)] rounded-3xl p-10 md:p-12 backdrop-blur-lg space-y-6"
		>
			<h1 class="text-4xl md:text-5xl font-semibold leading-tight">
				Cartographiez vos dossiers en quelques secondes
			</h1>
			<p class="text-lg text-[var(--muted)] max-w-3xl mx-auto">
				Surveillez l'espace disque, ouvrez les dossiers lourds et faites le tri sans perdre de vue la
				structure. L'interface met en avant les dossiers importants et reste parfaitement lisible
				même sur un petit écran.
			</p>

			<div class="flex flex-wrap items-center justify-center gap-4 pt-2">
				<button
					class="inline-flex items-center gap-3 px-6 py-3 rounded-2xl font-semibold text-slate-900 bg-gradient-to-r from-[var(--accent)] to-[var(--accent-2)] shadow-[0_18px_45px_rgba(124,242,179,0.32)] hover:shadow-[0_20px_55px_rgba(246,196,83,0.38)]"
					on:click={openFolderDialog}
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
							d="M3 7.5A2.25 2.25 0 0 1 5.25 5.25h4.072c.506 0 .99.201 1.347.558l1.12 1.12c.357.357.84.558 1.347.558h4.614A2.25 2.25 0 0 1 20.25 9.75v6.75A2.25 2.25 0 0 1 18 18.75H6A2.25 2.25 0 0 1 3.75 16.5V7.5Z"
						/>
					</svg>
					Sélectionner un dossier
				</button>

				<div
					class="text-sm text-[var(--muted)] px-4 py-3 rounded-2xl border border-[var(--border)] bg-[var(--panel-strong)]"
				>
					Aucune installation supplémentaire : glissez simplement vos dossiers pour les explorer.
				</div>
			</div>

			<div class="grid gap-4 md:grid-cols-3 text-left pt-4">
				<div class="p-4 rounded-2xl bg-[var(--panel-strong)] border border-[var(--border)]">
					<p class="text-sm uppercase tracking-[0.2em] text-[var(--muted)] mb-2">Vue claire</p>
					<p class="text-base">Barres de comparaison immédiates pour repérer les dossiers volumineux.</p>
				</div>
				<div class="p-4 rounded-2xl bg-[var(--panel)] border border-[var(--border)]">
					<p class="text-sm uppercase tracking-[0.2em] text-[var(--muted)] mb-2">Actions rapides</p>
					<p class="text-base">Ouvrez l'explorateur ou supprimez un dossier en un clic.</p>
				</div>
				<div class="p-4 rounded-2xl bg-[var(--panel)] border border-[var(--border)]">
					<p class="text-sm uppercase tracking-[0.2em] text-[var(--muted)] mb-2">Optimisé bureau</p>
					<p class="text-base">Lisible, contrasté et prêt pour les grands ou petits écrans.</p>
				</div>
			</div>
		</div>
	</section>
</div>
