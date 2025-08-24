<script lang="ts">
import { SignIn } from "svelte-clerk";
</script>

<SignIn signUpUrl="/auth/signup" routing="path" path="/auth/signin" appearance={{
	elements: {
		rootBox: "auth"
	}
}} />
