import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";
import { emitTo } from "@tauri-apps/api/event";

import {
  DB_PATH,
  HIGHLIGHT_QUALIFIED_NAME,
  NOTES_LIST_LABEL,
  NOTES_LIST_EVENT_NAME,
} from "./constants";

export const createNote = async () => {
  const documentHighlight = document.documentElement.getAttribute(
    HIGHLIGHT_QUALIFIED_NAME
  );

  const note: Note = {
    label: `note-${crypto.randomUUID()}`,
    lastModified: new Date().toString(),
    highlight: documentHighlight ? (documentHighlight as Color) : "yellow",
    text: "",
    open: true,
  };

  const db = await Database.load(DB_PATH);
  await db.execute(
    "INSERT into notes (label, lastModified, highlight, text, open) VALUES ($1, $2, $3, $4, $5)",
    [note.label, note.lastModified, note.highlight, note.text, note.open]
  );

  invoke("create_note", {
    label: note.label,
  });

  emitTo(NOTES_LIST_LABEL, NOTES_LIST_EVENT_NAME);
};

export const deleteNote = async (label: string) => {
  const db = await Database.load(DB_PATH);
  await db.execute("DELETE FROM notes WHERE label = $1", [label]);

  invoke("delete_note");

  emitTo(NOTES_LIST_LABEL, NOTES_LIST_EVENT_NAME);
};

export const closeWindow = async (label: string) => {
  if (label === NOTES_LIST_LABEL) {
    invoke("close_window");
  } else {
    const db = await Database.load(DB_PATH);
    const [note] = (await db.select("SELECT * FROM notes WHERE label = $1", [
      label,
    ])) as [Note];

    if (note.text.trim() == "") {
      deleteNote(label);
    } else {
      await db.execute("UPDATE notes SET open = false WHERE label = $1", [
        label,
      ]);
      invoke("close_window");
      emitTo(NOTES_LIST_LABEL, NOTES_LIST_EVENT_NAME);
    }
  }
};

export const showWindow = async (label: string) => {
  if (label !== NOTES_LIST_LABEL) {
    const db = await Database.load(DB_PATH);
    await db.execute("UPDATE notes SET open = true WHERE label = $1", [label]);
  }

  invoke("show_window", {
    label,
  });

  emitTo(NOTES_LIST_LABEL, NOTES_LIST_EVENT_NAME);
};
