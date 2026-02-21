import { load, type Store } from '@tauri-apps/plugin-store';

// ── Store configuration ────────────────────────────────────────────────────
const STORE_PATH = 'store.json';

export type StoreType = {
	projectDirectories: string[];
	displayReadmeContent: boolean;
};

const STORE_DEFAULTS: Record<string, unknown> = {
	projectDirectories: [],
	displayReadmeContent: true
};
// ──────────────────────────────────────────────────────────────────────────

class TauriStore {
	private storePromise: Promise<Store>;

	constructor(path: string = STORE_PATH, defaults: Record<string, unknown> = STORE_DEFAULTS) {
		this.storePromise = load(path, { autoSave: true, defaults });
	}

	async set(key: string, value: unknown): Promise<void> {
		const store = await this.storePromise;
		await store.set(key, value);
		await store.save();
	}

	async get<T>(key: string): Promise<T | null> {
		const store = await this.storePromise;
		return (await store.get<T>(key)) ?? null;
	}

	async delete(key: string): Promise<void> {
		const store = await this.storePromise;
		await store.delete(key);
	}

	async save(): Promise<void> {
		const store = await this.storePromise;
		await store.save();
	}

	async getAll(): Promise<StoreType> {
		const store = await this.storePromise;
		const entries = await store.entries();
		return Object.fromEntries(entries) as StoreType;
	}

	async setDefaults(): Promise<void> {
		const store = await this.storePromise;
		for (const [key, value] of Object.entries(STORE_DEFAULTS)) {
			if ((await store.get(key)) === undefined) {
				await store.set(key, value);
			}
		}
		await store.save();
	}
}

const tauriStore = new TauriStore();

export { TauriStore };
export default tauriStore;
