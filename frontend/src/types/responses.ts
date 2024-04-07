export type ServerOutput = {
  frames: Frame[];
};

export type PlayerId = {
  playerId: number;
  type: number;
};

export type Frame = {
  position: Position;
  cannonPosition: Position;
};
export type Position = {
  x: number;
  y: number;
};

export enum ProtobufType {
  IdResponse = 2,
  Frame = 1,
  InputRequest = 0,
}
