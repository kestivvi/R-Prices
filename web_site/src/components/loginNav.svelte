<script>
	import { onMount } from 'svelte';
	import { loggedInStatus, log_me_out } from '../auth_utils';
	import { fade } from 'svelte/transition';

	const logout = async () => {
		await log_me_out();
		window.location.href = '/';
	};
</script>

<nav
	class="absolute top-0 left-0 p-4 flex flex-col font-semibold hover:scale-110 transition-all ease-in-out"
>
	{#if $loggedInStatus == true}
		<button class="text-blue-500" on:click={logout}>Logout</button>
	{:else}
		<a class="text-blue-500" href="/auth/login">Login/Register</a>
	{/if}
</nav>
