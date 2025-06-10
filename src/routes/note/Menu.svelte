<script lang="ts">
  import { getLabelContext } from "$lib/context";
  import { createNote, closeWindow } from "$lib/tauri-commands";

  import NoteDialog from "./Dialog.svelte";

  const label = getLabelContext();

  let dialog: HTMLDialogElement;
  const openMenu = () => {
    dialog.showModal();
  };
</script>

<NoteDialog bind:dialog />

<nav data-tauri-drag-region class="note--start" aria-label="Menu note">
  <button
    onclick={createNote}
    aria-label="New note"
    class="icon-button"
    data-anchor="bottom left"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-add" />
    </svg>
  </button>
  <button
    onclick={openMenu}
    aria-label="Menu"
    class="icon-button"
    data-anchor="bottom"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-ellipsis" />
    </svg>
  </button>
  <button
    onclick={() => {
      closeWindow(label);
    }}
    aria-label="Close note"
    class="icon-button"
    data-anchor="bottom right"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-close" />
    </svg>
  </button>
</nav>
