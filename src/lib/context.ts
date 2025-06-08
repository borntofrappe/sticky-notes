import { getContext, setContext } from "svelte";

const key = Symbol("label");

export function setLabelContext(label: string) {
  setContext(key, label);
}

export function getLabelContext() {
  return getContext(key) as string;
}
