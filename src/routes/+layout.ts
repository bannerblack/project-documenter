export const ssr = false;
export const prerender = true;

import { invoke } from "@tauri-apps/api/core";
import type { Projects } from "@utils/types";

import type { LayoutLoad } from './$types';
import tauriStore from '@utils/tauri-store';

export const load = (async () => {
	// Get preferences from Tauri store
	let store = await tauriStore.getAll();

	if (Object.keys(store).length === 0) {
		await tauriStore.setDefaults();
		store = await tauriStore.getAll();
	}

	// Get projects from backend
	let projects : Projects[] = [];

	try {
		projects = await invoke("scan_project_folders", { paths: store.projectDirectories});
	} catch (error) {
		console.error("Failed to scan project folders. Please check your configuration and try again.", error);
	}

	return { store, projects };
}) satisfies LayoutLoad;
