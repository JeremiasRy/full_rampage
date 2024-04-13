export type ServerOutput = {
  players: PlayerResponse[];
  shots: CannonShotResponse[];
};

export type PlayerId = {
  playerId: number;
  type: number;
};

export type PlayerResponse = {
  position: Position;
  cannonPosition: Position;
};
export type CannonShotResponse = {
  position: Position;
  size: number;
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
