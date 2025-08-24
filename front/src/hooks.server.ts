import { sequence } from "@sveltejs/kit/hooks";
import { clerkGuard, signinGuard } from "$lib/server/clerk";
import { paraglideGuard } from "$lib/server/i18n";

export const handle = sequence(paraglideGuard, clerkGuard, signinGuard);
