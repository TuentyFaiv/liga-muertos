<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { onMount } from "svelte";
import { SignUp, useClerkContext } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

const ctx = useClerkContext();

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

onMount(() => {
	ctx.clerk?.handleRedirectCallback({
		continueSignUpUrl: "/auth/signup/continue",
		signInUrl: "/auth/signin",
		signUpUrl: "/auth/signup",
		reloadResource: "signUp",
	});
});
</script>

<SEO title={m["auth:signup.seo.title"]()} />

<SignUp signInUrl="/auth/signin" routing="path" {appearance} path="/auth/signup" />
