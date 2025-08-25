<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { m } from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

import ES from "./es.svx";
import EN from "./en.svx";

let spanish = $derived(getLocale() === "es");
</script>

<SEO title={m["legals:terms.seo.title"]()} />

{#if spanish}
	<ES />
{:else}
	<EN />
{/if}
