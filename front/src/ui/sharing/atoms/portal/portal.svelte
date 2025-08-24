<script lang="ts">
import type { Component, Snippet } from "svelte";

const { children }: { children: Snippet<[]> } = $props();
</script>

{@render children()}
