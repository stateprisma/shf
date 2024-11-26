<script lang="ts">
	import Header from '$lib/Header.svelte';
	import { encodeMsg } from '$lib/protocol';
	import { socketManager } from '$lib/socket';
	import { Types } from '$lib/types/communication.types';

	import { onMount } from 'svelte';

	onMount(() => {
		socketManager.connect(`ws://${window.location.host}/connect`);
		socketManager.subscribeConnection((con: boolean) => {
			if (!con) return;
			socketManager.send(
				encodeMsg({
					type: Types.Query,
					args: { path: window.location.search.replace('?query=', '') || '/' }
				})
			);
		});
	});
</script>

<Header />
