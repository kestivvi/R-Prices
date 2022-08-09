<script>
	import { slide } from 'svelte/transition';
	import { onDestroy, onMount } from 'svelte';
	import { toastStore } from '../stores/toastStore';

	export let toast;

	let terminationTime;
	let toBeTerminated = 5000;
	$: isMouseHover = false;

	onMount(async () => {
		setTimeout(update, 10);
		prolongLife();
	});

	const prolongLife = () => {
		terminationTime = new Date().getTime() + 5000;
	};

	const update = () => {
		if (isMouseHover) {
			prolongLife();
		}

		let now = new Date().getTime();
		toBeTerminated = (terminationTime - now) / 1000;
		toBeTerminated = toBeTerminated < 0 ? 0 : toBeTerminated;
		toBeTerminated = toBeTerminated.toFixed(1);

		if (now >= terminationTime) {
			close();
		} else {
			setTimeout(update, 10);
		}
	};

	const close = () => {
		toastStore.update((data) => data.filter((t) => t.id != toast.id));
	};
</script>

<div
	transition:slide
	class="text-white rounded-lg w-fit h-fit m-4 p-3 flex flex-row items-stretch justify-between relative z-40
	{toast.type == 'success'
		? 'bg-green-500'
		: toast.type == 'warning'
		? 'bg-yellow-500'
		: toast.type == 'error'
		? 'bg-red-500'
		: 'bg-gray-200 text-black'}"
	on:mousemove={() => {
		isMouseHover = true;
		prolongLife();
	}}
	on:mouseenter={() => (isMouseHover = true)}
	on:mouseout={() => (isMouseHover = false)}
	on:blur={() => (isMouseHover = false)}
>
	<!-- {#if toast.type == 'success'}
		<h3 class="text-lg uppercase">Success</h3>
	{/if} -->
	<div>
		<h1 class="font-bold">{toast.title}</h1>
		<p>{toast.content}</p>
	</div>
	<div class="flex flex-col justify-between items-center ml-3">
		<button
			on:click={close}
			class="bg-gray-100 rounded-full text-gray-800 border-none w-5 h-5 text-xs">X</button
		>
		<p class="text-xs text-right">{toBeTerminated} [s]</p>
	</div>
</div>
