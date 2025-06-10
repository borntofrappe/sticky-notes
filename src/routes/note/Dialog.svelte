<script lang="ts">
  import Database from "@tauri-apps/plugin-sql";
  import { emitTo } from "@tauri-apps/api/event";

  import { onMount } from "svelte";

  import {
    DB_PATH,
    HIGHLIGHT_QUALIFIED_NAME,
    NOTE_LIST_LABEL,
    NOTES_LIST_EVENT_NAME,
  } from "$lib/constants";
  import { getLabelContext } from "$lib/context";
  import { deleteNote, showWindow } from "$lib/tauri-commands";

  type Props = {
    dialog: HTMLDialogElement;
  };

  const label = getLabelContext();

  const colors: Color[] = [
    "yellow",
    "green",
    "pink",
    "purple",
    "blue",
    "grey",
    "charcoal",
  ];

  let option = $state<Color>("yellow");
  const timeout = 500;

  let { dialog = $bindable() }: Props = $props();

  onMount(async () => {
    const db = await Database.load(DB_PATH);
    const result = (await db.select("SELECT * FROM notes WHERE label = $1", [
      label,
    ])) as [Note] | [];

    if (result.length !== 0) {
      option = result[0].highlight;
    }
  });

  const updateColor = async (color: Color) => {
    option = color;
    document.documentElement.setAttribute(HIGHLIGHT_QUALIFIED_NAME, color);

    const db = await Database.load(DB_PATH);
    await db.execute("UPDATE notes SET highlight = $1 WHERE label = $2", [
      color,
      label,
    ]);

    emitTo(NOTE_LIST_LABEL, NOTES_LIST_EVENT_NAME);

    await new Promise((resolve) => setTimeout(resolve, timeout));
    dialog.close();
  };

  const listNotes = () => {
    showWindow(NOTE_LIST_LABEL);
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
    <button
      onclick={() => {
        deleteNote(label);
      }}
    >
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#icon-delete" />
      </svg>
      Delete note
    </button>
  </div>
</dialog>
