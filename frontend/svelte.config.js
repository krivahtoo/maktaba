import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // See https://svelte.dev/docs/kit/adapters for more information about adapters.
    adapter: adapter({
      fallback: '404.html'
    }),
    paths: {
      relative: false,
    },
    alias: {
      '@/*': './src/lib/*'
    }
  }
};

export default config;
