<script>
	import { fade, slide } from 'svelte/transition';
	import { trimString } from '../productOps';
	import { productStore } from '../stores/productStore';
	import { pushToast } from '../stores/toastStore';

	export let collection_id;

	let name = '';
	let description = '';

	const add = async () => {
		name = name.trim();
		description = description.trim();

		if (name.length == 0) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Product has not been added',
				content: 'Name cannot be zero length'
			};
			pushToast(newToast);
			return;
		}

		const body = JSON.stringify({
			query: `mutation createP($newProduct: CreateProductInput!) {
                            createProduct(input: $newProduct) { 
                                id
                                name
                            }
                        }
                `,
			variables: {
				newProduct: {
					name: name,
					description: description != '' ? description : null,
					collectionId: +collection_id
				}
			}
		});

		const response = await fetch('http://127.0.0.1:4000/graphql', {
			headers: { 'content-type': 'application/json' },
			method: 'POST',
			body: body,
			credentials: 'include'
		});

		const responseJson = await response.json();

		//  CHECK FOR ERRORS
		if (Object.hasOwn(responseJson, 'errors')) {
			const errorMsg = responseJson.errors.map((i) => i.message).join('\n');
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Error while adding the product',
				content: `${errorMsg}`
			};
			pushToast(newToast);
			return;
		}

		const id = responseJson.data.createProduct.id;
		const newProduct = {
			id: id,
			name: name
		};
		productStore.update((data) => {
			return [newProduct, ...data];
		});

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'New product has been added',
			content: `Now you can add offers for ${trimString(newProduct.name)}`
		};
		pushToast(newToast);

		name = '';
		description = '';
	};

	let adding = false;

	const cancel = () => {
		adding = false;
		name = '';
		description = '';
	};
</script>

<!-- <div
	class="mx-72 my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md transition-all flex flex-row justify-center"
	transition:fade
>
	<input
		type="text"
		placeholder="Name of the new product"
		class="p-2 rounded-lg outline-none border-gray-300 border-2 w-2/3 text-center"
		on:keydown={handleSubmit}
		bind:value
	/>
</div> -->

<div>
	<button
		on:click={() => (adding = !adding)}
		class="relative w-fit  p-4 active:bg-gray-300 
         {adding ? ' bg-gray-100 rounded-xl ' : ''}"
	>
		<div class="flex items-center gap-2 transition-all hover:scale-110">
			<div
				in:fade={{ duration: 250 }}
				class="w-7 h-7 transition-all {adding ? 'caret-down' : 'caret-right'}"
			/>

			<div class="text-xl font-semibold">Add Product</div>
		</div>
		{#if adding}
			<div class="absolute bottom-0 left-0 w-full h-2 bg-gray-100" />
		{/if}
	</button>
	{#if adding}
		<div
			transition:slide
			class="flex flex-col gap-5  p-4 bg-neutral-50 rounded-lg rounded-tl-none outline outline-1 outline-gray-100"
		>
			<label for="name" class="font-semibold">Name</label>
			<input
				bind:value={name}
				type="text"
				placeholder="Choose name for your new product"
				class="shadow-md rounded-lg p-2 outline outline-1 outline-gray-100"
			/>
			<label for="name" class="font-semibold">Description</label>
			<textarea
				bind:value={description}
				name="description"
				placeholder="(Optional) You can write informative description if you like"
				id=""
				cols="30"
				rows="3"
				class="shadow-md rounded-lg p-2 outline outline-1 outline-gray-100"
			/>
			<div class="flex flex-row gap-7">
				<button
					on:click={add}
					class="bg-gray-100 p-2 rounded-lg shadow-md outline outline-1 outline-gray-300 font-semibold hover:scale-110 transition-all"
					>Add</button
				>
				<button
					on:click={cancel}
					class="bg-gray-100 p-2 rounded-lg shadow-md outline outline-1 outline-gray-300 font-semibold text-gray-600 hover:scale-110 transition-all"
					>Cancel</button
				>
			</div>
		</div>
	{/if}
</div>

<style>
	.caret-down {
		background: url('/caret-right.png');
		background-size: contain;

		-webkit-transition: all 0.3s ease-in-out;
		-moz-transition: all 0.3s ease-in-out;
		transition: all 0.3s ease-in-out;
		transform: rotate(90deg);
	}

	.caret-right {
		background: url('/caret-down.png');
		background-size: contain;
		-webkit-transition: all 0.3s ease-in-out;
		-moz-transition: all 0.3s ease-in-out;
		transition: all 0.3s ease-in-out;
		transform: rotate(-90deg);
	}
</style>
