/// <reference types="svelte-clerk/env" />

import type { buildClerkProps } from "svelte-clerk/server";

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces

declare global {
	// biome-ignore lint/style/noNamespace: Necessary for SvelteKit types
	namespace App {
		// interface Error {}
		// interface Locals {}
		interface PageData {
			initialState: ReturnType<typeof buildClerkProps>["initialState"];
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
