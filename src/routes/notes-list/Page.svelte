<script lang="ts">
  import "./page.css";

  import Database from "@tauri-apps/plugin-sql";

  import { onMount } from "svelte";

  import { DB_PATH } from "$lib/constants";
  import { showWindow } from "$lib/tauri-commands";

  import Menu from "./Menu.svelte";
  import Placeholder from "./Placeholder.svelte";
  import LastModified from "./LastModified.svelte";

  let notes: Note[] = $state([]);

  onMount(async () => {
    const db = await Database.load(DB_PATH);
    notes = (await db.select("SELECT * FROM notes")) as [Note] | [];
  });

  const sort = (notes: Note[]): Note[] => {
    return notes.toSorted((a, b) => {
      const aDate = new Date(a.lastModified);
      const bDate = new Date(b.lastModified);

      return aDate.valueOf() < bDate.valueOf() ? 1 : -1;
    });
  };
</script>

<div class="notes-list">
  <Menu />

  <main class="notes-list--content">
    <h1>Sticky notes</h1>

    {#if notes.length === 0}
      <figure class="notes-list--figure">
        <Placeholder />
        <figcaption>Tap the new note button above to create a note</figcaption>
      </figure>
    {:else}
      {#each sort(notes) as note}
        <div
          role="button"
          aria-label="Open note"
          tabindex="0"
          onkeydown={(event) => {
            if (event.key !== "Enter") return;
            showWindow(note.label);
          }}
          ondblclick={() => {
            showWindow(note.label);
          }}
          class="notes-list--note"
          data-color={note.highlight}
        >
          <LastModified dateString={note.lastModified} />
          <div>{@html note.text}</div>
        </div>
      {/each}
    {/if}
  </main>
</div>
