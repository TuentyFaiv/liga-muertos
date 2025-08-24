import adapter from "@sveltejs/adapter-vercel";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { mdsvex } from "mdsvex";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	extensions: [".svelte", ".svx"],
	preprocess: [vitePreprocess(), mdsvex()],
	vitePlugin: {
		inspector: true,
	},
	kit: {
		adapter: adapter(),
		csp: {
			mode: "auto",
			directives: {
				"object-src": ["none"],
				"script-src": ["self", "unsafe-inline", "https:"], // "strict-dynamic"],
				"worker-src": ["self", "blob:"],
				// "style-src": ["self", "unsafe-hashes"], // "unsafe-inline"],
				// "style-src-elem": ["self", "unsafe-hashes"], // "unsafe-inline"],
				// "style-src-attr": ["self", "unsafe-hashes"], // "unsafe-inline"],
				"base-uri": ["self"],
			},
			reportOnly: {
				"object-src": ["none"],
				"script-src": ["self", "unsafe-inline", "https:"], // "strict-dynamic"],
				"worker-src": ["self", "blob:"],
				// "style-src": ["self", "unsafe-hashes"], // "unsafe-inline"],
				// "style-src-elem": ["self", "unsafe-hashes"], // "unsafe-inline"],
				// "style-src-attr": ["self", "unsafe-hashes"], // "unsafe
				"base-uri": ["self"],
				"report-to": ["self"],
				"report-uri": ["self"],
			},
		},
		files: {
			lib: "src/logic",
		},
		alias: {
			// Assets
			"@images/*": "src/assets/images/*",
			"@icons/*": "src/assets/icons/*",
			// Logic
			"@config": "src/logic/config.ts",
			"@stores/*": "src/logic/stores/*",
			"@actions/*": "src/logic/actions/*",
			"@schemas/*": "src/logic/schemas/*",
			"@services/*": "src/logic/services/*",
			"@typing/*": "src/logic/typing/*",
			"@utils/*": "src/logic/utils/*",
			// UI Home
			"@home/*": "src/ui/home/*",
			// UI Account
			"@account/*": "src/ui/account/*",
			// UI Auth
			"@auth/*": "src/ui/auth/*",

			/* NEXT_ALIAS */
			// UI Sharing
			"@sharing/*": "src/ui/sharing/*",
			"@styles": "src/ui/sharing/styles/index.ts",
			"@styles/*": "src/ui/sharing/styles/*",
		},
	},
};

export default config;
