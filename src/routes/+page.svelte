<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import "../app.css";
  import Icons from "./Icons.svelte";

  const closeNote = async () => {
    invoke("close_note");
  };

  const newNote = async () => {
    invoke("new_note");
  };
</script>

<Icons />

<main class="note">
  <nav data-tauri-drag-region class="note--start" aria-label="App menu">
    <!-- new note, options, close note  -->
    <button onclick={newNote} aria-label="New note" title="New note">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#plus" />
      </svg>
    </button>
    <button aria-pressed="false" aria-label="Menu" title="Menu">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#ellipsis" />
      </svg>
    </button>
    <button onclick={closeNote} aria-label="Close note" title="Close note">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#x" />
      </svg>
    </button>
  </nav>
  <textarea class="note--mid" placeholder="Take a note..."></textarea>
  <nav class="note--end" aria-label="App menu">
    <button aria-pressed="false" aria-label="Bold" title="Bold">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#bold" />
      </svg>
    </button>
    <button aria-pressed="false" aria-label="Italic" title="Italic">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#italic" />
      </svg>
    </button>
    <button aria-pressed="false" aria-label="Underline" title="Underline">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#underline" />
      </svg>
    </button>
    <button
      aria-pressed="false"
      aria-label="Strikethrough"
      title="Strikethrough"
    >
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#strikethrough" />
      </svg>
    </button>
    <button
      aria-pressed="false"
      aria-label="Toggle bullets"
      title="Toggle Bullets"
    >
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#list" />
      </svg>
    </button>
    <button aria-label="Add image" title="Add Image">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#image" />
      </svg>
    </button>
  </nav>
</main>
