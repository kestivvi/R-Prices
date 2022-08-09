<script>
	import AddCollection from '../components/addCollection.svelte';
	import MyCollectionList from '../components/myCollectionList.svelte';
</script>

<svelte:head>
	<title>My Collections</title>
</svelte:head>

<div class="grid grid-cols-1 divide-y">
	<div class="p-4">
		<AddCollection />
	</div>
	<div class="p-4">
		<MyCollectionList />
	</div>
</div>
