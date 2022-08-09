<script>
	import { scale, fade, slide } from 'svelte/transition';
	import { trimString } from '../productOps';
	import OutClick from 'svelte-outclick';
	import { myCollectionStore } from '../stores/myCollectionsStore';
	import { pushToast } from '../stores/toastStore';
	import { waitForElm } from '../utils';
	import { deleteCollection, renameCollection } from '../api/collection';

	export let collection;

	let newName = collection.name;

	let options = false;
	let renaming = false;

	$: acceptRenameShouldBeGray = collection.name == newName || newName.trim().length == 0;

	const startRenaming = async () => {
		renaming = true;
		const elm = await waitForElm(`#collection-card-rename-input-${collection.id}`);
		elm.focus();
	};

	const handleRename = async () => {
		let res = await renameCollection(collection.id, collection.name, newName);
		if (res) {
			renaming = false;
		}
	};

	const handleRenameInput = async (e) => {
		if (e.key == 'Enter') {
			await handleRename();
		}
	};

	const changeVisibility = async () => {
		const body = JSON.stringify({
			query: `
                mutation changeVisibility {
                    changeVisibilityOfCollection(
                        collectionId:${collection.id}, public:${!collection.public}
                    ) {
                        id
                        name
                        public
                    }
                }
            `
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
				title: 'Error while changing visibility of the collection',
				content: `${errorMsg}`
			};
			pushToast(newToast);
			return;
		}

		console.log(`Got data: ${JSON.stringify(responseJson)}`);
		const changedCollection = responseJson.data.changeVisibilityOfCollection;
		console.log(`Changed collection is: ${JSON.stringify(changedCollection)}`);

		myCollectionStore.update((data) =>
			data.map((c) => {
				if (c.id == collection.id) {
					c.public = changedCollection.public;
				}
				return c;
			})
		);

		// PUSH TOAST
		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: `The collection "${trimString(
				changedCollection.name,
				25
			)}" has been successfully updated`,
			content: `Now it is ${changedCollection.public ? 'public' : 'private'}`
		};
		pushToast(newToast);
	};
</script>

<div
	in:scale
	class="h-full relative p-3 bg-neutral-50 rounded-md shadow-md outline outline-1 outline-neutral-100 hover:scale-105 transition-all hover:bg-orange-50"
>
	<!-- 3 dots with options -->
	<button on:click={() => (options = true)} class="absolute right-0 top-0 p-1 pt-1">
		<img class="w-6 h-6 opacity-40 hover:opacity-80" src="/options.png" alt="options" />
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
						on:click={async () => await deleteCollection(collection.id)}
						class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100">Delete</button
					>
					<button
						on:click={async () => await startRenaming()}
						class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100">Rename</button
					>
					<button
						on:click={changeVisibility}
						transition:fade|local
						class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100"
						>{collection.public ? 'Make private' : 'Make public'}</button
					>
				</div>
			</OutClick>
		</div>
	{/if}

	<!-- Header with name -->
	<div class="flex flex-row justify-start items-center gap-2">
		<!-- Icon with visibility status. Public or Private -->
		<div class="h-4 w-4 z-10">
			{#if collection.public}
				<img transition:slide|local src="/globe-color.png" alt="public" title="public" />
			{:else}
				<img transition:slide|local src="/lock.png" alt="public" title="private" />
			{/if}
		</div>
		<h3 class="text-lg font-semibold">
			{#if renaming}
				<div transition:slide|local class="flex flex-row justify-start align-middle gap-6 ml-2">
					<input
						id="collection-card-rename-input-{collection.id}"
						on:keydown={handleRenameInput}
						type="text"
						bind:value={newName}
					/>
					<button
						on:click={handleRename}
						class="transition-all  w-6 h-6 opacity-70 hover:opacity-100 {acceptRenameShouldBeGray
							? 'saturate-0'
							: 'saturate-100'}"><img src="/accept.png" alt="" /></button
					>
					<button
						on:click={() => {
							renaming = false;
							newName = collection.name;
						}}
						class="w-6 h-6 opacity-70 hover:opacity-100"><img src="/cancel.png" alt="" /></button
					>
				</div>
			{:else}
				<a transition:slide|local href="/collection/{collection.id}">
					{trimString(collection.name, 53)}
				</a>
			{/if}
		</h3>
	</div>

	<!-- Description -->
	<a href="/collection/{collection.id}" class="block text-justify">
		{collection.description ? trimString(collection.description, 210) : ''}
	</a>
</div>
