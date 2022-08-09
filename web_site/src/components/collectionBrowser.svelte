<script>
	import { collectionStore } from '../stores/collectionStore';
</script>

{#each $collectionStore as collection (collection.id)}
	<p>
		{collection.name}
	</p>
{/each}
