import { validInputs } from "../types/input";

export function isValidInput(keyCode: string): boolean {
  return !!validInputs.find((validKeyCode) => validKeyCode === keyCode);
}
