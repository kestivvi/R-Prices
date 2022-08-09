<script>
	export let msg = '';
</script>

<div class="flex justify-center items-center">
	<div class="w-fit flex flex-col gap-12 justify-center items-center">
		<h1 class="text-5xl font-bold">Error occured</h1>
		<img class="h-48 w-48" src="/lock.png" alt="lock icon" />
		<p class="text-xl font-semibold">{msg}</p>
	</div>
</div>
