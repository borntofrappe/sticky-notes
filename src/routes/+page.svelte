<script lang="ts">
  import "../app.css";

  import { getCurrentWebview } from "@tauri-apps/api/webview";

  import { setLabelContext } from "$lib/context";

  import Icons from "./Icons.svelte";
  import Note from "./note/Page.svelte";

  const { label } = getCurrentWebview();
  setLabelContext(label);
</script>

<Icons />

{#if label === "notes-list"}
  <!-- notes list -->
{:else}
  <Note />
{/if}
