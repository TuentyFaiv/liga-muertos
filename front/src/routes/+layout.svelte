<script lang="ts">
import { ModeWatcher } from "mode-watcher";

import "@styles";

import { Header } from "@sharing/organisms/header";

let { children } = $props();
</script>

<ModeWatcher />
<Header />
{@render children?.()}
