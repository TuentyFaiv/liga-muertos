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
		class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
	/>
	<Moon
		class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
	/>
	<span class="md:sr-only">{m["translation:header.theme-toggle"]()}</span>
</Button>
