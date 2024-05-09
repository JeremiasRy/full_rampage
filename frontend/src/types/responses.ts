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
  id: number;
  position: Position;
  cannonPosition: Position;
  status: number;
};
export type CannonEventResponse = {
  position: Position;
  size: number;
  fromId: number;
  id: number;
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
