<script lang="ts">
	import Directory from './filetypes/Directory.svelte';
	import File from './filetypes/File.svelte';
	import Symlink from './filetypes/Symlink.svelte';
	import { FileEntryType, type FileEntry } from './types/query.types';

	let adjustedEntries: FileEntry[] = $state([]);
	let { entries } = $props();

	function getQueryParam(): string {
		if (typeof window !== 'undefined') {
			return window.location.search.replace('?query=', '') || '/';
		} else {
			return '/';
		}
	}

	$effect(() => {
		adjustedEntries = [
			...(getQueryParam() !== '/'
				? [{ name: '..', size: 0, type_: FileEntryType.Directory, last_modified: 0 }]
				: []),
			...entries
		];
	});
</script>

{#each adjustedEntries as entry}
	{#if entry.type_ === FileEntryType.File}
		<File descriptor={entry} />
	{:else if entry.type_ === FileEntryType.Directory}
		<Directory descriptor={entry} />
	{:else if entry.type_ === FileEntryType.Symlink}
		<Symlink descriptor={entry} />
	{:else if entry.type_ === FileEntryType.Unknown}
		<h1>ERROR: UNKNOWN</h1>
	{/if}
{/each}
