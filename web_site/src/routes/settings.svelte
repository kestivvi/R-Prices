<script>
	import { scale } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { changeEmail, changeName, changePassword, me } from '../auth_utils';
	import { pushToast } from '../stores/toastStore';

	let user = {
		name: '',
		email: ''
	};

	let newName = '';
	let newEmail = '';

	let oldPassword = '';
	let newPassword = '';

	onMount(async () => {
		await fetchMe();
		newName = user.name;
		newEmail = user.email;
	});

	const fetchMe = async () => {
		let res = await me();
		//// - CHECK FOR ERRORS
		if (Object.hasOwn(res, 'errors')) {
		} else {
			user = res;
		}
	};

	const handleSavePersonalInformation = async () => {
		newName = newName.trim();
		newEmail = newEmail.trim();

		if (newName != user.name) {
			await handleNewName();
		}

		if (newEmail != user.email) {
			await handleNewEmail();
		}
	};

	const handleNewName = async () => {
		if (newName.length == 0) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Name was not updated',
				content: `New name cannot be zero length`
			};
			pushToast(newToast);
			return;
		}
		if (newName == user.name) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Name was not updated',
				content: `New name cannot be the same as previous one`
			};
			pushToast(newToast);
			return;
		}

		let error = await changeName(newName);

		if (error) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Name was not updated',
				content: `${error}`
			};
			pushToast(newToast);
			return;
		}

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Name has been updated',
			content: `You have successfully updated your name`
		};
		pushToast(newToast);

		user.name = newName;
	};

	const handleNewEmail = async () => {
		// TODO: VALIDATE EMAIL
		if (newEmail.length == 0) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Email was not updated',
				content: `New email cannot be zero length`
			};
			pushToast(newToast);
			return;
		}
		if (newEmail == user.email) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Email was not updated',
				content: `New email cannot be the same as previous one`
			};
			pushToast(newToast);
			return;
		}

		let error = await changeEmail(newEmail);

		if (error) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Email was not updated',
				content: `${error}`
			};
			pushToast(newToast);
			return;
		}

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Email has been updated',
			content: `You have successfully updated your email`
		};
		pushToast(newToast);

		user.email = newEmail;
	};

	const handleCancelPersonalInformation = async () => {
		newName = user.name;
		newEmail = user.email;
	};

	const handleSavePassword = async () => {
		oldPassword = oldPassword.trim();
		newPassword = newPassword.trim();

		// TODO: VALIDATE STRENGTH OF PASSWORD
		if (oldPassword.length < 4) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Password was not updated',
				content: `Old password surely cannot be shorter than 4 characters`
			};
			pushToast(newToast);
			return;
		}
		if (newPassword.length < 4) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'warning',
				title: 'Password was not updated',
				content: `New password cannot be shorter than 4 characters`
			};
			pushToast(newToast);
			return;
		}

		let error = await changePassword(oldPassword, newPassword);

		if (error) {
			const newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: 'Password was not updated',
				content: `${error}`
			};
			pushToast(newToast);
			return;
		}

		const newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Password has been updated',
			content: `You have successfully set a new password`
		};
		pushToast(newToast);

		oldPassword = '';
		newPassword = '';
	};
	const handleCancelPassword = async () => {
		oldPassword = '';
		newPassword = '';
	};
</script>

<svelte:head><title>Settings</title></svelte:head>

<h1 in:scale class="text-2xl font-semibold px-4 text-center">Settings</h1>
<div class="w-full m-auto grid grid-flow-col justify-center items-center gap-16 my-8">
	<div in:scale class="bg-white shadow-md rounded-lg outline outline-1 outline-gray-100 p-8 w-96">
		<h2 class="text-xl my-4 font-semibold text-center">Personal Information</h2>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="name"> Name </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
				id="name"
				type="text"
				placeholder="Your Nickname"
				bind:value={newName}
			/>
		</div>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="email"> Email </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
				id="email"
				type="text"
				placeholder="example@domain.com"
				bind:value={newEmail}
			/>
		</div>
		<div class="flex items-center justify-between">
			<button
				class="bg-orange-500 hover:bg-orange-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-all ease-in-out"
				type="button"
				on:click={handleSavePersonalInformation}
			>
				Save
			</button>
			<button
				class="bg-gray-400 hover:bg-gray-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-all ease-in-out"
				type="button"
				on:click={handleCancelPersonalInformation}
			>
				Cancel
			</button>
		</div>
	</div>

	<div in:scale class="bg-white shadow-md rounded-lg outline outline-1 outline-gray-100 p-8 w-96">
		<h2 class="text-xl my-4 font-semibold text-center">Change password</h2>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="oldPassword">
				Old password
			</label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
				id="oldPassword"
				type="password"
				placeholder="*****************"
				bind:value={oldPassword}
			/>
		</div>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="newPassword">
				New password
			</label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
				id="newPassword"
				type="password"
				placeholder="*****************"
				bind:value={newPassword}
			/>
		</div>
		<div class="flex items-center justify-between">
			<button
				class="bg-orange-500 hover:bg-orange-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-all ease-in-out"
				type="button"
				on:click={handleSavePassword}
			>
				Save
			</button>
			<button
				class="bg-gray-400 hover:bg-gray-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-all ease-in-out"
				type="button"
				on:click={handleCancelPassword}
			>
				Cancel
			</button>
		</div>
	</div>
</div>
