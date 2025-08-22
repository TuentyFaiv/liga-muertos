<script lang="ts">
import config from "@config";
import { page } from "$app/state";
import { m } from "$lib/paraglide/messages";

import type { Props } from "./seo.proptypes";

let {
	title = null,
	description = m["translation:seo.description"](),
	cover = `${page.url.origin}/images/svelte-welcome.webp`,
	metas = [],
	metaid = "id",
}: Props = $props();

let titleSeo = $derived(title === null ? config.brand : `${title} | ${config.brand}`);
</script>

<svelte:head>
  <title>{titleSeo}</title>
  <meta name="description" content={description} />
  <meta property="twitter:image" content={cover} />
  <meta property="twitter:card" content="summary_large_image" />
  <meta property="twitter:title" content={titleSeo} />
  <meta property="twitter:description" content={description} />
  <meta property="og:image" content={cover} />
  <meta property="og:site_name" content={config.brand} />
  <meta property="og:title" content={titleSeo} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={page.url.toString()} />

  {#each metas as meta (meta[metaid])}
    <meta {...meta} />
  {/each}
</svelte:head>
