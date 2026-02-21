import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: { adapter: adapter(), alias: { '@blocks': 'src/@blocks', '@utils': 'src/@utils' } },
	preprocess: [mdsvex()],
	extensions: ['.svelte', '.svx']
};

export default config;
