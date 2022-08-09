<script>
	import { pushToast } from '../../stores/toastStore';
	import validator from 'email-validator';
	import { onMount } from 'svelte';
	import { loggedInStatus } from '../../auth_utils';

	let name = '';
	let email = '';
	let password = '';

	let name_remark = '';
	let email_remark = '';
	let password_remark = '';

	onMount(async () => {
		if ($loggedInStatus == true) {
			window.location.href = '/';
			return;
		}
	});

	const validate_form_fields = () => {
		// TODO: name is already used for registration
		if (name == '') {
			name_remark = 'Name cannot be empty!';
			return false;
		} else {
			name_remark = '';
		}

		// TODO: VALIDATE EMAIL
		// TODO: email is already used for registration
		if (!validator.validate(email)) {
			email_remark = 'Email is not valid a email!';
			return false;
		} else {
			email_remark = '';
		}

		// TODO: Password need to be strong enough for registration
		if (password == '') {
			password_remark = 'Password cannot be empty!';
			return false;
		} else {
			password_remark = '';
		}

		return true;
	};

	const register = async () => {
		// VALIDATION
		if (!validate_form_fields()) {
			return;
		}

		// REQUEST
		const body = JSON.stringify({
			name,
			email,
			password
		});

		const response = await fetch('http://127.0.0.1:4000/auth/register', {
			headers: { 'content-type': 'application/json' },
			method: 'POST',
			body: body,
			credentials: 'include'
		});

		// HANDLE RESPONSE AND ERRORS
		if (response.status != 201) {
			console.log(JSON.stringify(await response.json()));
			let newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: "Couldn't register a user",
				content: `${await response.text()}`
			};
			pushToast(newToast);
			return;
		}

		let newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Successfully registered a user',
			content: ``
		};
		pushToast(newToast);

		// REDIRECT
		window.location.href = '/';
	};

	const login = async () => {
		// VALIDATION
		if (!validate_form_fields()) {
			return;
		}

		// REQUEST
		const body = JSON.stringify({
			name,
			email,
			password
		});

		console.log('Want to log in!!');
		const response = await fetch('http://127.0.0.1:4000/auth/login', {
			headers: { 'content-type': 'application/json' },
			method: 'POST',
			body: body,
			credentials: 'include'
		});

		// HANDLE RESPONSE AND ERRORS
		console.log(JSON.stringify(response.headers));

		if (response.status != 200) {
			console.log(JSON.stringify(response));
			let newToast = {
				id: 'id' + new Date().getTime(),
				type: 'error',
				title: "Couldn't login a user",
				content: `${await response.text()}`
			};
			pushToast(newToast);
			return;
		}

		let newToast = {
			id: 'id' + new Date().getTime(),
			type: 'success',
			title: 'Successfully loggend in',
			content: ``
		};
		pushToast(newToast);

		// REDIRECT
		window.location.href = '/';
		// window.history.pushState({}, null, '/');
	};
</script>

<div class="w-full max-w-xs m-auto my-36">
	<form method="post" class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="name"> Name </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline {name_remark !=
				''
					? 'border-red-500'
					: ''}"
				id="name"
				type="text"
				placeholder="Your Nickname"
				bind:value={name}
			/>
			<p class="text-red-500 text-xs italic">{name_remark}</p>
		</div>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="email"> Email </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline {email_remark !=
				''
					? 'border-red-500'
					: ''}"
				id="email"
				type="text"
				placeholder="example@domain.com"
				bind:value={email}
			/>
			<p class="text-red-500 text-xs italic">{email_remark}</p>
		</div>
		<div class="mb-6">
			<label class="block text-gray-700 text-sm font-bold mb-2" for="password"> Password </label>
			<input
				class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline {password_remark !=
				''
					? 'border-red-500'
					: ''}
				"
				id="password"
				type="password"
				placeholder="******************"
				bind:value={password}
			/>
			<p class="text-red-500 text-xs italic">{password_remark}</p>
		</div>
		<div class="flex items-center justify-between">
			<button
				class="bg-orange-500 hover:bg-orange-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline ease-in-out"
				type="button"
				on:click={register}
			>
				Register
			</button>
			<button
				class="bg-orange-500 hover:bg-orange-600 hover:scale-110 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline ease-in-out"
				type="button"
				on:click={login}
			>
				Log In
			</button>
		</div>
	</form>
</div>
