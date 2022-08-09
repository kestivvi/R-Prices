<script>
	import { scale } from 'svelte/transition';
</script>

<svelte:head>
	<title>R Prices</title>
</svelte:head>

<!-- <h2 class="mx-2 text-xl my-4 text-center font-semibold">Add new product here</h2>
<AddProduct />
<h3 class="text-3xl my-8 text-center font-semibold">Product List</h3>
<ProductList /> -->

<!-- <input
	class="w-full rounded-md text-lg p-4 border-2 border-gray-200"
	type="text"
	placeholder="Search Your Product"
	bind:value={searchTerm}
/> -->

<a
	in:scale
	href="/"
	class="text-9xl font-semibold my-8 uppercase mb-16 flex justify-center gap-4 text-orange-600 hover:scale-105 transition-all ease-in-out"
	><img src="/rust.png" alt="Rust Logo" id="rust-logo" class="w-32 h-32" />
	<div>Prices</div></a
>

<div in:scale>
	<p class="text-xl text-justify">
		Welcome to <strong class="text-orange-500">R Prices</strong>, a web service where backend was
		built entirely with Rust with usage of such crates as
		<a href="https://crates.io/crates/juniper" class="text-orange-500 font-bold">juniper</a>,
		<a href="https://crates.io/crates/diesel" class="text-orange-500 font-bold">diesel</a>
		or <a href="https://crates.io/crates/actix-web" class="text-orange-500 font-bold">web actix</a>.
		Whereas frontend was built with
		<a href="https://svelte.dev/" class="text-yellow-400 font-bold">svelte</a>, unfortunately. I
		mean... Svelte is my favourite JS framework, but it is not Rust.
	</p>
	<br />
	<p class="text-xl text-justify">
		This web app works as follows. You can create collections of products you found online. Each
		collection can be public or private. Public collection can be found by anyone in the browse
		page. In each collection you can have products. And you can assign urls of online stores' pages
		to each product, so they are tracked and prices are being shown. Go ahead and try.
	</p>
</div>

<style>
	#rust-logo {
		filter: invert(38%) sepia(52%) saturate(4336%) hue-rotate(4deg) brightness(100%) contrast(91%);
	}
</style>
