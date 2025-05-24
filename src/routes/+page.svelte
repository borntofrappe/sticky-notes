<script lang="ts">
  import "../app.css";
  import Icons from "./Icons.svelte";
</script>

<Icons />

<main class="note">
  <nav class="note--start" aria-label="App menu">
    <!-- new note, options, close note  -->
    <button aria-label="New note" title="New note">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#plus" />
      </svg>
    </button>
    <button aria-pressed="false" aria-label="Menu" title="Menu">
      <svg width="1em" height="1em" viewBox="0 0 1 1">
        <use href="#ellipsis" />
      </svg>
    </button>
    <button aria-label="Close note" title="Close note">
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

<style>
  .note {
    width: 385px;
    height: 400px;
    --button-size-1: 30px;
    --button-size-2: 35px;
    --border-radius: 10px;
    --ui-padding: 8px;
    --button-padding: 7px;

    --color-text-1: #ededed;
    --color-text-2: #c1c1c1;
    --color-surface-1: #333333;
    --color-surface-2: #454545;

    color: var(--color-text-1);
    background: var(--color-surface-1);
    border-radius: var(--border-radius);
    display: flex;
    flex-direction: column;

    > .note--mid {
      flex-grow: 1;
    }
  }

  .note--start {
    --button-size: var(--button-size-2);
    border-start-start-radius: var(--border-radius);
    border-start-end-radius: var(--border-radius);
    color: var(--color-text-2);
    background: var(--color-surface-2);
    display: flex;
    align-items: center;

    & *:first-child {
      margin-inline-end: auto;
      border-start-start-radius: inherit;
    }
    & *:last-child {
      border-start-end-radius: inherit;
    }
  }

  .note--end {
    color: var(--color-text-2);
    --button-size: var(--button-size-1);
    padding: var(--ui-padding);
    border-end-start-radius: var(--border-radius);
    border-end-end-radius: var(--border-radius);
    border-block-start: 1px solid var(--color-surface-2);
  }

  textarea {
    font-size: 1.05em;
    border: none;
    color: inherit;
    background: none;
    resize: none;
    padding: var(--ui-padding);
    line-height: 1.5;
    scrollbar-width: thin;
    scrollbar-color: var(--color-surface-2) transparent;

    &:focus {
      outline: none;
    }

    &::placeholder {
      color: var(--color-text-2);
    }
  }

  button {
    cursor: pointer;
    --_button-size: var(--button-size, 26px);
    width: var(--_button-size);
    height: var(--_button-size);
    padding: var(--button-padding, 6px);
    border: none;
    background: none;
    color: inherit;

    & > svg {
      display: block;
      width: 100%;
      height: 100%;
    }

    &:hover {
      background: oklch(from currentColor l c h / 0.1);
    }

    position: relative;
    /* will work on pressed buttons */
    /*
    &[aria-pressed="true"]::before {
      content: "";
      position: absolute;
      inset: 0;
      background: oklch(from currentColor l c h / 0.1);
    }
     */
  }
</style>
