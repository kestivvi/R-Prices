<script>
	import { fade, scale } from 'svelte/transition';
	import { productStore } from '../stores/productStore';
	import { deleteOffer, handleNotification } from '../productOps';
	import { createEventDispatcher, onMount } from 'svelte';
	import { pushToast } from '../stores/toastStore';

	export let offer;
	// console.log(`OfferCard got: ${JSON.stringify(offer)}`);
	let dispatch = createEventDispatcher();

	let newUrl = offer.url;
	let isChangingUrl = false;

	const lastNotNullPrice = (offer) => {
		let last = 'No Data';
		for (let i = offer.prices.length - 1; i >= 0; i--) {
			if (offer.prices[i].value != null) return offer.prices[i].value.toFixed(2);
		}

		return last;
	};

	let lastPrice = lastNotNullPrice(offer);

	// TODO: Price data is not synced with price value
	let lastPriceDate =
		offer.prices.length > 0
			? new Date(offer.prices[offer.prices.length - 1].createdAt * 1000).toLocaleString()
			: 'No Data';

	$: acceptRenameShouldBeGray = offer.url == newUrl || newUrl.trim().length == 0;

	const handleDelete = () => {
		let success = deleteOffer(offer.id);
		if (success) {
			dispatch('deleteOffer', offer);
		}
	};

	const handleRename = async () => {
		newUrl = newUrl.trim();

		if (offer.url == newUrl) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Url has not been updated',
				content: 'New url cannot be the same as the previous one'
			};
			pushToast(newToast);
			return;
		}

		if (newUrl.length == 0) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Url has not been updated',
				content: 'New url cannot be zero length'
			};
			pushToast(newToast);
			return;
		}

		// Send request
		const body = JSON.stringify({
			query: `mutation changeUrl {
						changeUrlOfOffer(id:${offer.id}, newValue:"${newUrl}") {
							id
							url
						}
					}
                `
		});

		const response = await fetch('http://127.0.0.1:4000/graphql', {
			headers: { 'content-type': 'application/json' },
			method: 'POST',
			body: body
		});

		const responseJson = await response.json();

		// Check for errors
		if (Object.hasOwn(responseJson, 'errors')) {
			const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Error while changing url of the offer',
				content: `${errorMsg}`
			};
			pushToast(newToast);
			return;
		}

		// console.log(`Got data: ${JSON.stringify(responseJson)}`);
		const changedOffer = responseJson.data.changeUrlOfOffer;
		// console.log(`New offer is: ${JSON.stringify(changedOffer)}`);

		if (changedOffer.url != newUrl) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: "Error while chaging offer's url",
				content: 'It was unsuccesful for some unknown reason'
			};
			pushToast(newToast);
			return;
		}

		//// Push Toast
		//// break;

		// Change productStore
		let oldUrl = offer.url;
		offer.url = newUrl;

		dispatch('changedUrl', offer);

		isChangingUrl = false;
	};

	const handleRenameInput = (e) => {
		if (newUrl == offer.url || newUrl.trim().length == 0) {
			return;
		}
		if (e.key == 'Enter') {
			handleRename();
		}
	};
</script>

<div
	class="flex flex-row justify-between align-middle my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md transition-all"
	in:scale
>
	<div class="flex flex-row justify-start align-middle gap-4 ml-2 w-60">
		<div>
			{offer.id}.
		</div>
		{#if isChangingUrl == true}
			<div
				in:fade|local={{ duration: 250, delay: 250 }}
				out:fade|local={{ duration: 250 }}
				class="flex flex-row justify-start align-middle gap-6 ml-2"
			>
				<input on:keydown={handleRenameInput} type="text" bind:value={newUrl} />
				<button
					on:click={handleRename}
					class="transition-all  w-6 h-6 opacity-70 hover:opacity-100 {acceptRenameShouldBeGray
						? 'saturate-0'
						: 'saturate-100'}"><img src="/accept.png" alt="" /></button
				>
				<button
					on:click={() => {
						isChangingUrl = false;
						newUrl = offer.url;
					}}
					class="w-6 h-6 opacity-70 hover:opacity-100"><img src="/cancel.png" alt="" /></button
				>
			</div>
		{:else}
			<div
				in:fade|local={{ duration: 250, delay: 250 }}
				out:fade|local={{ duration: 250 }}
				class="flex flex-row justify-start align-middle gap-6 ml-2 font-semibold text-blue-600"
			>
				<a
					href={offer.url}
					class="text-blue-500 font-semibold hover:scale-110 transition-all ease-in-out"
				>
					{offer.site}
				</a>
				<button
					title="Rename"
					on:click={() => (isChangingUrl = true)}
					class="w-5 h-5 opacity-70 hover:opacity-100 hover:scale-110 transition-all ease-in-out"
				>
					<img src="/rename.png" alt="" />
				</button>
			</div>
		{/if}
	</div>
	<div>
		{lastPrice}
		{offer.prices.length > 0 ? 'PLN' : ''}
	</div>
	<div>
		{lastPriceDate}
	</div>
	<div class="flex flex-row justify-start align-middle gap-8 mr-4">
		<!-- <button
			on:click={() => handleNotification(offer)}
			class="font-bold transition-all 
            {offer.notification == true ? 'text-green-500' : 'text-gray-400'}"
		>
			Notification
		</button> -->
		<button
			on:click={handleDelete}
			class="text-red-500 font-bold opacity-90 hover:opacity-100 hover:scale-110 transition-all ease-in-out"
		>
			DELETE
		</button>
	</div>
</div>
