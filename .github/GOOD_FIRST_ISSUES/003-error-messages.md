# Improve Error Messages in API Response Handling

## 📝 Description
Currently, when API requests fail in the frontend, users see generic error messages that don't provide helpful information about what went wrong or how to fix it. We need to improve error handling to show more user-friendly and descriptive error messages that help users understand the issue and potential solutions.

## 🎯 Expected Outcome
API error responses should be parsed and displayed with clear, user-friendly messages instead of raw error codes or generic "Something went wrong" messages.

## ✅ Acceptance Criteria
- [ ] Create a utility function to parse API error responses
- [ ] Map common HTTP status codes to user-friendly messages
- [ ] Handle specific error scenarios (network issues, auth failures, validation errors)
- [ ] Display error messages in a consistent UI component (toast, alert, or modal)
- [ ] Include helpful suggestions when possible (e.g., "Please try again" or "Check your internet connection")
- [ ] Ensure error messages are accessible (proper ARIA labels, screen reader friendly)
- [ ] Test with different error scenarios

## 🔗 Helpful Resources
- Frontend directory: [`front/src/`](../../front/src/)
- Look for API calling functions in `front/src/lib/`
- Existing utility functions in `front/src/lib/utils/`
- TailwindCSS for styling error components
- SvelteKit stores for state management
- Accessibility guidelines for error messages

## 💡 Hints for Implementation

### Finding API Calls
- Search for `fetch()` calls in the frontend codebase
- Look for files handling authentication, data fetching, or form submissions
- Check for existing error handling patterns

### Creating Error Utility
Create a new file like `front/src/lib/utils/errors.ts`:

```typescript
interface ApiError {
  status: number;
  message: string;
  details?: string;
}

const ERROR_MESSAGES: Record<number, string> = {
  401: 'Please sign in to continue',
  403: 'You don\'t have permission to perform this action',
  404: 'The requested resource was not found',
  429: 'Too many requests. Please wait a moment and try again',
  500: 'Server error. Please try again later',
};

export function parseApiError(response: Response, fallbackMessage = 'Something went wrong'): ApiError {
  // Implementation here
}

export function getErrorMessage(error: ApiError): string {
  return ERROR_MESSAGES[error.status] ?? error.message ?? 'An unexpected error occurred';
}
```

### Error Component
Consider creating a reusable error display component:

```svelte
<!-- ErrorMessage.svelte -->
<script>
  let { error, showIcon = true } = $props();
</script>

{#if error}
  <div class="bg-red-50 border border-red-200 rounded-md p-4" role="alert">
    <!-- Error content -->
  </div>
{/if}
```

## 🎨 Common Error Scenarios to Handle

### Authentication Errors (401/403)
- Invalid or expired tokens
- Missing permissions
- Suggest re-authentication

### Network Errors
- No internet connection
- Request timeout
- Server unavailable

### Validation Errors (400)
- Invalid form data
- Missing required fields
- Format errors

### Rate Limiting (429)
- Too many requests
- Suggest waiting and retrying

## 🧪 Testing Your Changes
- Test with different HTTP status codes
- Simulate network failures (disconnect internet)
- Test with invalid authentication tokens
- Verify error messages are displayed correctly
- Check accessibility with screen readers
- Test on mobile devices

## 💡 Implementation Steps
1. **Audit existing error handling** - Find where errors are currently handled
2. **Create error utilities** - Build functions to parse and format errors
3. **Create error UI component** - Design consistent error display
4. **Update API calls** - Apply new error handling to existing fetch calls
5. **Test thoroughly** - Verify all error scenarios work correctly

## 🤝 Getting Help
- Comment if you need help finding specific API calls or error handling code
- Ask about the preferred UI design for error messages
- Request clarification on specific error scenarios to handle
- Join our [discussions](https://github.com/TuentyFaiv/liga-muertos/issues) for general questions
- Tag @TuentyFaiv if you need help with the error handling approach

---

**Labels to apply:** `good first issue`, `component: frontend`, `component: api`, `type: feature`, `priority: high`
**Estimated time:** 3-4 hours
**Skills needed:** TypeScript, Svelte 5 runes and props, API handling, basic UX understanding
