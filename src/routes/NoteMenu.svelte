<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import NoteDialog from "./NoteDialog.svelte";

  const closeNote = async () => {
    invoke("close_note");
  };

  const newNote = async () => {
    invoke("create_note");
  };

  let dialog: HTMLDialogElement;
  const openMenu = () => {
    dialog.showModal();
  };
</script>

<NoteDialog bind:dialog />

<nav data-tauri-drag-region class="note--start" aria-label="Menu note">
  <button class="note-button" onclick={newNote} aria-label="New note">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-add" />
    </svg>
  </button>
  <button class="note-button" onclick={openMenu} aria-label="Menu">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-ellipsis" />
    </svg>
  </button>
  <button class="note-button" onclick={closeNote} aria-label="Close note">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-close" />
    </svg>
  </button>
</nav>
