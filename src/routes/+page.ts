import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load = (async () => {
	return { };
}) satisfies PageLoad;
