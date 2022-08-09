<script>
	import LoginLogoutButton from './loginLogoutButton.svelte';
	import { page } from '$app/stores';
	import { loggedInStatus } from '../auth_utils';

	let browse_selected;
	let my_collections_selected;
	let settings_selected;

	$: {
		browse_selected = $page.url.pathname == '/browse';
		my_collections_selected = $page.url.pathname == '/my-collections';
		settings_selected = $page.url.pathname == '/settings';
	}
</script>

<div class="h-12 bg-neutral-100 shadow-md flex flex-row gap-8 justify-between items-center mb-8">
	<a
		href="/"
		class="text-xl font-bold uppercase flex justify-center gap-2 text-orange-600 hover:scale-105 transition-all ease-in-out px-4"
		><img src="/rust.png" alt="Rust Logo" id="rust-logo" class="w-7 h-7" />
		<div>Prices</div></a
	>
	<div class="flex flex-row gap-8">
		<a
			href="/browse"
			class="font-semibold hover:scale-110 transition-all ease-in-out 
            {browse_selected ? 'text-orange-600 font-bold' : ''}">Browse</a
		>
		{#if $loggedInStatus}
			<a
				href="/my-collections"
				class="font-semibold hover:scale-110 transition-all ease-in-out
					{my_collections_selected ? 'text-orange-600 font-bold' : ''}">My Collections</a
			>
			<a
				href="/settings"
				class="font-semibold hover:scale-110 transition-all ease-in-out
					{settings_selected ? 'text-orange-600 font-bold' : ''}">Settings</a
			>
		{/if}
	</div>
	<div class="px-4">
		<LoginLogoutButton />
	</div>
</div>

<style>
	#rust-logo {
		filter: invert(38%) sepia(52%) saturate(4336%) hue-rotate(4deg) brightness(100%) contrast(91%);
	}
</style>
