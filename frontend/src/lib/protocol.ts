import { encode } from '@msgpack/msgpack';
import type { Types } from './types/communication.types';

export function encodeMsg(params: { type: Types; args: object }): Uint8Array {
	return encode({ t: params.type, a: encode(params.args) });
}
