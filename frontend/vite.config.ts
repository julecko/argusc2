import devtoolsJson from 'vite-plugin-devtools-json';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
	plugins: [sveltekit(), devtoolsJson()],
    server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:3000',
				changeOrigin: true,
			}
		}
	},
    resolve: {
		alias: {
			$lib: path.resolve('./src/lib'),
			$styles: path.resolve('./src/lib/styles'),
			$components: path.resolve('./src/lib/components'),
            $assets: path.resolve('./src/lib/assets'),
		}
	}
});
