import type { QueryResp } from './query.types';

export enum Types {
	Error = 0,
	Query = 1
}

export interface Header {
	t: Types;
	a: Uint8Array;
}

export type Responses = QueryResp;
