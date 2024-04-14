export const validInputs = {
  ArrowUp: () => 1,
  ArrowRight: () => 1 << 1,
  ArrowDown: () => 1 << 2,
  ArrowLeft: () => 1 << 3,
  KeyZ: () => 1 << 4,
  KeyX: () => 1 << 5,
  Space: () => 1 << 6,
  Fire: () => 1 << 7,
} as const;

export type ValidInput = ReturnType<
  (typeof validInputs)[keyof typeof validInputs]
>;
