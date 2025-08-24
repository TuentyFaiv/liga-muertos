
<script lang="ts">
import {
	Bell,
	CircleDollarSign,
	CircleUserRound,
	Gamepad2,
	Languages,
	Moon,
	Sun,
} from "@lucide/svelte";
import { toggleMode } from "mode-watcher";
import { SignedIn, UserButton } from "svelte-clerk";
import { m } from "$lib/paraglide/messages";
import { getLocale, setLocale } from "$lib/paraglide/runtime";

import "./header.user.css";

import { goto } from "$app/navigation";
import { page } from "$app/state";

function onChangeLang() {
	const spanish = getLocale() === "es";
	setLocale(spanish ? "en" : "es");
}
</script>

<SignedIn>
	<UserButton showName userProfileUrl="/account" userProfileMode="navigation" afterSignOutUrl="/" appearance={{
		elements: {
			userButtonPopoverActionItemButtonIcon: "user__action-icon",
		}
	}}>
		<UserButton.MenuItems>
			<UserButton.Action label={m["translation:header.wealth"]()} onclick={() => {
				console.log('open wealth');
			}}>
				{#snippet labelIcon()}
					<CircleDollarSign size={16} />
				{/snippet}
			</UserButton.Action>
			<UserButton.Action label={m["translation:header.profile"]()} onclick={() => goto("/account/player")}>
				{#snippet labelIcon()}
					<CircleUserRound size={16} />
				{/snippet}
			</UserButton.Action>
			{#if page.data.initialState.session?.checkAuthorization({
				permission: "teams:view_own"
			})}
				<UserButton.Action label={m["translation:header.team"]()} onclick={() => goto("/account/team")}>
					{#snippet labelIcon()}
						<Gamepad2 size={16} />
					{/snippet}
				</UserButton.Action>
			{/if}
			<UserButton.Action label={m["translation:header.notifications"]()} onclick={() => {
				console.log('open notifications');
			}}>
				{#snippet labelIcon()}
					<Bell size={16} />
				{/snippet}
			</UserButton.Action>
			<UserButton.Action label={m["translation:header.change-theme"]()} onclick={toggleMode}>
				{#snippet labelIcon()}
					<Sun
						class="user__icon-theme"
					/>
					<Moon
						class="user__icon-theme user__icon-theme--dark"
					/>
				{/snippet}
			</UserButton.Action>
			<UserButton.Action label={m["translation:header.change-language"]()} onclick={onChangeLang}>
				{#snippet labelIcon()}
					<Languages size={16} />
				{/snippet}
			</UserButton.Action>
		</UserButton.MenuItems>
	</UserButton>
</SignedIn>
