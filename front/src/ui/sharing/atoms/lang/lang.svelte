<script lang="ts">
import { m } from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";

import "./lang.css";

import { Languages } from "@lucide/svelte";
import { buttonVariants } from "@sharing/molecules/button";
import * as DropdownMenu from "@sharing/molecules/dropdown-menu";

// let { mobile = false } = $props();

let showES = $derived(getLocale() === "es");
let showEN = $derived(getLocale() === "en");

function onChange() {
	const spanish = getLocale() === "es";
	setLocale(spanish ? "en" : "es");
}
</script>

<!-- {#if !mobile}
  <DropdownMenu.Sub>
    <DropdownMenu.SubTrigger class="min-h-[3.25rem] text-lg leading-5">
      <Languages class="size-5 mr-3" />
      <span>{m["translation:header.language"]()}</span>
    </DropdownMenu.SubTrigger>
    <DropdownMenu.SubContent>
      <DropdownMenu.CheckboxItem checked={showES} onCheckedChange={onChange}>
        <span>Español</span>
      </DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem checked={showEN} onCheckedChange={onChange}>
        <span>English</span>
      </DropdownMenu.CheckboxItem>
    </DropdownMenu.SubContent>
  </DropdownMenu.Sub>
{:else} -->
  <DropdownMenu.Root>
    <DropdownMenu.Trigger
      class={buttonVariants({
        variant: "ghost",
        size: "icon",
        class: "w-auto min-h-0 justify-center size-10 rounded-full"
      })}
    >
      <Languages className="size-5" />
      <span class="sr-only">{m["translation:header.language"]()}</span>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
      <DropdownMenu.CheckboxItem checked={showES} onCheckedChange={onChange}>
        <span>Español</span>
      </DropdownMenu.CheckboxItem>
      <DropdownMenu.CheckboxItem checked={showEN} onCheckedChange={onChange}>
        <span>English</span>
      </DropdownMenu.CheckboxItem>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
<!-- {/if} -->
