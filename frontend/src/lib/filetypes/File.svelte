<script lang="ts">
	import { goto } from '$app/navigation';
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

	const getFileExtension = (filename: string): string | undefined => {
		return filename.split('.').pop();
	};

	const handleOnClick = () => {
		switch (getFileExtension(descriptor.name)) {
			case 'mp4':
			case 'webm':
			case 'ogv':
			case 'avi':
			case 'mkv':
			case 'mov':
				goto(`/video?query=${getQueryParam()}${descriptor.name}`);
				break;
			default:
				break;
		}
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="file-entry file" on:click={() => handleOnClick()}>
	<div class="icon">📄</div>
	<div class="name">{descriptor.name}</div>
	{#if descriptor.size}
		<div class="size">{descriptor.size} bytes</div>
	{/if}
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

	.size {
		font-size: 12px;
		margin-left: auto;
		color: #777;
	}

	.modif {
		font-size: 14px;
		color: #757575;
		margin-left: auto;
	}

	.file {
		background-color: #d4f4dd;
	}
</style>
