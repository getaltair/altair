import adapter_node from '@sveltejs/adapter-node';
import adapter_static from '@sveltejs/adapter-static';

const target = process.env.BUILD_TARGET || 'web';

const adapter =
	target === 'desktop'
		? adapter_static({
				pages: 'build',
				fallback: 'index.html'
			})
		: adapter_node();

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: { adapter },
	vitePlugin: {
		dynamicCompileOptions: ({ filename }) => ({ runes: !filename.includes('node_modules') })
	}
};

export default config;
