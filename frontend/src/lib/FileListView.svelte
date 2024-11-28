<script lang="ts">
	import Directory from './filetypes/Directory.svelte';
	import File from './filetypes/File.svelte';
	import Symlink from './filetypes/Symlink.svelte';
	import { FileEntryType, type FileEntry } from './types/query.types';

	export let entries: Array<FileEntry>;
</script>

{#each entries as entry}
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
