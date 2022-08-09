<script>
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import {
		changeNotification,
		deleteProduct,
		getFullProductById,
		renameProduct
	} from '../../api/product';
	import AddOffer from '../../components/addOffer.svelte';
	import NotAuthorizedPage from '../../components/notAuthorizedPage.svelte';
	import OfferList from '../../components/offerList.svelte';
	import { handleNotification } from '../../productOps';
	import { currentProductStore } from '../../stores/productStore';
	import { scale, fade } from 'svelte/transition';
	import OutClick from 'svelte-outclick';
	import ThreeDotsOptions from '../../components/threeDotsOptions.svelte';
	import { am_i_owner_of_collection } from '../../api/collection';
	import { waitForElm } from '../../utils';
	import ChartOfOfferPrices from '../../components/chartOfOfferPrices.svelte';

	let product_id = $page.params.id;
	let loading = true;
	let error_loading;

	let product;
	let newName = '';
	let owner_of_collection;

	onMount(async () => {
		let error = await getFullProductById(product_id);
		if (error) {
			error_loading = error;
		}
		product = $currentProductStore;
		newName = product.name;
		owner_of_collection = await am_i_owner_of_collection(product.collection.id);
		initOptionActions();

		loading = false;
	});

	const handleNewOffer = (e) => {
		const newOffer = e.detail;
		product.offers = [newOffer, ...product.offers];
	};

	const deleteAction = async () => {
		let error = await deleteProduct(product_id);
		if (!error) {
			history.back();
		}
	};

	const renameAction = async () => {
		renaming = true;
		const elem = await waitForElm(`#product-renaming-${product_id}`);
		elem.focus();
	};

	const notifyAction = async () => {
		await changeNotification(product_id, !product.notification);
		product.notification = !product.notification;
	};

	let actions = [];

	const initOptionActions = () => {
		if (owner_of_collection) {
			actions.push({
				name: 'Delete',
				callback: deleteAction
			});
			actions.push({
				name: 'Rename',
				callback: renameAction
			});
		}
		actions.push({
			name: 'Notify',
			callback: notifyAction
		});
	};

	let renaming = false;

	const handleRename = async () => {
		newName = newName.trim();
		let res = await renameProduct(product_id, newName);
		if (res) {
			product.name = newName;
			renaming = false;
		}
	};

	const handleRenameInput = async (e) => {
		if (e.key == 'Enter') {
			await handleRename();
		}
	};

	$: acceptRenameShouldBeGray =
		(product ? product.name : '') == newName || newName.trim().length == 0;
</script>

<svelte:head>
	<title>{product ? product.name : ''}</title>
</svelte:head>

{#if loading == false}
	{#if error_loading}
		<NotAuthorizedPage msg={error_loading} />
	{:else}
		<div class="bg-gray-50 rounded-xl shadow-sm outline outline-1 outline-gray-100 p-4 relative">
			<!-- Three dots options -->
			<div class="absolute right-0 top-0 m-2">
				<ThreeDotsOptions {actions} />
			</div>

			<!-- Title of the collection -->
			<div class="text-3xl font-semibold text-center mb-12 flex justify-center mx-4">
				{#if renaming == true}
					<div
						transition:fade|local
						class="w-full flex flex-col justify-start items-center gap-6 ml-2"
					>
						<input
							id="product-renaming-{product_id}"
							class="w-11/12 text-2xl"
							on:keydown={handleRenameInput}
							type="text"
							bind:value={newName}
						/>
						<div class="flex justify-center gap-8">
							<button
								on:click={handleRename}
								class="transition-all  w-8 h-8 opacity-70 hover:opacity-100 {acceptRenameShouldBeGray
									? 'saturate-0'
									: 'saturate-100'}"><img src="/accept.png" alt="" /></button
							>
							<button
								on:click={() => {
									renaming = false;
									newName = product.name;
								}}
								class="w-8 h-8 opacity-70 hover:opacity-100"
								><img src="/cancel.png" alt="" /></button
							>
						</div>
					</div>
				{:else}
					<div transition:fade|local class="flex flex-row justify-start items-center gap-6">
						<h1>{product.name}</h1>
					</div>
				{/if}
			</div>

			<div in:scale>
				<ChartOfOfferPrices />
			</div>

			{#if owner_of_collection}
				<AddOffer productId={product.id} on:NewOffer={handleNewOffer} />
			{/if}
			<OfferList />
		</div>
	{/if}
{/if}
