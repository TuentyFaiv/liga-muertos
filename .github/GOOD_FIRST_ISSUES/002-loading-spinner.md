# Add Loading Spinner to Authentication Button

## 📝 Description
Currently, when users click the authentication button to sign in with Clerk/Twitch, there's no visual feedback to indicate that the authentication process is in progress. This can lead to users clicking multiple times or thinking the button is broken. Adding a loading spinner will improve the user experience by providing clear feedback during the authentication flow.

## 🎯 Expected Outcome
The authentication button should display a loading spinner and be disabled while the authentication request is being processed.

## ✅ Acceptance Criteria
- [ ] Add a loading state to the authentication button component
- [ ] Display a spinner icon when authentication is in progress
- [ ] Disable the button during loading to prevent multiple clicks
- [ ] Change button text to "Signing in..." or similar during loading
- [ ] Spinner should use consistent styling with the rest of the application
- [ ] Loading state should be cleared after authentication completes (success or error)
- [ ] Component should be responsive and work on mobile devices

## 🔗 Helpful Resources
- Frontend directory: [`front/src/`](../../front/src/)
- Look for authentication-related components in `front/src/lib/components/`
- TailwindCSS documentation for spinner animations
- Lucide Svelte icons (already included): `@lucide/svelte`
- SvelteKit reactivity documentation for managing loading state

## 💡 Hints for Implementation

### Finding the Right Component
- Look for authentication/login related components in `front/src/lib/components/`
- Search for Clerk integration files
- The button might be in a layout file or auth-specific component

### Technical Approach
1. Add a `loading` state variable using `$state()` to track the authentication state
2. Import a spinner icon from `@lucide/svelte` (e.g., `Loader2`)
3. Use Svelte's conditional rendering with `{#if}` blocks to show/hide the spinner
4. Apply TailwindCSS classes for styling and animations
5. Use the `disabled` attribute when loading is true and `onclick` instead of `on:click`

### Example Structure
```svelte
<script>
  import { Loader2 } from '@lucide/svelte';

  let loading = $state(false);

  async function handleAuth() {
    loading = true;
    try {
      // Authentication logic here
    } finally {
      loading = false;
    }
  }
</script>

<button
  onclick={handleAuth}
  disabled={loading}
  class="..."
>
  {#if loading}
    <Loader2 class="animate-spin" />
    Signing in...
  {:else}
    Sign in with Twitch
  {/if}
</button>
```

## 🎨 Design Considerations
- Use a small spinner (16x16 or 20x20 pixels)
- Apply `animate-spin` TailwindCSS class for rotation
- Consider the button's existing color scheme
- Ensure good contrast for accessibility
- Keep the button the same size to prevent layout shift

## 🧪 Testing Your Changes
- Test the loading state manually by clicking the auth button
- Verify the button is properly disabled during loading
- Check responsive behavior on different screen sizes
- Test with slow network conditions (you can simulate this in browser dev tools)

## 🤝 Getting Help
- Comment on this issue if you can't find the authentication component
- Ask for clarification about the expected visual design
- Join our [discussions](https://github.com/TuentyFaiv/liga-muertos/discussions) for general questions about SvelteKit
- Tag @TuentyFaiv if you need help locating the right files

---

**Labels to apply:** `good first issue`, `component: frontend`, `component: auth`, `type: feature`, `priority: medium`
**Estimated time:** 2-3 hours
**Skills needed:** Basic HTML/CSS, Svelte 5 runes and reactivity, TailwindCSS
