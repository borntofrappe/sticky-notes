<script lang="ts">
  import "./page.css";

  import Database from "@tauri-apps/plugin-sql";
  import { emitTo, listen } from "@tauri-apps/api/event";

  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";

  import {
    DB_PATH,
    HIGHLIGHT_QUALIFIED_NAME,
    NOTES_LIST_LABEL,
    NOTES_LIST_EVENT_NAME,
  } from "$lib/constants";
  import { getLabelContext } from "$lib/context";
  import {
    createNote,
    deleteNote,
    closeWindow,
    showWindow,
  } from "$lib/tauri-commands";

  import Menu from "./Menu.svelte";
  import MenuEditor from "./MenuEditor.svelte";

  const label = getLabelContext();
  let options = $state.raw<Set<Command>>(new SvelteSet());
  let editor: HTMLDivElement;
  let pointerdown: boolean = false;
  const saveDebounceDelay = 100;
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

    emitTo(NOTES_LIST_LABEL, NOTES_LIST_EVENT_NAME);
  };

  const oninput = () => {
    clearTimeout(timeoutID);

    timeoutID = setTimeout(() => {
      saveNote();
    }, saveDebounceDelay);
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
    const { key, ctrlKey, altKey } = event;
    if (altKey && key === "F4") {
      closeWindow();
    } else if (ctrlKey) {
      switch (key.toLowerCase()) {
        case "n":
          createNote();
          break;
        case "w":
          closeWindow();
          break;
        case "d":
          deleteNote(label);
          break;
        case "h":
          showWindow(NOTES_LIST_LABEL);
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

<main class="note">
  <Menu />
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
  <MenuEditor
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
