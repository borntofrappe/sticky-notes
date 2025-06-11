<script lang="ts">
  type Props = {
    options: Set<Command>;
    bold: () => void;
    italic: () => void;
    underline: () => void;
    strikeThrough: () => void;
    insertUnorderedList: () => void;
  };

  let {
    options,
    bold,
    italic,
    underline,
    strikeThrough,
    insertUnorderedList,
  }: Props = $props();
</script>

<nav class="note--end" aria-label="Menu editor">
  <button
    onclick={bold}
    aria-pressed={options.has("bold")}
    aria-label="Bold"
    class="icon-button"
    data-anchor="top left"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-bold" />
    </svg>
  </button>
  <button
    onclick={italic}
    aria-pressed={options.has("italic")}
    aria-label="Italic"
    class="icon-button"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-italic" />
    </svg>
  </button>
  <button
    onclick={underline}
    aria-pressed={options.has("underline")}
    aria-label="Underline"
    class="icon-button"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-underline" />
    </svg>
  </button>
  <button
    onclick={strikeThrough}
    aria-pressed={options.has("strikeThrough")}
    aria-label="Strikethrough"
    class="icon-button"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-strikethrough" />
    </svg>
  </button>
  <button
    onclick={insertUnorderedList}
    aria-pressed={options.has("insertUnorderedList")}
    aria-label="Toggle bullets"
    class="icon-button"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-unordered-list" />
    </svg>
  </button>
</nav>
