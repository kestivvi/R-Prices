<script>
	import { fade } from 'svelte/transition';
	import { productStore } from '../stores/productStore';
	import { changeNotification } from '../api/product';

	import { trimString } from '../productOps';
	import { pushToast } from '../stores/toastStore';
	import ThreeDotsOptions from './threeDotsOptions.svelte';
	import { waitForElm } from '../utils';

	export let product;
	export let owner_of_collection;
	let isRenaming = false;
	let newName = product.name;

	$: acceptRenameShouldBeGray = product.name == newName || newName.trim().length == 0;

	let actions = [];

	const handleDelete = async () => {
		const body = JSON.stringify({
			query: `mutation delProd {
                        deleteProduct(id:${product.id}) {
                            id
                            name
                            notification
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
		productStore.update((data) => data.filter((p) => p.id != product.id));

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Product has been deleted',
			content: `${trimString(product.name)} is no longer there`
		};
		pushToast(newToast);
	};

	const handleRename = async () => {
		newName = newName.trim();

		if (product.name == newName) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Name has not been updated',
				content: 'New name cannot be the same as the previous one'
			};
			pushToast(newToast);
			return;
		}

		if (newName.length == 0) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Name has not been updated',
				content: 'New name cannot be zero length'
			};
			pushToast(newToast);
			return;
		}

		// Send request
		const body = JSON.stringify({
			query: `mutation renameProd {
						renameProduct(id:${product.id}, newValue:"${newName}") {
							id
							name
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
				title: 'Error while renaming the product',
				content: `${errorMsg}`
			};
			pushToast(newToast);
			return;
		}

		console.log(`Got data: ${JSON.stringify(responseJson)}`);
		const renamedProduct = responseJson.data.renameProduct;
		console.log(`New offer is: ${JSON.stringify(renamedProduct)}`);

		if (renamedProduct.name != newName) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Error while renaming the product',
				content: 'It was unsuccesful for some unknown reason'
			};
			pushToast(newToast);
			return;
		}

		//// Push Toast
		//// break;

		// Change productStore
		let oldName = product.name;
		productStore.update((data) => {
			data.map((p) => {
				if (p.id == product.id) {
					p.name = newName;
				}
			});
			return data;
		});

		isRenaming = false;

		// Push Toast
		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'The product has been successfully renamed',
			content: `Now instead of "${oldName}" it is called "${renamedProduct.name}"`
		};

		pushToast(newToast);

		console.log(JSON.stringify($productStore));
	};

	const handleRenameInput = (e) => {
		if (newName == product.name || newName.trim().length == 0) {
			return;
		}
		if (e.key == 'Enter') {
			handleRename();
		}
	};

	const renameAction = async () => {
		isRenaming = true;
		const elem = await waitForElm(`#product-card-rename-input-${product.id}`);
		elem.focus();
	};

	const notifyAction = async () => {
		await changeNotification(product.id, !product.notification);
	};

	const initOptionActions = () => {
		if (owner_of_collection) {
			actions.push({
				name: 'Delete',
				callback: handleDelete
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

	initOptionActions();
</script>

<div
	class="flex flex-row justify-between items-center my-2 p-2 bg-gray-100 rounded-md shadow-sm hover:shadow-md transition-all"
>
	<div class="flex flex-row justify-start items-center gap-4 ml-2">
		<!-- Notification button (Bell) -->
		<button
			on:click={notifyAction}
			class="font-bold transition-all hover:scale-125 ease-in-out
            {product.notification == true ? 'saturate-100' : 'saturate-0'}"
		>
			<img class="h-6 transition-all ease-in-out" src="/bell.png" alt="notification bell icon" />
		</button>

		<!-- Name of the product -->
		{#if isRenaming == true}
			<div
				in:fade|local={{ duration: 250, delay: 250 }}
				out:fade|local={{ duration: 250 }}
				class="flex flex-row justify-start items-center gap-6"
			>
				<input
					class="w-96 shadow appearance-none  rounded focus:outline-none bg-gray-50 text-md p-2 font-semibold shadow-orange-300"
					id="product-card-rename-input-{product.id}"
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
						isRenaming = false;
						newName = product.name;
					}}
					class="w-6 h-6 opacity-70 hover:opacity-100"><img src="/cancel.png" alt="" /></button
				>
			</div>
		{:else}
			<a
				in:fade|local={{ duration: 250, delay: 250 }}
				out:fade|local={{ duration: 250 }}
				href="/product/{product.id}"
				class="hover:scale-110 transition-all ease-in-out font-semibold"
			>
				{product.name}
			</a>
		{/if}
	</div>

	<ThreeDotsOptions {actions} />
</div>
