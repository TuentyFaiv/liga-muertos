<script lang="ts">
import config from "@config";
import { m } from "$lib/paraglide/messages";

import "./footer.css";

let nav = $derived([
	{
		title: m["translation:footer.menu"](),
		links: [
			{
				href: "/",
				label: m["translation:header.home"](),
			},
			{
				href: "/table",
				label: m["translation:header.table"](),
			},
			{
				href: "/seasons",
				label: m["translation:header.seasons"](),
			},
			{
				href: "/tournaments",
				label: m["translation:header.tournaments"](),
			},
			{
				href: "/teams",
				label: m["translation:header.teams"](),
			},
			{
				href: "/players",
				label: m["translation:header.players"](),
			},
			{
				href: "/rules",
				label: m["translation:header.rules"](),
			},
			{
				href: "/contact",
				label: m["translation:header.contact"](),
			},
		],
	},
	{
		title: m["translation:footer.legals"](),
		links: [
			{
				href: "/legals/terms",
				label: m["translation:footer.terms"](),
			},
			{
				href: "/legals/privacy",
				label: m["translation:footer.privacy"](),
			},
		],
	},
]);
</script>

<footer class="footer section">
	<div class="footer__container section__container">
		<a href="/">
			<img src="" alt={config.brand} />
		</a>
		<div class="footer__side">
			{#each nav as { links, title } (title)}
				<ul class="footer__nav">
					<li class="footer__item">
						<h3 class="footer__title">{title}</h3>
					</li>
					<li>
						<ul class="footer__list">
							{#each links as { href, label } (href)}
								<li>
									<a {href}>{label}</a>
								</li>
							{/each}
						</ul>
					</li>
				</ul>
			{/each}
		</div>
	</div>
</footer>
