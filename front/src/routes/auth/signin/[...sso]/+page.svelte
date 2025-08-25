<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { onMount } from "svelte";
import { SignIn, useClerkContext } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

const ctx = useClerkContext();

const appearance = {
	elements: {
		rootBox: "auth",
		cardBox: "auth__box",
		formFieldCheckboxLabel: "auth__checkbox-label",
	},
};

onMount(() => {
	ctx.clerk?.handleRedirectCallback({
		continueSignUpUrl: "/auth/signin/continue",
		signInUrl: "/auth/signin",
		signUpUrl: "/auth/signup",
		reloadResource: "signIn",
	});
});
</script>

<SEO title={m["auth:seo.title"]()} />

<SignIn signInUrl="/auth/signin" routing="path" {appearance} path="/auth/signin" />
