<script lang="ts">
	import type { FileEntry } from '$lib/types/query.types';

	export let descriptor: FileEntry;
</script>

<div class="file-entry symlink">
	<div class="icon">🔗</div>
	<div class="name">{descriptor.name}</div>
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

	.symlink {
		background-color: #f0f0f0;
	}
</style>
