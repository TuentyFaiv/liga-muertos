<script lang="ts">
import { enUS, esMX } from "@clerk/localizations";
import { shadcn } from "@clerk/themes";
import { ModeWatcher } from "mode-watcher";
import type { Snippet } from "svelte";
import { ClerkProvider } from "svelte-clerk";
import { getLocale } from "$lib/paraglide/runtime";

import "@styles";

import { Header } from "@sharing/organisms/header";
import { Footer } from "@sharing/organisms/footer";

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
