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
      <div className="lobby__headers">
        <h1>Lobby</h1>
        <h2>
          {printGameStatus()}{" "}
          {gameStatus === GameControllerStatus.Countdown &&
            Math.ceil(countdownAmount / 60)}
        </h2>
      </div>
      <div className="lobby__clients">
        {clients
          .sort((a, b) => b.score - a.score)
          .map(({ id, lobbyStatus, score }) => (
            <div
              className={`lobby-wrapper__lobby-item ${
                currentClientId === id && "client"
              } ${
                lobbyStatus === ClientLobbyStatus.Ready ? "ready" : "waiting"
              }`}
              key={id}
            >
              <div className="soldier__avatar" />
              {id === winnerOfLastGame && (
                <span className="winner">Champ!</span>
              )}
              <div className="info">
                <span>
                  {GameControllerStatus.Playing && <> Score: {score} </>}
                </span>
                <span>{printClientStatus(lobbyStatus)}</span>
                <span>
                  {currentClientId === id &&
                    lobbyStatus === ClientLobbyStatus.Waiting &&
                    gameStatus === GameControllerStatus.Stopped && (
                      <button onClick={onAction}>Ready?</button>
                    )}
                </span>
              </div>
            </div>
          ))}
      </div>
    </div>
  );
}
