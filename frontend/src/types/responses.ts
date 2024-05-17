export type InGameOutput = {
  type: MessageType;
  players: PlayerResponse[];
  shots: CannonEventResponse[];
  explosions: CannonEventResponse[];
};

export type LobbyMessage = {
  type: MessageType;
  gameStatus: GameControllerStatus;
  clients: Client[];
  countdownAmount: number;
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
  inGameStatus: InGameStatus;
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

export enum InGameStatus {
  Alive = 1,
  Dead = 2,
  Respawning = 3,
}

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
  Waiting = 1,
  Ready = 2,
}

export enum RequestType {
  InGameInput = 1,
  LobbyInput = 2,
}

export enum GameControllerStatus {
  Countdown = 1,
  Playing = 2,
  Stopped = 3,
}
