<script>
	import { productStore } from '../stores/productStore';
	import EmptyReaction from './EmptyReaction.svelte';
	import ProductCard from './productCard.svelte';

	export let owner_of_collection;
	// <div
	// 	class="flex flex-row justify-between align-middle my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md border-2 font-bold"
	// >
	// 	<div class="w-80 text-center">Product</div>
	// 	<!-- <div class="ml-11">Last Lowest Price</div> -->
	// 	<!-- <div>Last Update</div> -->
	// 	<div class="w-52 text-center">Actions</div>
	// </div>
</script>

<div class="m-4">
	{#if $productStore.length == 0}
		<EmptyReaction />
	{:else}
		{#each $productStore as product (product.id)}
			<ProductCard {product} {owner_of_collection} />
		{/each}
	{/if}
</div>
