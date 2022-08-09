<script>
	import { onMount } from 'svelte';
	import { fetchStatus } from '../auth_utils';

	import LoginNav from '../components/loginNav.svelte';
	import Navbar from '../components/navbar.svelte';
	import ToastStack from '../components/toastStack.svelte';
	onMount(async () => {
		await fetchStatus();
	});
</script>

<Navbar />

<div class="p-8 max-w-6xl mx-auto">
	<slot />
	<ToastStack />
</div>

<style>
	@tailwind base;
	@tailwind components;
	@tailwind utilities;
</style>
