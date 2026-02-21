export const ssr = false;
export const prerender = true;

import type { LayoutLoad } from './$types';
import tauriStore from '@utils/tauri-store';

export const load = (async () => {
	const store = await tauriStore.getAll();

	if (Object.keys(store).length === 0) {
		await tauriStore.setDefaults();
		return { store: await tauriStore.getAll() };
	}

	return { store };
}) satisfies LayoutLoad;
