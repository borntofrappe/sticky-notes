<script lang="ts">
  import "../app.css";
  import Icons from "./Icons.svelte";
  import NoteMenu from "./NoteMenu.svelte";
  import EditorMenu from "./EditorMenu.svelte";
</script>

<Icons />

<main class="note">
  <NoteMenu />
  <div
    contenteditable="true"
    class="note--mid note--editor"
    placeholder="Take a note..."
  ></div>
  <EditorMenu />
</main>
