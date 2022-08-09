<script>
	import { publicCollectionsStore } from '../stores/collectionStore';
	import stringSimilarity from 'string-similarity';

	let searchValue = '';

	$: {
		if (searchValue.trim().length != 0) {
			publicCollectionsStore.update((data) => {
				data.map((c) => {
					let similarity = stringSimilarity.compareTwoStrings(
						c.name.toLowerCase(),
						searchValue.toLowerCase()
					);
					c.searchTermSimilarity = similarity;
				});
				data.sort((a, b) => b.searchTermSimilarity - a.searchTermSimilarity);
				return data;
			});
		} else {
			publicCollectionsStore.update((data) => {
				data.sort((a, b) => b.id - a.id);
				return data;
			});
		}
	}
</script>

<div
	class="w-full p-4 rounded-2xl shadow outline outline-gray-100 outline-1 flex flex-row justify-between items-center"
>
	<input bind:value={searchValue} type="text" class="w-full appearance-none outline-none text-xl" />
	<div class="flex flex-row">
		<div style="width: 4px;" class="h-auto bg-gray-300" />
		<img class="h-8 w-8 mx-2 ml-4" src="/loupe.png" alt="" />
	</div>
</div>
