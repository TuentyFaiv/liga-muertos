<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { SignIn } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";
</script>

<SEO title={m["auth:seo.title"]()} />

<SignIn signUpUrl="/auth/signup" routing="path" path="/auth/signin" appearance={{
	elements: {
		rootBox: "auth"
	}
}} />
