<script lang="ts">
import { format } from "@formkit/tempo";
import { getLocale } from "$lib/paraglide/runtime";

import type { Props } from "./formatter-date.proptypes";

import "./formatter-date.css";

let { datetime, formatter, empty, tiny = false }: Props = $props();

let date = $derived(datetime ? new Date(datetime) : null);
let locale = $derived(getLocale());
</script>

<span class="formatterdate" class:tiny>
  {#if date}
    {format(date, formatter ?? "MMM D, YYYY HH:mm", locale)}
  {:else}
    {empty}
  {/if}
</span>
