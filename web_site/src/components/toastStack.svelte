<script>
	import { toastStore } from '../stores/toastStore';
	import Toast from './toast.svelte';
</script>

<div class="fixed w-fit h-fit right-0 bottom-0 flex flex-col justify-end items-end z-40">
	<!-- <button on:click={() => console.log(`toastStore: ${JSON.stringify($toastStore)}`)}
		>Pokaż TostStore</button
	> -->
	{#each $toastStore.reverse() as toast (toast.id)}
		<Toast {toast} />
	{/each}
</div>
