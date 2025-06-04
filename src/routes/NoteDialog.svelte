<script lang="ts">
  import { onMount } from "svelte";

  type Props = {
    dialog: HTMLDialogElement;
  };

  const colors: Color[] = [
    "yellow",
    "green",
    "pink",
    "purple",
    "blue",
    "grey",
    "charcoal",
  ];

  let option = $state<Color>("charcoal");
  let timeoutID: number;
  const timeout = 500;

  let { dialog = $bindable() }: Props = $props();

  const updateColor = (color: Color) => {
    // TODO: ACTUALLY DO THE THING
    option = color;

    timeoutID = setTimeout(() => {
      clearTimeout(timeoutID);
      dialog.close();
    }, timeout);
  };

  const listNotes = () => {
    // TODO: ACTUALLY DO THE THING
    dialog.close();
  };

  const deleteNote = () => {
    // TODO: ACTUALLY DO THE THING
    dialog.close();
  };

  onMount(() => {
    return () => {
      clearTimeout(timeoutID);
    };
  });
</script>

<dialog class="note--dialog" bind:this={dialog}>
  <div class="note--dialog--colors">
    {#each colors as color}
      <button
        onclick={() => {
          updateColor(color);
        }}
        aria-label={color}
        data-higlight={color}
        data-active={color === option}
      >
      </button>
    {/each}
  </div>
  <div class="note--dialog--actions">
    <button onclick={listNotes}>
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#icon-list" />
      </svg>
      Notes list
    </button>
    <button onclick={deleteNote}>
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#icon-delete" />
      </svg>
      Delete note
    </button>
  </div>
</dialog>
