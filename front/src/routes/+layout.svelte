<script lang="ts">
import { enUS, esMX } from "@clerk/localizations";
import { shadcn } from "@clerk/themes";
import type { LocalizationResource } from "@clerk/types";
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

let localization = $derived<LocalizationResource>(
	getLocale() === "es"
		? {
				...esMX,
				signUp: {
					legalConsent: {
						checkbox: {
							label__onlyPrivacyPolicy:
								'Estoy de acuerdo con la {{ privacyPolicyLink || link("Política de Privacidad") }}',
							label__onlyTermsOfService:
								'Estoy de acuerdo con los {{ termsOfServiceLink || link("Términos y Condiciones") }}',
							label__termsOfServiceAndPrivacyPolicy:
								'Estoy de acuerdo con los {{ termsOfServiceLink || link("Términos y Condiciones") }} y la {{ privacyPolicyLink || link("Política de Privacidad") }}',
						},
						continue: {
							subtitle: "Por favor, lee y acepta los términos para continuar",
							title: "Consentimiento legal",
						},
					},
				},
			}
		: enUS
);

let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />
<ClerkProvider appearance={{ baseTheme: shadcn, layout: { socialButtonsPlacement: "bottom" } }}  {localization}>
	<Header />
	<main>
		{@render children?.()}
	</main>
	<Footer />
</ClerkProvider>
