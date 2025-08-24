<script lang="ts">
import { Moon, Sun } from "@lucide/svelte";
import { Button } from "@sharing/molecules/button";
import { toggleMode } from "mode-watcher";
import { MediaQuery } from "svelte/reactivity";
import { m } from "$lib/paraglide/messages";

const desktop = new MediaQuery("(min-width: 768px)");
</script>

<Button onclick={toggleMode} variant="ghost" size={desktop.current ? "icon" : "default"}
class="w-full min-h-[3.25rem] text-lg leading-5 justify-start md:w-auto md:min-h-0 md:justify-center gap-3 md:size-10 md:rounded-full"
>
	<Sun
		class="size-5 rotate-0 opacity-100 transition-all dark:-rotate-90 dark:opacity-0"
	/>
	<Moon
		class="absolute size-5 rotate-90 opacity-0 transition-all dark:rotate-0 dark:opacity-100"
	/>
	<span class="md:sr-only">{m["translation:header.theme-toggle"]()}</span>
</Button>
