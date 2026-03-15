import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),
	kit: {
        adapter: adapter(),
        alias: {
            // Custom aliases
            '@lib': path.resolve('./src/lib'),
            '@components': path.resolve('./src/lib/components'),
            '@styles': path.resolve('./src/lib/styles'),
            '@utils': path.resolve('./src/lib/utils')
        }
    }
};

export default config;
