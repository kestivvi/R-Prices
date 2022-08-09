<script>
	import { onMount } from 'svelte';
	import { fetchMyCollections, myCollectionStore } from '../stores/myCollectionsStore';
	import CollectionCard from './collectionCard.svelte';
	import { flip } from 'svelte/animate';

	onMount(async () => {
		await fetchMyCollections();
	});
</script>

<div class="grid grid-cols-2 gap-8">
	{#each $myCollectionStore as collection (collection.id)}
		<div animate:flip={{ duration: 400 }}>
			<CollectionCard {collection} />
		</div>
	{/each}
</div>
