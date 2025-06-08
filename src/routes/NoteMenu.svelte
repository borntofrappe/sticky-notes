<script lang="ts">
  import { closeNote, newNote } from "$lib/tauri-commands";
  
  import NoteDialog from "./NoteDialog.svelte";

  let dialog: HTMLDialogElement;
  const openMenu = () => {
    dialog.showModal();
  };
</script>

<NoteDialog bind:dialog />

<nav data-tauri-drag-region class="note--start" aria-label="Menu note">
  <button
    class="note-button"
    onclick={newNote}
    aria-label="New note"
    data-anchor="bottom left"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-add" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={openMenu}
    aria-label="Menu"
    data-anchor="bottom"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-ellipsis" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={closeNote}
    aria-label="Close note"
    data-anchor="bottom right"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-close" />
    </svg>
  </button>
</nav>
