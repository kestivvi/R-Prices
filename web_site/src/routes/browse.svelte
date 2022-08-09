<script>
	import { onMount } from 'svelte';
	import { publicCollectionsStore, fetchPublicCollections } from '../stores/collectionStore';
	import { flip } from 'svelte/animate';
	import { fade, scale } from 'svelte/transition';
	import PublicCollectionCard from '../components/publicCollectionCard.svelte';
	import PublicCollectionSearchBox from '../components/publicCollectionSearchBox.svelte';

	onMount(async () => {
		await fetchPublicCollections();
	});
</script>

<div in:scale class="mx-4 mb-10">
	<PublicCollectionSearchBox />
</div>

<div in:scale class="grid grid-cols-2 gap-8">
	{#each $publicCollectionsStore as collection (collection.id)}
		<div class={collection.show ? '' : 'hidden'} animate:flip={{ duration: 400 }}>
			<PublicCollectionCard {collection} />
		</div>
	{/each}
</div>
