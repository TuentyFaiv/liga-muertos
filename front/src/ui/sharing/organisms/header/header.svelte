<script lang="ts">
import config from "@config";
import { SignedOut } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";

import "./header.css";

import { Language } from "@sharing/atoms/lang";
import { SwitchTheme } from "@sharing/atoms/theme";
import { Button } from "@sharing/molecules/button";
import User from "./user.svelte";

let links = $derived([
	{
		name: m["translation:header.home"](),
		href: "/",
	},
	{
		name: m["translation:header.table"](),
		href: "/table",
	},
	{
		name: m["translation:header.seasons"](),
		href: "/seasons",
	},
	{
		name: m["translation:header.tournaments"](),
		href: "/tournaments",
	},
	{
		name: m["translation:header.teams"](),
		href: "/teams",
	},
	{
		name: m["translation:header.players"](),
		href: "/players",
	},
	{
		name: m["translation:header.rules"](),
		href: "/rules",
	},
	{
		name: m["translation:header.contact"](),
		href: "/contact",
	},
]);
</script>

<header class="header">
	<a href="/">
		<img src="" alt={config.brand} />
	</a>
	<div class="header__side">
		<nav class="header__nav">
			{#each links as { href, name } (href)}
				<a {href}>{name}</a>
			{/each}
		</nav>
		<SignedOut>
			<Language />
			<SwitchTheme />
			<Button href="/auth/signin">
				{m["translation:header.signin"]()}
			</Button>
		</SignedOut>
		<User />
	</div>
</header>
