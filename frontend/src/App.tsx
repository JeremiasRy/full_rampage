import GameWindow from "./components/GameWindow";
import { validInputs } from "./types/input";
import { isValidInput } from "./utils/helpers";
import { useEffect, useRef, useState } from "react";
import isEqual from "lodash/isEqual";
import proto from "protobufjs";
import {
  PlayerResponse,
  ServerOutput,
  CannonShotResponse,
} from "./types/responses";

function App() {
  const [keysDown, setKeysDown] = useState<Set<number>>(new Set());
  const [sentInputs, setSentInputs] = useState<number[]>([]);
  const [id, setId] = useState(0);
  const [serverOutput, setServerOutput] = useState<ServerOutput>({
    players: [],
    shots: [],
  });
  const connection = useRef<WebSocket | null>(null);
  const protoRootRef = useRef<proto.Root>();

  function handleKeyDown(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code];
    setKeysDown((keys) => new Set([...keys, parsedInput]));
  }

  function handleKeyUp(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code];
    setKeysDown((prevKeys) => {
      const updatedSet = new Set(prevKeys);
      if (parsedInput == 1 << 6 && prevKeys.has(1 << 6)) {
        updatedSet.add(1 << 7);
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
    let protoEnum: proto.Enum;

    const protoLoad = async () => {
      try {
        const root = await proto.load("messages-frontend.proto");
        protoRoot = root;
        protoEnum = root.lookupEnum("MessageType");
        protoRootRef.current = protoRoot;
      } catch (e) {
        Error(`Failed to load proto: ${e}`);
      }
    };

    protoLoad();

    const socket = new WebSocket("ws://127.0.0.1:9999");

    socket.addEventListener("message", async (event): Promise<void> => {
      if (!protoRoot || !protoEnum) {
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
          if (messageFlag === 2) {
            const { playerId } = {
              ...(protoRoot
                .lookupType("PlayerId")
                .decode(uintArr) as unknown as {
                playerId: number;
                type: number;
              }),
            };
            setId(playerId);
            return;
          }
          if (messageFlag === 1) {
            const { players, shots } = {
              ...(protoRoot
                .lookupType("ServerOutput")
                .decode(uintArr) as unknown as {
                type: number;
                players: PlayerResponse[];
                shots: CannonShotResponse[];
              }),
            };
            setServerOutput({
              players,
              shots,
            });
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

  return <GameWindow serverOutput={serverOutput} />;
}

export default App;
