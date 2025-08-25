<script lang="ts">
  import type { Snippet } from "svelte";

  import "@legals/styles/layout.css";

  let { children }: { children: Snippet } = $props();
</script>

<section class="legals section">
	<div class="legals__container section__container">
		<div class="legals__text">
			{@render children?.()}
		</div>
	</div>
</section>
