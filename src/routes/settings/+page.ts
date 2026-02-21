import type { PageLoad } from './$types';
import tauriStore from '@utils/tauri-store';

export const load = (async () => {
	const store = await tauriStore.getAll();

	return { store };
}) satisfies PageLoad;
