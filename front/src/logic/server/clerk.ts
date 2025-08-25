import type { Handle } from "@sveltejs/kit";
import { redirect } from "@sveltejs/kit";
import { withClerkHandler } from "svelte-clerk/server";

export const clerkGuard = withClerkHandler();

export const signinGuard: Handle = async ({ event, resolve }) => {
	const { userId } = event.locals.auth();

	if (
		(userId && event.url.pathname.includes("/auth")) ||
		(!userId && event.url.pathname.includes("/account"))
	) {
		redirect(303, "/");
	}

	return resolve(event);
};
