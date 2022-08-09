<script>
	// IMPORTS
	import { scale } from 'svelte/transition';
	import OutClick from 'svelte-outclick';

	// PROPERTIES
	export let actions = [];
	// actions = [{name: "action_name_1", callback: fn1}, {...}, {...}];

	// LOCAL VARIABLES
	let showMenu = false;
</script>

<div class="relative flex justify-center items-center">
	<!-- 3 dots with options -->
	<button on:click={() => (showMenu = true)}>
		<img class="w-6 h-6 opacity-40 hover:opacity-80" src="/options.png" alt="options" />
	</button>

	<!-- Options menu pop up -->
	{#if showMenu}
		<OutClick on:outclick={() => (showMenu = false)}>
			<div
				transition:scale
				class="absolute top-5 left-0 z-10 bg-slate-100 rounded-md px-2  outline outline-1 outline-gray-200 text-md font-semibold grid grid-cols-1 divide-y p-1 w-max"
			>
				{#each actions as action (action.name)}
					<button
						on:click={action.callback}
						class="p-1 transition-all hover:scale-110 opacity-80 hover:opacity-100"
					>
						{action.name}
					</button>
				{/each}
			</div>
		</OutClick>
	{/if}
</div>
