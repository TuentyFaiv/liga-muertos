<script lang="ts">
import { SignUp } from "svelte-clerk";
</script>

<SignUp signInUrl="/auth/signin" routing="path" path="/auth/signup" appearance={{
	elements: {
		rootBox: "auth"
	}
}} />
