<script lang="ts">
import { SEO } from "@sharing/atoms/seo";
import { m } from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import EN from "./en.svx";
import ES from "./es.svx";

let spanish = $derived(getLocale() === "es");
</script>

<SEO title={m["legals:terms.seo.title"]()} />

{#if spanish}
	<ES />
{:else}
	<EN />
{/if}
