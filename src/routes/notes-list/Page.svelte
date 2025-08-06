<script lang="ts">
  import "./page.css";

  import Database from "@tauri-apps/plugin-sql";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";

  import { onDestroy, onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { flip } from "svelte/animate";

  import {
    DB_PATH,
    NOTES_LIST_EVENT_NAME,
  } from "$lib/constants";
  import { getLabelContext } from "$lib/context";
  import { createNote, showWindow, closeWindow } from "$lib/tauri-commands";

  import Menu from "./Menu.svelte";
  import Placeholder from "./Placeholder.svelte";
  import LastModified from "./LastModified.svelte";

  const label = getLabelContext();

  let notes: Note[] = $state([]);
  let unlisten: UnlistenFn;

  const DURATIONS = {
    in: 500,
    out: 300,
    animate: (d: number): number => Math.sqrt(d * 700),
  };

  onMount(async () => {
    update();
    unlisten = await listen(NOTES_LIST_EVENT_NAME, update);
  });

  const update = async () => {
    const db = await Database.load(DB_PATH);
    notes = sort(await db.select("SELECT * FROM notes")) as [Note] | [];
  };

  onDestroy(() => {
    unlisten();
  });

  const sort = (notes: Note[]): Note[] => {
    return notes.toSorted((a, b) => {
      const aDate = new Date(a.lastModified);
      const bDate = new Date(b.lastModified);

      return aDate.valueOf() < bDate.valueOf() ? 1 : -1;
    });
  };

  const onshortcut = async (event: KeyboardEvent) => {
    const { key, ctrlKey, altKey } = event;
    if (altKey && key === "F4") {
      closeWindow(label);
    } else if (ctrlKey) {
      switch (key.toLowerCase()) {
        case "n":
          createNote();
          break;
        case "w":
          closeWindow(label);
          break;
      }
    }
  };
</script>

<svelte:window onkeydown={onshortcut} />

<div class="notes-list">
  <Menu />

  <main class="notes-list--content">
    <h1>Sticky Notes</h1>

    {#if notes.length === 0}
      <figure
        in:fade={{ duration: DURATIONS.in, delay: DURATIONS.out }}
        class="notes-list--figure"
      >
        <Placeholder />
        <figcaption>Tap the new note button above to create a note</figcaption>
      </figure>
    {:else}
      {#each notes as note (note.label)}
        <div
          in:fade|global={{ duration: DURATIONS.in }}
          out:fade|global={{ duration: DURATIONS.out }}
          animate:flip={{ duration: (d) => DURATIONS.animate(d) }}
          role="button"
          aria-label="Open note"
          tabindex="0"
          onkeydown={(event) => {
            if (["Enter", " "].includes(event.key)) {
              event.preventDefault();
              showWindow(note.label);
            }
          }}
          ondblclick={() => {
            showWindow(note.label);
          }}
          class="notes-list--note"
          class:open={note.open}
          data-color={note.highlight}
        >
          <LastModified dateString={note.lastModified} />
          <div>{@html note.text}</div>
        </div>
      {/each}
    {/if}
  </main>
</div>
