<script>
	import AddProduct from '../../components/addProduct.svelte';
	import ProductList from '../../components/productList.svelte';
	import { page } from '$app/stores';
	import { pushToast } from '../../stores/toastStore';
	import NotAuthorizedPage from '../../components/notAuthorizedPage.svelte';
	import { fade, scale, slide } from 'svelte/transition';
	import OutClick from 'svelte-outclick';
	import {
		am_i_owner_of_collection,
		deleteCollection,
		renameCollection
	} from '../../api/collection';
	import { onMount } from 'svelte';
	import { waitForElm } from '../../utils';
	import { fetchProductsOfCollection } from '../../stores/productStore';

	let loading = true;
	let error_loading;
	let collection;
	let renaming = false;
	let options = false;
	let newName = collection ? collection.name : '';
	let collection_id = $page.params.id;
	let owner_of_collection;

	onMount(async () => {
		let response = await fetch('http://127.0.0.1:4000/graphql', {
			method: 'POST',
			body: JSON.stringify({
				query: `
				query getCollection {
					getCollectionById(collectionId:${collection_id}) {
						id
						name
						description
						owner {
							id
							name
						}
						products {
							id
							name
						}
					}
				}
				`
			}),
			headers: {
				'content-type': 'application/json'
			},
			credentials: 'include'
		});

		const responseJson = await response.json();
		if (Object.hasOwn(responseJson, 'errors')) {
			const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
			error_loading = errorMsg;
			loading = false;
			return;
		}

		collection = responseJson.data.getCollectionById;
		newName = collection.name;
		owner_of_collection = await am_i_owner_of_collection(collection_id);
		await fetchProductsOfCollection(collection_id);
		loading = false;
	});

	const startRenaming = async () => {
		renaming = true;
		const elm = await waitForElm(`#collection-card-rename-input-${collection.id}`);
		elm.focus();
	};

	const handleDelete = async () => {
		let res = await deleteCollection(collection.id);
		if (res) {
			window.location.href = '/my-collections';
		}
	};

	const handleRename = async () => {
		newName = newName.trim();
		let res = await renameCollection(collection.id, collection.name, newName);
		if (res) {
			collection.name = newName;
			renaming = false;
		}
	};

	const handleRenameInput = async (e) => {
		if (e.key == 'Enter') {
			await handleRename();
		}
	};

	$: acceptRenameShouldBeGray =
		(collection ? collection.name : '') == newName || newName.trim().length == 0;
</script>

<svelte:head>
	<title>{collection ? collection.name : 'Access Denied'}</title>
</svelte:head>

{#if !loading}
	{#if error_loading}
		<NotAuthorizedPage msg={error_loading} />
	{:else}
		<div
			in:scale
			class="bg-gray-50 rounded-xl shadow-sm outline outline-1 outline-gray-100 p-4 relative"
		>
			{#if owner_of_collection}
				<!-- Three dots options -->
				<button
					on:click={() => (options = true)}
					class="h-6 w-6 absolute right-0 top-0 m-2 opacity-50 hover:opacity-100 hover:scale-125 transition-all ease-in-out"
				>
					<img src="/options.png" alt="" />
				</button>

				<!-- Options menu pop up -->
				{#if options}
					<div
						transition:scale|local
						class="absolute -top-2 -right-5 z-10 bg-slate-100 rounded-md px-2  outline outline-1 outline-gray-200 text-md font-semibold"
					>
						<OutClick on:outclick={() => (options = false)}>
							<div class="grid grid-cols-1 divide-y">
								<button
									on:click={handleDelete}
									class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100"
									>Delete</button
								>
								<button
									on:click={async () => await startRenaming()}
									class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100"
									>Rename</button
								>
								<!-- TODO: on click change visibility -->
								<button
									transition:fade|local
									class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100"
									>{collection.public ? 'Make private' : 'Make public'}</button
								>
							</div>
						</OutClick>
					</div>
				{/if}
			{/if}

			<!-- Title of the collection -->
			<div class="text-3xl font-semibold text-center mb-12 flex justify-center mx-4">
				{#if renaming == true}
					<div
						transition:fade|local
						class="w-full flex flex-col justify-start items-center gap-6 ml-2"
					>
						<input
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
									newName = collection.name;
								}}
								class="w-8 h-8 opacity-70 hover:opacity-100"
								><img src="/cancel.png" alt="" /></button
							>
						</div>
					</div>
				{:else}
					<div transition:fade|local class="flex flex-row justify-start items-center gap-6">
						<h1>{collection.name}</h1>
					</div>
				{/if}
			</div>

			{#if owner_of_collection}
				<AddProduct {collection_id} />
			{/if}
			<ProductList {owner_of_collection} />
		</div>
	{/if}
{/if}

<!-- 
<h1 class="text-3xl text-center font-bold">{product.name}</h1>
<div class="m-8 flex justify-center text-xl gap-8">
	<button
		on:click={async () => {
			product = await handleNotification(product);
			console.log(`Got back: ${JSON.stringify(product)}`);
		}}
		class="font-bold transition-all hover:scale-110 ease-in-out 
			{product.notification == true ? 'text-green-500' : 'text-gray-400'}"
	>
		Notification
	</button>
</div>

<AddOffer productId={product.id} on:NewOffer={handleNewOffer} />

<OfferList offers={product.offers} /> -->
