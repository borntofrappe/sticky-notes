<script lang="ts">
  import "../app.css";

  import { getCurrentWebview } from "@tauri-apps/api/webview";

  import { NOTE_LIST_LABEL } from "$lib/constants";
  import { setLabelContext } from "$lib/context";

  import Icons from "./Icons.svelte";
  import Note from "./note/Page.svelte";

  const { label } = getCurrentWebview();
  setLabelContext(label);
</script>

<Icons />

{#if label === NOTE_LIST_LABEL}
  <!-- notes list -->
{:else}
  <Note />
{/if}
