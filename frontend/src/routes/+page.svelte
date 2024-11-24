<script lang="ts">
	import Header from '$lib/Header.svelte';
	import { socketManager } from '$lib/socket';
	import { onMount } from 'svelte';

	onMount(() => {
		socketManager.connect(`ws://${window.location.host}/connect`);
	});
</script>

<Header />
