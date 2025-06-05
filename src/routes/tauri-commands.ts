import { invoke } from "@tauri-apps/api/core";

export const closeNote = async () => {
  invoke("close_note");
};

export const newNote = async () => {
  invoke("create_note");
};

export const deleteNote = async () => {
  invoke("delete_note");
};
