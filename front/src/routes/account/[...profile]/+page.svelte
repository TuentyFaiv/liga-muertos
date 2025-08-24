<script lang="ts">
import type { CustomPage } from "@clerk/types";
import { mount, onMount, unmount } from "svelte";
import { UserProfile } from "svelte-clerk";

import "@account/styles/page.css";

import { CollectionIcon, collection } from "@account/organisms/collection";
import { PersonalizationIcon, personalization } from "@account/organisms/personalization";
import { ProfileIcon, profile } from "@account/organisms/profile";
import { Portal } from "@sharing/atoms/portal";

let customPages = $state<CustomPage[]>([]);

onMount(() => {
	let page: Record<string, unknown>;

	customPages.push({
		url: "player",
		label: "Perfil de jugador",
		mountIcon: (el) => {
			el.innerHTML = ProfileIcon;
		},
		unmountIcon: (el) => {
			if (el) {
				el.innerHTML = "";
			}
		},
		mount: (el) => {
			page = mount(Portal, {
				target: el,
				props: { children: profile },
			});
		},
		unmount: (el) => {
			if (el && page) {
				unmount(page);
			}
		},
	});
	customPages.push({
		url: "personalization",
		label: "Personalización",
		mountIcon: (el) => {
			el.innerHTML = PersonalizationIcon;
		},
		unmountIcon: (el) => {
			if (el) {
				el.innerHTML = "";
			}
		},
		mount: (el) => {
			page = mount(Portal, {
				target: el,
				props: { children: personalization },
			});
		},
		unmount: (el) => {
			if (el && page) {
				unmount(page);
			}
		},
	});
	customPages.push({
		url: "collection",
		label: "Colección",
		mountIcon: (el) => {
			el.innerHTML = CollectionIcon;
		},
		unmountIcon: (el) => {
			if (el) {
				el.innerHTML = "";
			}
		},
		mount: (el) => {
			page = mount(Portal, {
				target: el,
				props: { children: collection },
			});
		},
		unmount: (el) => {
			if (el && page) {
				unmount(page);
			}
		},
	});
	customPages.push({
		label: "security",
	});
});
</script>

<UserProfile {customPages} path="/account" routing="path" appearance={{
	elements: {
		rootBox: "account",
		cardBox: "account__box",
	}
}} />
