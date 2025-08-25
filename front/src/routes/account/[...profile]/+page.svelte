<script lang="ts">
import { mount, onMount, unmount } from "svelte";
import { UserProfile } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

import type { CustomPage } from "@clerk/types";
import type { PageProps } from "./$types";

import "@account/styles/page.css";

import { CollectionIcon, collection } from "@account/organisms/collection";
import { PersonalizationIcon, personalization } from "@account/organisms/personalization";
import { ProfileIcon, profile } from "@account/organisms/profile";
import { Portal } from "@sharing/atoms/portal";
import { SEO } from "@sharing/atoms/seo";

let { data }: PageProps = $props();

let customPages = $state<CustomPage[]>([]);

onMount(() => {
	let page: Record<string, unknown>;

	customPages.push({
		url: "player",
		label: m["account:tabs.player"](),
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
		label: m["account:tabs.personalization"](),
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
		label: m["account:tabs.collection"](),
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

<SEO title={m[`account:${data.page}.seo.title`]?.()} />

<UserProfile {customPages} path="/account" routing="path" appearance={{
	elements: {
		rootBox: "account",
		cardBox: "account__box",
	}
}} />
