<script lang="ts">
  type Props = {
    options: Set<Command>;
    bold: () => void;
    italic: () => void;
    underline: () => void;
    strikeThrough: () => void;
    insertUnorderedList: () => void;
    insertImage: () => void;
  };

  let {
    options,
    bold,
    italic,
    underline,
    strikeThrough,
    insertUnorderedList,
    insertImage,
  }: Props = $props();
</script>

<nav class="note--end" aria-label="Menu editor">
  <button
    class="note-button"
    onclick={bold}
    aria-pressed={options.has("bold")}
    aria-label="Bold"
    data-anchor="top left"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-bold" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={italic}
    aria-pressed={options.has("italic")}
    aria-label="Italic"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-italic" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={underline}
    aria-pressed={options.has("underline")}
    aria-label="Underline"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-underline" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={strikeThrough}
    aria-pressed={options.has("strikeThrough")}
    aria-label="Strikethrough"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-strikethrough" />
    </svg>
  </button>
  <button
    class="note-button"
    onclick={insertUnorderedList}
    aria-pressed={options.has("insertUnorderedList")}
    aria-label="Toggle bullets"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-unordered-list" />
    </svg>
  </button>
  <button class="note-button" onclick={insertImage} aria-label="Add image">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#icon-image" />
    </svg>
  </button>
</nav>
