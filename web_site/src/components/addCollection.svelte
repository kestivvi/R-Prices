<script>
	import { fade, slide, scale } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';
	import { pushToast } from '../stores/toastStore';
	import { fetchMyCollections, myCollectionStore } from '../stores/myCollectionsStore';

	let adding = false;
	let name = '';
	let description = '';
	let isPublic = false;

	const add = async () => {
		// VALIDATE
		name = name.trim();
		description = description.trim();

		// TODO: Dont allow user to have two collections of the same name
		// Though, two different users can have their own collections of the same name
		if (name == '') {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: "Couldn't add a new collection",
				content: `Name for new collection cannot be empty!`
			};
			pushToast(newToast);
			return;
		}
		// REQUEST

		const body = JSON.stringify({
			query: `mutation createCol($newCollection:CreateCollectionInput!) {
						createCollection(newCollection:$newCollection) {
							id
							name
							description
							public
						}
					}`,
			variables: {
				newCollection: {
					name,
					description: description || null,
					public: isPublic
				}
			}
		});

		// HANDLE RESPONSE
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
				title: 'Error while adding the collection',
				content: `${errorMsg}`
			};
			pushToast(newToast);
			return;
		}

		const newCollection = responseJson.data.createCollection;

		myCollectionStore.update((data) => {
			data = [...data, newCollection];
			data.sort((a, b) => a.name.localeCompare(b.name));
			return data;
		});

		name = '';
		description = '';

		// PUSH TOAST
		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'The new collection has been added',
			content: `It's called "${newCollection.name}"`
		};
		pushToast(newToast);
	};

	const cancel = () => {
		adding = false;
		name = '';
		description = '';
	};
</script>

<button
	in:scale
	on:click={() => (adding = !adding)}
	class="relative w-fit  p-4 active:bg-gray-300 
	 {adding ? ' bg-gray-100 rounded-xl ' : ''}"
>
	<div class="flex items-center gap-2 transition-all hover:scale-110">
		<div
			in:fade|local={{ duration: 250, delay: 250 }}
			class="w-7 h-7 transition-all {adding ? 'caret-down' : 'caret-right'}"
		/>

		<div class="text-xl font-semibold">Add Collection</div>
	</div>
	{#if adding}
		<div class="absolute bottom-0 left-0 w-full h-2 bg-gray-100" />
	{/if}
</button>
{#if adding}
	<div
		transition:slide|local
		class="flex flex-col gap-5  p-4 bg-neutral-50 rounded-lg rounded-tl-none outline outline-1 outline-gray-100"
	>
		<label for="name" class="font-semibold">Name</label>
		<input
			bind:value={name}
			type="text"
			placeholder="Choose name for your new collection"
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
		<div class="flex flex-row gap-4 items-center">
			<label for="public" class="font-semibold">Public</label>
			<input bind:checked={isPublic} type="checkbox" class="h-5 w-5 accent-orange-400" />
		</div>
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
