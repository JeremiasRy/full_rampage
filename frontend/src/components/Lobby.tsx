import { Client, ClientLobbyStatus } from "../types/responses";

export interface LobbyProps {
  currentClientId: number;
  clients: Client[];
  onAction: () => void;
}

export default function Lobby(props: LobbyProps) {
  const { clients, currentClientId, onAction } = { ...props };
  return (
    <div className="lobby-wrapper">
      <h1>Lobby!</h1>
      {clients.map((client) => (
        <div className="lobby-wrapper__lobby-item" key={client.id}>
          Player: {client.id} | {ClientLobbyStatus[client.lobbyStatus]}
          {currentClientId === client.id &&
            client.lobbyStatus == ClientLobbyStatus.WaitingConfirmation && (
              <button onClick={onAction}>Ready?</button>
            )}
        </div>
      ))}
    </div>
  );
}
