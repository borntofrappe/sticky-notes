<script>
  import { createNote, closeWindow } from "$lib/tauri-commands";
</script>

<nav data-tauri-drag-region class="notes-list--menu" aria-label="Menu">
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
    onclick={closeWindow}
    aria-label="Close window"
    class="icon-button"
    data-anchor="bottom right"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-close" />
    </svg>
  </button>
</nav>
