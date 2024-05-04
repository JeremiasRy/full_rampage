export type ServerOutput = {
  players: PlayerResponse[];
  shots: CannonEventResponse[];
  explosions: CannonEventResponse[];
};

export type PlayerId = {
  playerId: number;
  type: number;
};

export type PlayerResponse = {
  position: Position;
  cannonPosition: Position;
  dead: boolean;
};
export type CannonEventResponse = {
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
