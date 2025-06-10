<script lang="ts">
  import "../app.css";

  import { getCurrentWebview } from "@tauri-apps/api/webview";

  import { NOTES_LIST_LABEL } from "$lib/constants";
  import { setLabelContext } from "$lib/context";

  import Icons from "./Icons.svelte";
  import Note from "./note/Page.svelte";
  import NotesList from "./notes-list/Page.svelte";

  const { label } = getCurrentWebview();
  setLabelContext(label);
</script>

<Icons />

{#if label === NOTES_LIST_LABEL}
  <NotesList />
{:else}
  <Note />
{/if}
