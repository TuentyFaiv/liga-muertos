import type { Preview } from "@storybook/sveltekit";

import "../src/ui/sharing/styles/app.css";

const preview: Preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i,
			},
		},
		a11y: {
			context: "body",
			config: {},
			options: {
				runOnly: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "best-practice", "wcag2aaa"],
			},
		},
	},
};

export default preview;
