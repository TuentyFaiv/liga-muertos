<script module>
import { Button } from "@sharing/molecules/button";
import { defineMeta } from "@storybook/addon-svelte-csf";
import { fn } from "storybook/test";

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
const { Story } = defineMeta({
	title: "Example/Button",
	component: Button,
	tags: ["autodocs"],
	argTypes: {
		type: {
			control: { type: "select" },
			options: ["button", "submit", "reset"],
			description: "The type of the button",
			defaultValue: "button",
		},
		size: {
			control: { type: "select" },
			options: ["default", "sm", "lg", "icon"],
			description: "The size of the button",
			defaultValue: "default",
		},
		variant: {
			control: { type: "select" },
			options: ["link", "default", "destructive", "outline", "secondary", "ghost"],
			description: "The variant of the button",
			defaultValue: "default",
		},
		disabled: {
			control: { type: "boolean" },
		},
		className: {
			control: { type: "text" },
		},
	},
	args: {
		onclick: fn(),
	},
});
</script>

<!-- More on writing stories with args: https://storybook.js.org/docs/writing-stories/args -->
<Story name="Primary">
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story
	name="Secondary"
	args={{ variant: "secondary" }}
>
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story
	name="Destructive"
	args={{ variant: "destructive" }}
>
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story
	name="Link"
	args={{ variant: "link" }}
>
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story
	name="Outline"
	args={{ variant: "outline" }}
>
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story
	name="Ghost"
	args={{ variant: "ghost" }}
>
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story name="Large" args={{ size: "lg" }} >
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>

<Story name="Small" args={{ size: "sm" }} >
	{#snippet template(args)}
    <Button {...args}>Click me</Button>
  {/snippet}
</Story>
