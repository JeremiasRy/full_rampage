export const validInputs = {
  ArrowUp: () => parseInt("0001", 2),
  ArrowRight: () => parseInt("0010", 2),
  ArrowDown: () => parseInt("0100", 2),
  ArrowLeft: () => parseInt("1000", 2),
} as const;

export type ValidInput = ReturnType<
  (typeof validInputs)[keyof typeof validInputs]
>;
