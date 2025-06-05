<script lang="ts">
  import Database from "@tauri-apps/plugin-sql";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { onMount } from "svelte";
  import { deleteNote } from "./tauri-commands";

  type Props = {
    dialog: HTMLDialogElement;
  };

  const colors: Color[] = [
    "yellow",
    "green",
    "pink",
    "purple",
    "blue",
    "grey",
    "charcoal",
  ];

  let option = $state<Color>("charcoal");
  const timeout = 500;

  let { dialog = $bindable() }: Props = $props();

  const updateColor = async (color: Color) => {
    option = color;
    document.documentElement.setAttribute("data-highlight", color);

    const { label } = await getCurrentWebview();
    const db = await Database.load("sqlite:notes.db");
    await db.execute("UPDATE notes SET highlight = $1 WHERE label = $2", [
      color,
      label,
    ]);

    await new Promise((resolve) => setTimeout(resolve, timeout));
    dialog.close();
  };

  const listNotes = () => {
    // TODO: ACTUALLY DO THE THING
    dialog.close();
  };
</script>

<dialog class="note--dialog" bind:this={dialog}>
  <div class="note--dialog--colors">
    {#each colors as color}
      <button
        onclick={() => {
          updateColor(color);
        }}
        aria-label={color}
        data-active={color === option}
        data-color={color}
      >
      </button>
    {/each}
  </div>
  <div class="note--dialog--actions">
    <button onclick={listNotes}>
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#icon-list" />
      </svg>
      Notes list
    </button>
    <button onclick={deleteNote}>
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#icon-delete" />
      </svg>
      Delete note
    </button>
  </div>
</dialog>
