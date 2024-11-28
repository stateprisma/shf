<script lang="ts">
	import { encodeMsg } from '$lib/protocol';
	import { socketManager } from '$lib/socket';
	import { Types } from '$lib/types/communication.types';
	import type { FileEntry } from '$lib/types/query.types';
	import { formatDate } from '$lib/utils';

	export let descriptor: FileEntry;

	function updateQueryParam(newQueryValue: string) {
		if (window) {
			const newUrl =
				window.location.protocol +
				'//' +
				window.location.host +
				window.location.pathname +
				`?query=${newQueryValue}`;

			window.history.pushState({ path: newUrl }, '', newUrl);
		}
	}

	function getQueryParam(): string {
		if (window) {
			return window.location.search.replace('?query=', '') || '/';
		} else {
			return '/';
		}
	}

	const handleOnClick = () => {
		if (descriptor.name === '..') {
			updateQueryParam(`${getQueryParam().split('/').slice(0, -1).join('/') || '/'}`);
		} else {
			updateQueryParam(
				`${window.location.search.replace('?query=', '') || '/'}/${descriptor.name}`.replace(
					'//',
					'/'
				)
			);
		}
		socketManager.send(
			encodeMsg({
				type: Types.Query,
				args: { path: getQueryParam() }
			})
		);
	};
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="file-entry directory" on:click={() => handleOnClick()}>
	<div class="icon">📁</div>
	<div class="name">{descriptor.name}</div>
	<div class="modif">{formatDate(descriptor.last_modified)}</div>
</div>

<style>
	.file-entry {
		display: flex;
		align-items: center;
		padding: 10px;
		margin: 5px;
		background-color: #f5f5f5;
		border-radius: 5px;
		width: 100%;
		box-sizing: border-box;
		cursor: pointer;
		transition: background-color 0.3s;
	}

	.file-entry:hover {
		background-color: #e2e2e2;
	}

	.icon {
		font-size: 24px;
		margin-right: 10px;
	}

	.name {
		font-size: 16px;
		font-weight: bold;
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.modif {
		font-size: 14px;
		color: #757575;
		margin-left: auto;
	}

	.directory {
		background-color: #e2f7ff;
	}
</style>
