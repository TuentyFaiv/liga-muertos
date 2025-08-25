<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { SignUp } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";
</script>

<SEO title={m["auth:signup.seo.title"]()} />

<SignUp signInUrl="/auth/signin" routing="path" path="/auth/signup" appearance={{
	elements: {
		rootBox: "auth"
	}
}} />
