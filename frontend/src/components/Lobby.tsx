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
  winnerOfLastGame: number;
  onAction: () => void;
}

export default function Lobby(props: LobbyProps) {
  const {
    clients,
    currentClientId,
    gameStatus,
    countdownAmount,
    winnerOfLastGame,
    onAction,
  } = {
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
        return "Spectating...";
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
          Math.floor(countdownAmount / 60)}
      </h2>
      {clients
        .sort((a, b) => b.score - a.score)
        .map(({ id, lobbyStatus, score }) => (
          <div
            className={`lobby-wrapper__lobby-item ${
              currentClientId === id && "client"
            } 
          ${lobbyStatus === ClientLobbyStatus.Ready ? "ready" : "waiting"}`}
            key={id}
          >
            Player: {id}{" "}
            {GameControllerStatus.Playing && <> | Score: {score} </>}|{" "}
            {printClientStatus(lobbyStatus)}
            {currentClientId === id &&
              lobbyStatus === ClientLobbyStatus.Waiting &&
              gameStatus === GameControllerStatus.Stopped && (
                <button onClick={onAction}>Ready?</button>
              )}
            {id === winnerOfLastGame && "Winner"}
          </div>
        ))}
    </div>
  );
}
