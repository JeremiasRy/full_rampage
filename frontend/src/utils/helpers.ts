import { validInputs } from "../types/input";

export function isValidInput(input: string): input is keyof typeof validInputs {
  return input in validInputs;
}
