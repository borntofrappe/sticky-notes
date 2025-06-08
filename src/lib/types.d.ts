type Command =
  | "bold"
  | "italic"
  | "underline"
  | "strikeThrough"
  | "insertUnorderedList"
  | "insertImage";

type Color =
  | "yellow"
  | "green"
  | "pink"
  | "purple"
  | "blue"
  | "grey"
  | "charcoal";

type Note = {
  label: string,
  lastModified: string;
  highlight: Color,
  text: string
};
