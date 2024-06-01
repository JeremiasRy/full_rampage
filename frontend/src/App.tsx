import GameWindow from "./components/GameWindow";
import { validInputs } from "./types/input";
import { isValidInput } from "./utils/helpers";
import { useEffect, useRef, useState } from "react";
import isEqual from "lodash/isEqual";
import proto from "protobufjs";
import {
  InGameOutput,
  LobbyMessage,
  MessageType,
  PlayerId,
  ClientLobbyStatus,
  RequestType,
  GameControllerStatus,
} from "./types/responses";
import Lobby from "./components/Lobby";

function App() {
  const [keysDown, setKeysDown] = useState<Set<number>>(new Set());
  const [sentInputs, setSentInputs] = useState<number[]>([]);
  const [id, setId] = useState(0);
  const [inGameOutput, setInGameOutput] = useState<InGameOutput>({
    type: MessageType.Frame,
    players: [],
    shots: [],
    explosions: [],
  });
  const [lobby, setLobby] = useState<LobbyMessage>({
    gameStatus: GameControllerStatus.Stopped,
    type: MessageType.LobbyMessage,
    countdownAmount: 0,
    winnerOfLastGame: 0,
    clients: [],
  });
  const connection = useRef<WebSocket | null>(null);
  const protoRootRef = useRef<proto.Root>();

  function sendLobbyStatusRequest() {
    if (!connection.current || !protoRootRef.current) {
      return;
    }
    const payload = {
      type: RequestType.LobbyInput,
      playerId: id,
      input: 0,
      status: ClientLobbyStatus.Ready,
    };

    const message = protoRootRef.current
      .lookupType("InputRequest")
      .encode(payload)
      .finish();

    connection.current.send(message);
  }

  function handleKeyDown(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code];
    setKeysDown((prevKeys) => {
      const updatedSet = new Set([...prevKeys, parsedInput]);
      if (parsedInput == validInputs.Space && prevKeys.has(validInputs.Fire)) {
        updatedSet.delete(validInputs.Fire);
      }
      return updatedSet;
    });
  }

  function handleKeyUp(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code];
    setKeysDown((prevKeys) => {
      const updatedSet = new Set(prevKeys);

      if (parsedInput == validInputs.Space && prevKeys.has(validInputs.Space)) {
        updatedSet.add(validInputs.Fire);
      }

      updatedSet.delete(parsedInput);
      return updatedSet;
    });
  }

  function handleVisibilityChange() {
    setKeysDown(new Set());
  }

  // Send users inputs from keysDown to backend for processing
  useEffect(() => {
    const keysArray = Array.from(keysDown);

    if (
      !connection.current ||
      !protoRootRef.current ||
      id == 0 ||
      isEqual(keysArray, sentInputs)
    ) {
      return;
    }
    const payload = {
      type: RequestType.InGameInput,
      playerId: id,
      input: keysArray.reduce((a, b) => a + b, 0),
    };

    const message = protoRootRef.current
      .lookupType("InputRequest")
      .encode(payload)
      .finish();

    connection.current.send(message);
    setSentInputs(keysArray);
  }, [keysDown]);

  useEffect(() => {
    let protoRoot: proto.Root;

    const protoLoad = async () => {
      try {
        const root = await proto.load("messages-frontend.proto");
        protoRoot = root;
        protoRootRef.current = protoRoot;
      } catch (e) {
        Error(`Failed to load proto: ${e}`);
      }
    };

    protoLoad();
    const socket = new WebSocket(import.meta.env.VITE_BACKEND_URL);

    socket.addEventListener("message", async (event): Promise<void> => {
      if (!protoRoot) {
        throw Error("protos not loaded correctly");
      }
      const data = event.data as Blob;
      const reader = new FileReader();
      reader.readAsArrayBuffer(data);

      reader.onload = async () => {
        const arrayBuffer = reader.result;
        if (arrayBuffer && typeof arrayBuffer !== "string") {
          const uintArr = new Uint8Array(arrayBuffer);
          const messageFlag = uintArr[1];

          if (messageFlag === MessageType.IdResponse) {
            const { playerId } = protoRoot
              .lookupType("PlayerId")
              .decode(uintArr) as unknown as PlayerId;

            setId(playerId);
            return;
          }

          if (messageFlag === MessageType.Frame) {
            const frame = {
              ...(protoRoot
                .lookupType("ServerGameFrameResponse")
                .decode(uintArr) as unknown as InGameOutput),
            };
            setInGameOutput(frame);
            return;
          }

          if (messageFlag === MessageType.LobbyMessage) {
            const lobby = protoRoot
              .lookupType("ServerLobbyResponse")
              .decode(uintArr) as unknown as LobbyMessage;

            setLobby(lobby);
            return;
          }
        }
      };
    });

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);
    document.addEventListener("visibilitychange", handleVisibilityChange);
    window.addEventListener("blur", handleVisibilityChange);

    connection.current = socket;

    return () => {
      socket.close();
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  }, []);

  return (
    <div className="main-wrapper">
      <GameWindow
        inGameOutput={inGameOutput}
        currentClient={id}
        status={lobby.gameStatus}
        countdown={lobby.countdownAmount}
      />
      <Lobby
        {...lobby}
        currentClientId={id}
        onAction={sendLobbyStatusRequest}
      />
    </div>
  );
}

export default App;
