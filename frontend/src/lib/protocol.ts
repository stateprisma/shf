import { decode, encode } from '@msgpack/msgpack';
import { Types, type CommonResp, type Responses } from './types/communication.types';
import type { QueryResp } from './types/query.types';

export function encodeMsg(params: { type: Types; args: object }): Uint8Array {
	return encode({ t: params.type, a: encode(params.args) });
}

export function decodeMsg(msg: Uint8Array): [Types | null, Responses | null] {
	const cresp: CommonResp = decode(msg) as CommonResp;
	switch (cresp.t) {
		case Types.Error:
			return [Types.Error, null];
		case Types.Query:
			return [Types.Query, decode(cresp.a) as QueryResp];
	}

	return [null, null];
}
