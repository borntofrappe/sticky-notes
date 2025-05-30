<script lang="ts">
  import "../app.css";
  import { SvelteSet } from "svelte/reactivity";

  import Icons from "./Icons.svelte";
  import NoteMenu from "./NoteMenu.svelte";
  import EditorMenu from "./EditorMenu.svelte";

  let text = $state("");
  let options = $state.raw<Set<Command>>(new SvelteSet());
  let editor: HTMLDivElement;
  let pointerdown: boolean = false;

  const toggleOption = (option: Command) => {
    document.execCommand(option, false, undefined);

    editor.focus();
    updateOptions();
  };

  const updateOptions = async () => {
    await new Promise((resolve) => setTimeout(resolve, 50)); // delay seems to fix detection, especially following keydown
    let currentCommands = new Set<Command>();
    const commands: Command[] = [
      "bold",
      "italic",
      "underline",
      "strikeThrough",
      "insertUnorderedList",
    ];

    commands.forEach((command) => {
      if (document.queryCommandState(command)) {
        currentCommands.add(command);
      }
    });

    options = currentCommands;
  };

  const insertImage = () => {
    // TODO document.execCommand('insertImage',...)
    editor.focus();
  };

  const onpointerdown = () => {
    pointerdown = true;
    updateOptions();
  };

  const onpointermove = () => {
    if (pointerdown) {
      updateOptions();
    }
  };

  const onpointerleave = () => {
    pointerdown = false;
  };

  const onpointerup = () => {
    pointerdown = false;
  };

  const onkeydown = async (event: KeyboardEvent) => {
    const { key, ctrlKey, shiftKey } = event;

    if (
      ["ArrowUp", "ArrowRight", "ArrowDown", "ArrowLeft"].includes(key) ||
      (ctrlKey && key.toLowerCase() === "a")
    ) {
      updateOptions();
    } else {
      if (ctrlKey && shiftKey && key.toLowerCase() === "l") {
        event.preventDefault();
        toggleOption("insertUnorderedList");
      } else if (ctrlKey) {
        let command: Command | undefined;

        switch (key.toLowerCase()) {
          case "b":
            command = "bold";
            break;
          case "i":
            command = "italic";
            break;
          case "u":
            command = "underline";
            break;
          case "t":
            command = "strikeThrough";
            break;
        }

        if (command) {
          event.preventDefault();
          toggleOption(command);
        }
      }
    }
  };
</script>

<Icons />

<main class="note">
  <NoteMenu />
  <div
    role="application"
    {onpointerdown}
    {onpointerup}
    {onpointerleave}
    {onpointermove}
    {onkeydown}
    bind:this={editor}
    bind:innerHTML={text}
    contenteditable="true"
    class="note--mid note--editor"
    placeholder="Take a note..."
  ></div>
  <EditorMenu
    {options}
    bold={() => {
      toggleOption("bold");
    }}
    italic={() => {
      toggleOption("italic");
    }}
    underline={() => {
      toggleOption("underline");
    }}
    strikeThrough={() => {
      toggleOption("strikeThrough");
    }}
    insertUnorderedList={() => {
      toggleOption("insertUnorderedList");
    }}
    {insertImage}
  />
</main>
