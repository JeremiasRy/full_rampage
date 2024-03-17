export const validInputs = [
  "ArrowRight",
  "ArrowLeft",
  "ArrowUp",
  "ArrowDown",
  "Space",
] as const;
export type ValidInput = (typeof validInputs)[number];

export type InputRequest = {
  keys: ValidInput[];
};
