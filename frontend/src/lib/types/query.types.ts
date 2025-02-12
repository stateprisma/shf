export interface QueryResp {
	entries: FileEntry[];
}

export interface FileEntry {
	name: string;
	size: number;
	type_: FileEntryType;
	last_modified: number;
}

export enum FileEntryType {
	Unknown = 0,
	File = 1,
	Directory = 2,
	Symlink = 3
}
