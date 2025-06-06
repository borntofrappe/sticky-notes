import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";
import { DB_PATH, HIGHLIGHT_QUALIFIED_NAME } from "./constants";

export const closeNote = async () => {
  invoke("close_note");
};

export const newNote = async () => {
  const documentHighlight = document.documentElement.getAttribute(
    HIGHLIGHT_QUALIFIED_NAME
  );

  const note: Note = {
    label: `note-${crypto.randomUUID()}`,
    lastModified: new Date().toString(),
    highlight: documentHighlight ? (documentHighlight as Color) : "yellow",
    text: "",
  };

  const db = await Database.load(DB_PATH);
  await db.execute(
    "INSERT into notes (label, lastModified, highlight, text) VALUES ($1, $2, $3, $4)",
    [note.label, note.lastModified, note.highlight, note.text]
  );

  invoke("create_note", {
    label: note.label,
  });
};

export const deleteNote = async (label: string) => {
  const db = await Database.load(DB_PATH);

  await db.execute("DELETE FROM notes WHERE label = $1", [label]);
  invoke("delete_note");
};
