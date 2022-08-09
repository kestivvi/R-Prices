<script>
	import { scale } from 'svelte/transition';
	import { pushToast } from '../stores/toastStore';
	import { trimString } from '../productOps';
	import OfferCard from './offerCard.svelte';
	import { currentProductStore } from '../stores/productStore';

	$: offers = $currentProductStore.offers;

	const handleDelete = (e) => {
		let offer = e.detail;
		offers = offers.filter((o) => o.id != offer.id);
		let newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: `Offer has been succesfully deleted`,
			content: `Now offer from ${trimString(offer.site)} is no longer tracked`
		};
		pushToast(newToast);
	};

	const handleChangedUrl = (e) => {
		let offer = e.detail;
		offers = offers.map((o) => {
			if (o.id == offer.id) {
				o.url = offer.url;
			}
			return o;
		});

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: "The offer's url has been successfully changed",
			content: `Now it is "${offer.url}"`
		};
		pushToast(newToast);
	};
</script>

<div class="m-4">
	<div
		in:scale
		class="flex flex-row justify-between align-middle my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md border-2 font-bold"
	>
		<div class="w-52 text-center">Site</div>
		<div class="ml-11">Last Price</div>
		<div>Last Update</div>
		<div class="mr-5">Delete</div>
	</div>
	{#each offers as offer (offer.id)}
		<OfferCard {offer} on:deleteOffer={handleDelete} on:changedUrl={handleChangedUrl} />
	{/each}
</div>
