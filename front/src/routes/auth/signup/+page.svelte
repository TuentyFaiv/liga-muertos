<script lang="ts">
import { SignUp } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

import { SEO } from "@sharing/atoms/seo";
</script>

<SEO title={m["auth:signup.seo.title"]()} />

<SignUp signInUrl="/auth/signin" routing="path" path="/auth/signup" appearance={{
	elements: {
		rootBox: "auth"
	}
}} />
