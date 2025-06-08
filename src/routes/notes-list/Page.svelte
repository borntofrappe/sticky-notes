<script lang="ts">
  import Database from "@tauri-apps/plugin-sql";

  import { onMount } from "svelte";

  import { DB_PATH } from "$lib/constants";

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

{JSON.stringify(sort(notes))}
