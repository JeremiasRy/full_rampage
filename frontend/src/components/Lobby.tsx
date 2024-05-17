import {
  Client,
  ClientLobbyStatus,
  GameControllerStatus,
} from "../types/responses";

export interface LobbyProps {
  gameStatus: GameControllerStatus;
  currentClientId: number;
  clients: Client[];
  countdownAmount: number;
  onAction: () => void;
}

export default function Lobby(props: LobbyProps) {
  const { clients, currentClientId, gameStatus, countdownAmount, onAction } = {
    ...props,
  };
  const printGameStatus = () => {
    switch (gameStatus) {
      case GameControllerStatus.Playing: {
        return "Game in play!";
      }
      case GameControllerStatus.Countdown: {
        return "Game starting in...";
      }
      case GameControllerStatus.Stopped: {
        return "Waiting for players to be ready";
      }
    }
  };

  const printClientStatus = (clientStatus: ClientLobbyStatus) => {
    if (gameStatus === GameControllerStatus.Playing) {
      if (clientStatus === ClientLobbyStatus.Ready) {
        return "Playing";
      } else {
        return "Waiting...";
      }
    } else if (gameStatus === GameControllerStatus.Countdown) {
      if (clientStatus === ClientLobbyStatus.Ready) {
        return "Going for war!";
      } else {
        return "Waiting...";
      }
    } else {
      if (clientStatus === ClientLobbyStatus.Ready) {
        return "Ready!";
      } else {
        return "Waiting...";
      }
    }
  };
  return (
    <div className="lobby-wrapper">
      <h1>Lobby!</h1>
      <h2>
        {printGameStatus()}{" "}
        {gameStatus === GameControllerStatus.Countdown &&
          Math.round(countdownAmount / 10)}
      </h2>
      {clients.map(({ id, lobbyStatus }) => (
        <div
          className={`lobby-wrapper__lobby-item ${
            currentClientId === id && "client"
          } 
          ${lobbyStatus === ClientLobbyStatus.Ready ? "ready" : "waiting"}`}
          key={id}
        >
          Player: {id} | {printClientStatus(lobbyStatus)}
          {currentClientId === id &&
            lobbyStatus === ClientLobbyStatus.Waiting &&
            gameStatus === GameControllerStatus.Stopped && (
              <button onClick={onAction}>Ready?</button>
            )}
        </div>
      ))}
    </div>
  );
}
