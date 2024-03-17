import { Graphics } from "@pixi/graphics";

export type RenderableObject = {
  position: Position;
  draw: (graphics: Graphics) => void;
};
export type Position = {
  x: number;
  y: number;
};
