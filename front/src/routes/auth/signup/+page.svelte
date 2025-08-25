<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { SignUp } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

const appearance = {
	elements: {
		rootBox: "auth",
		cardBox: "auth__box",
		formFieldCheckboxLabel: "auth__checkbox-label",
	},
	layout: {
		showOptionalFields: false,
	},
};
</script>

<SEO title={m["auth:signup.seo.title"]()} />

<SignUp signInUrl="/auth/signin" fallbackRedirectUrl="/auth/signup" routing="path" path="/auth/signup" {appearance} />
