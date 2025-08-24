<script lang="ts">
import type { Snippet } from "svelte";

import "@auth/styles/layout.css";

let { children }: { children: Snippet } = $props();
</script>

<section class="auth-layout">
	{@render children?.()}
</section>
