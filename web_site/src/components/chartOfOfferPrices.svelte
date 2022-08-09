<script>
	import { onMount } from 'svelte';
	import { currentProductStore } from '../stores/productStore';
	import ApexCharts from 'apexcharts?client';
	window.ApexCharts = ApexCharts;

	////////////////////////////////////////////////////////////////////////
	/// UTILS

	// function uniq(a) {
	// 	return a.sort().filter(function (item, pos, ary) {
	// 		return !pos || item != ary[pos - 1];
	// 	});
	// }

	// function formatDate(d) {
	// 	return d.getFullYear() + '-' + (d.getMonth() + 1) + '-' + d.getDate();
	// }

	// https://coolors.co/palette/e63232-f3722c-f8961e-ffd043-7fc96b-43aa8b-277da1-3b498e-66418a
	const COLORS = [
		'#E63232',
		'#F3722C',
		'#F8961E',
		'#FFD043',
		'#7FC96B',
		'#43AA8B',
		'#277DA1',
		'#3B498E',
		'#66418A'
	];

	// let colorsToRandom = [...COLORS];
	// function generateColor() {
	// 	if (colorsToRandom.length == 0) {
	// 		colorsToRandom = [...COLORS];
	// 	}
	// 	let index = Math.floor(Math.random() * colorsToRandom.length);
	// 	return colorsToRandom.splice(index)[0];
	// }

	const getArrayOfShuffledColors = () => {
		let array = [...COLORS];
		for (let i = array.length - 1; i > 0; i--) {
			const j = Math.floor(Math.random() * (i + 1));
			[array[i], array[j]] = [array[j], array[i]];
		}
		return array;
	};

	////////////////////////////////////////////////////////////////////////
	/// DATA

	const getSeriesData = () => {
		let series = $currentProductStore.offers.map((offer) => {
			let data = offer.prices.map((priceObj) => {
				let value =
					priceObj.value == 0 || priceObj.value == null ? null : priceObj.value.toFixed(2);
				return [new Date(priceObj.createdAt * 1000), value];
			});
			return {
				name: offer.site,
				data
			};
		});
		return series;
	};

	function walkAndGetLowestOnEachDay(array) {
		let newArray = [];
		let currentLowest = array[0][1];

		for (let i = 1; i < array.length; i++) {
			// New date
			if (array[i - 1][0].getDate() != array[i][0].getDate()) {
				newArray.push([array[i - 1][0], currentLowest ? parseFloat(currentLowest) : null]);
				currentLowest = array[i][1];
			}
			if (currentLowest == null && array[i][1] != null) {
				currentLowest = array[i][1];
			}
			// In the middle
			else if (parseFloat(array[i][1]) < parseFloat(currentLowest)) {
				currentLowest = array[i][1];
			}
		}
		return newArray;
	}

	const getMinSeriesData = () => {
		let data = $currentProductStore.offers.map((offer) => {
			let data = offer.prices.map((priceObj) => {
				let value =
					priceObj.value == 0 || priceObj.value == null ? null : priceObj.value.toFixed(0);
				let date = new Date(priceObj.createdAt * 1000);
				date.setHours(0, 0, 0, 0);
				return [date, value];
			});
			return {
				name: offer.site,
				data
			};
		});
		data = data.map((serie) => serie.data).flat();
		data.sort((a, b) => b[0] - a[0]);
		data = walkAndGetLowestOnEachDay(data);
		data = [[new Date(), null], ...data];
		return [{ name: 'ds', data }];
	};

	let sevenDaysAgo = new Date();
	sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);

	////////////////////////////////////////////////////////////////////////
	/// MOUNTING

	onMount(() => {
		const options = {
			series: getSeriesData(),
			chart: {
				id: 'chartPrices',
				type: 'line',
				height: 400
				// toolbar: {
				// 	autoSelected: 'pan',
				// 	show: false
				// }
			},
			colors: getArrayOfShuffledColors(),
			stroke: {
				width: 3,
				curve: 'stepline'
			},
			dataLabels: {
				enabled: false
			},
			fill: {
				opacity: 1
			},
			markers: {
				size: 0
			},
			xaxis: {
				type: 'datetime'
			},
			tooltip: {
				x: {
					show: true,
					format: 'dd MMM yyyy | HH:mm',
					formatter: undefined
				}
			}
		};

		const chartPrices = new ApexCharts(document.querySelector('#chart-prices'), options);
		chartPrices.render();

		// const optionsTimeline = {
		// 	series: getMinSeriesData(),
		// 	chart: {
		// 		id: 'chartTimeline',
		// 		height: 130,
		// 		type: 'area',
		// 		brush: {
		// 			target: 'chartPrices',
		// 			enabled: true
		// 		},
		// 		selection: {
		// 			enabled: true,
		// 			xaxis: {
		// 				min: sevenDaysAgo.getTime(),
		// 				max: new Date().getTime()
		// 			}
		// 		}
		// 	},
		// 	stroke: {
		// 		curve: 'stepline'
		// 	},
		// 	colors: ['#008FFB'],
		// 	fill: {
		// 		type: 'gradient',
		// 		gradient: {
		// 			opacityFrom: 0.91,
		// 			opacityTo: 0.1
		// 		}
		// 	},
		// 	xaxis: {
		// 		type: 'datetime',
		// 		tooltip: {
		// 			enabled: false
		// 		}
		// 	},
		// 	yaxis: {
		// 		tickAmount: 2
		// 	}
		// };

		// const chartTimeline = new ApexCharts(
		// 	document.querySelector('#chart-timeline'),
		// 	optionsTimeline
		// );
		// chartTimeline.render();
	});
</script>

<div class="p-1">
	<div id="chart-prices" />
	<div id="chart-timeline" />
</div>
