export const validInputs = {
  ArrowUp: () => parseInt("1", 2),
  ArrowRight: () => parseInt("10", 2),
  ArrowDown: () => parseInt("100", 2),
  ArrowLeft: () => parseInt("1000", 2),
  KeyZ: () => parseInt("10000", 2),
  KeyX: () => parseInt("100000", 2),
} as const;

export type ValidInput = ReturnType<
  (typeof validInputs)[keyof typeof validInputs]
>;
