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
  <button onclick={bold} aria-pressed={options.has("bold")} aria-label="Bold">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#bold" />
    </svg>
  </button>
  <button
    onclick={italic}
    aria-pressed={options.has("italic")}
    aria-label="Italic"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#italic" />
    </svg>
  </button>
  <button
    onclick={underline}
    aria-pressed={options.has("underline")}
    aria-label="Underline"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#underline" />
    </svg>
  </button>
  <button
    onclick={strikeThrough}
    aria-pressed={options.has("strikeThrough")}
    aria-label="Strikethrough"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#strikethrough" />
    </svg>
  </button>
  <button
    onclick={insertUnorderedList}
    aria-pressed={options.has("insertUnorderedList")}
    aria-label="Toggle bullets"
  >
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#list" />
    </svg>
  </button>
  <button onclick={insertImage} aria-label="Add image">
    <svg width="1em" height="1em" viewBox="0 0 1 1">
      <use href="#image" />
    </svg>
  </button>
</nav>
