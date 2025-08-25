<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { SignIn } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

const appearance = {
	elements: {
		rootBox: "auth",
		cardBox: "auth__box",
	},
};
</script>

<SEO title={m["auth:seo.title"]()} />

<SignIn signUpUrl="/auth/signup" fallbackRedirectUrl="/auth/signin" routing="path" path="/auth/signin" {appearance} />
