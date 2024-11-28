<script lang="ts">
	import Header from '$lib/Header.svelte';
	import ListView from '$lib/FileListView.svelte';
	import { decodeMsg, encodeMsg } from '$lib/protocol';
	import { socketManager } from '$lib/socket';
	import { Types } from '$lib/types/communication.types';

	import { onMount } from 'svelte';
	import type { FileEntry } from '$lib/types/query.types';

	let entries: Array<FileEntry> = [];

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

		socketManager.subscribeMessages((msg: Uint8Array) => {
			let [type, resp] = decodeMsg(msg);
			if (resp === null) {
				console.error('Error while decoding incoming response');
				return;
			}

			switch (type) {
				case Types.Query:
					console.log(resp.entries);
					entries = resp.entries;
			}
		});
	});
</script>

<Header />
<ListView {entries} />
