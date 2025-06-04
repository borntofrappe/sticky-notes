<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  const closeNote = async () => {
    invoke("close_note");
  };

  const newNote = async () => {
    invoke("new_note");
  };
</script>

<nav data-tauri-drag-region class="note--start" aria-label="Menu note">
  <button class="note-button" onclick={newNote} aria-label="New note">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#plus" />
    </svg>
  </button>
  <button class="note-button" aria-label="Menu">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#ellipsis" />
    </svg>
  </button>
  <button class="note-button" onclick={closeNote} aria-label="Close note">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#x" />
    </svg>
  </button>
</nav>
