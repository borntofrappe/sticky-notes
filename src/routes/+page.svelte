<script lang="ts">
  import "../app.css";
  import Database from "@tauri-apps/plugin-sql";
  import { getCurrentWebview } from "@tauri-apps/api/webview";
  import { SvelteSet } from "svelte/reactivity";
  import { onMount } from "svelte";

  import { closeNote, deleteNote, newNote } from "./tauri-commands";
  import { DB_PATH, HIGHLIGHT_QUALIFIED_NAME } from "./constants";
  import { setLabelContext } from "./context";

  import Icons from "./Icons.svelte";
  import NoteMenu from "./NoteMenu.svelte";
  import EditorMenu from "./EditorMenu.svelte";

  const { label } = getCurrentWebview();
  setLabelContext(label);
  let options = $state.raw<Set<Command>>(new SvelteSet());
  let editor: HTMLDivElement;
  let pointerdown: boolean = false;
  const timeoutDelay = 200;
  let timeoutID: number;

  onMount(async () => {
    const db = await Database.load(DB_PATH);
    const result = (await db.select("SELECT * FROM notes WHERE label = $1", [
      label,
    ])) as [Note] | [];

    if (result.length === 0) {
      const note: Note = {
        label,
        lastModified: new Date().toString(),
        highlight: "yellow",
        text: "",
      };
      document.documentElement.setAttribute(
        HIGHLIGHT_QUALIFIED_NAME,
        note.highlight
      );

      await db.execute(
        "INSERT into notes (label, lastModified, highlight, text) VALUES ($1, $2, $3, $4)",
        [note.label, note.lastModified, note.highlight, note.text]
      );
    } else {
      const [note] = result;
      document.documentElement.setAttribute(
        HIGHLIGHT_QUALIFIED_NAME,
        note.highlight
      );
      editor.innerHTML = note.text;
    }
  });

  const saveNote = async () => {
    const { innerHTML: text } = editor;
    const lastModified = new Date().toString();

    const db = await Database.load(DB_PATH);
    await db.execute(
      "UPDATE notes SET text = $1, lastModified = $2 WHERE label = $3",
      [text, lastModified, label]
    );
  };

  const oninput = () => {
    clearTimeout(timeoutID);

    timeoutID = setTimeout(() => {
      saveNote();
    }, timeoutDelay);
  };

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

  const onshortcut = async (event: KeyboardEvent) => {
    const { key, ctrlKey } = event;
    if (ctrlKey) {
      switch (key.toLowerCase()) {
        case "n":
          newNote();
          break;
        case "w":
          closeNote();
          break;
        case "d":
          deleteNote(label);
          break;
      }
    }
  };

  const onkeydown = async (event: KeyboardEvent) => {
    const { key, ctrlKey, shiftKey } = event;

    if (
      ["ArrowUp", "ArrowRight", "ArrowDown", "ArrowLeft"].includes(key) ||
      (ctrlKey && key.toLowerCase() === "a")
    ) {
      updateOptions();
      event.stopPropagation();
    } else {
      if (ctrlKey && shiftKey && key.toLowerCase() === "l") {
        event.preventDefault();
        event.stopPropagation();
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
          event.stopPropagation();

          toggleOption(command);
        }
      }
    }
  };
</script>

<svelte:window onkeydown={onshortcut} />

<Icons />

<main class="note">
  <NoteMenu />
  <div
    tabindex="0"
    role="textbox"
    {oninput}
    {onpointerdown}
    {onpointerup}
    {onpointerleave}
    {onpointermove}
    {onkeydown}
    bind:this={editor}
    contenteditable="true"
    class="note--mid note--editor"
    placeholder="Take a note..."
    spellcheck="false"
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
