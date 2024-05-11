export type InGameOutput = {
  type: MessageType;
  players: PlayerResponse[];
  shots: CannonEventResponse[];
  explosions: CannonEventResponse[];
};

export type LobbyMessage = {
  type: MessageType;
  clients: Client[];
};

export type Client = {
  id: number;
  lobbyStatus: ClientLobbyStatus;
  status: ClientStatus;
};
export type PlayerId = {
  type: number;
  playerId: number;
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

export enum MessageType {
  Normal = 1,
  Frame = 2,
  IdResponse = 3,
  LobbyMessage = 4,
}

export enum ClientStatus {
  Lobby = 1,
  InGame = 2,
}

export enum ClientLobbyStatus {
  WaitingConfirmation = 1,
  Ready = 2,
}

export enum RequestType {
  InGameInput = 1,
  LobbyInput = 2,
}
