<script>
	import { scale } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';
	import { productStore } from '../stores/productStore';
	import { pushToast } from '../stores/toastStore';
	import { trimString } from '../productOps';
	import { addOfferToProduct } from '../api/product';

	export let productId;
	let newOfferUrl;
	let dispatch = createEventDispatcher();

	const handleSubmit = async (e) => {
		if (e.key == 'Enter' && newOfferUrl.trim().length != 0) {
			let errors = await addOfferToProduct(productId, newOfferUrl);
			if (!errors) {
				newOfferUrl = '';
			}
		}
	};
</script>

<div
	class="mx-80 my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md transition-all flex flex-row justify-center"
	in:scale
>
	<input
		type="text"
		placeholder="Paste in here the url to the new offer"
		class="p-2 rounded-lg outline-none border-gray-300 border-2 w-3/4 text-center"
		on:keydown={handleSubmit}
		bind:value={newOfferUrl}
	/>
</div>
