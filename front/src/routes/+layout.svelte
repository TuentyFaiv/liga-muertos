<script lang="ts">
import { enUS, esMX } from "@clerk/localizations";
import { shadcn } from "@clerk/themes";
import { injectAnalytics } from "@vercel/analytics/sveltekit";
import { ModeWatcher } from "mode-watcher";
import type { Snippet } from "svelte";
import { ClerkProvider } from "svelte-clerk";
import { dev } from "$app/environment";
import { getLocale } from "$lib/paraglide/runtime";

import "@styles";

import { Footer } from "@sharing/organisms/footer";
import { Header } from "@sharing/organisms/header";

injectAnalytics({ mode: dev ? "development" : "production" });

let localization = $derived(getLocale() === "es" ? esMX : enUS);

let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />
<ClerkProvider appearance={{ baseTheme: shadcn, layout: { socialButtonsPlacement: "bottom" } }} {localization}>
	<Header />
	<main>
		{@render children?.()}
	</main>
	<Footer />
</ClerkProvider>
