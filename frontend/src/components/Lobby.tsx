import {
  Client,
  ClientLobbyStatus,
  GameControllerStatus,
} from "../types/responses";

export interface LobbyProps {
  gameStatus: GameControllerStatus;
  currentClientId: number;
  clients: Client[];
  onAction: () => void;
}

export default function Lobby(props: LobbyProps) {
  const { clients, currentClientId, gameStatus, onAction } = { ...props };
  return (
    <div className="lobby-wrapper">
      <h1>Lobby!</h1>
      {clients.map(({ id, lobbyStatus }) => (
        <div className="lobby-wrapper__lobby-item" key={id}>
          Player: {id} | {ClientLobbyStatus[lobbyStatus]}
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
