<script>
	import { scale, fade, slide } from 'svelte/transition';
	import { trimString } from '../productOps';
	import OutClick from 'svelte-outclick';
	import { myCollectionStore } from '../stores/myCollectionsStore';
	import { pushToast } from '../stores/toastStore';
	import { waitForElm } from '../utils';
	import { deleteCollection, renameCollection } from '../api/collection';
	import ThreeDotsOptions from './threeDotsOptions.svelte';

	export let collection;
</script>

<div
	class="h-full relative p-3 bg-neutral-50 rounded-md shadow-md outline outline-1 outline-neutral-100 hover:scale-105 transition-all hover:bg-orange-50"
>
	<!-- <div class="absolute right-0 top-0 p-1 pt-1">
		<ThreeDotsOptions />
	</div> -->

	<!-- Header with name -->
	<div class="flex flex-row justify-start items-center gap-2">
		<!-- Icon with visibility status. Public or Private -->
		<div class="h-4 w-4 z-10">
			<img src="/globe-color.png" alt="public" title="public" />
		</div>
		<h3 class="text-lg font-semibold">
			<a href="/collection/{collection.id}">
				{trimString(collection.name, 53)}
			</a>
		</h3>
	</div>

	<!-- Description -->
	<a href="/collection/{collection.id}" class="block text-justify mb-4">
		{collection.description ? trimString(collection.description, 210) : ''}
	</a>

	<div
		class="absolute right-0 bottom-0 m-1 mr-2 font-semibold text-gray-700 italic flex flex-row items-center gap-2"
	>
		<img class="h-3 w-3" src="/user.png" alt="user icon" />
		{collection.owner.name}
	</div>
</div>
