import GameWindow from "./components/GameWindow";
import { validInputs } from "./types/input";
import { isValidInput } from "./utils/helpers";
import { useEffect, useRef, useState } from "react";
import isEqual from "lodash/isEqual";
import proto from "protobufjs";

function App() {
  const [keysDown, setKeysDown] = useState<Set<number>>(new Set());
  const [sentInputs, setSentInputs] = useState<number[]>([]);
  const [id, setId] = useState(0);
  const [frame, setFrame] = useState<Frame[]>([]);
  const connection = useRef<WebSocket | null>(null);
  const inputRequest = useRef<proto.Type | null>(null);
  const idResponse = useRef<proto.Type | null>(null);
  const frameResponse = useRef<proto.Type | null>(null);

  function handleKeyDown(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code]();
    setKeysDown((keys) => new Set([...keys, parsedInput]));
  }

  function handleKeyUp(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code]();
    setKeysDown((prevKeys) => {
      const updatedSet = new Set(prevKeys);
      updatedSet.delete(parsedInput);
      return updatedSet;
    });
  }

  // Send users inputs from keysDown to backend for processing
  useEffect(() => {
    const keysArray = Array.from(keysDown);

    if (id == 0 || isEqual(keysArray, sentInputs)) {
      return;
    }
    const payload = {
      playerId: id,
      input: keysArray.reduce((a, b) => a + b, 0),
    };

    const message = inputRequest.current?.create(payload);

    if (!message) {
      return;
    }
    const buffer = inputRequest.current?.encode(message).finish();

    if (!buffer) {
      return;
    }
    connection.current?.send(buffer);
    setSentInputs(keysArray);
  }, [keysDown]);

  useEffect(() => {
    proto.load("messages.proto", (err, root) => {
      if (err) {
        console.log(err);
      }
      inputRequest.current = root?.lookupType("InputRequest") || null;
      idResponse.current = root?.lookupType("PlayerId") || null;
      frameResponse.current = root?.lookupType("ServerOutput") || null;
    });

    const socket = new WebSocket("ws://127.0.0.1:9999");
    socket.addEventListener("open", () => {
      console.log("We are open!");
    });

    socket.addEventListener("error", (e) => {
      console.log("things went south  ", e);
    });

    socket.addEventListener("message", async (event): Promise<void> => {
      const data = event.data as Blob;
      const uintArr = new Uint8Array(await data.arrayBuffer());

      if (id === 0) {
        console.log("lets set id");
        let isId = idResponse.current?.decode(uintArr);
        if (isId) {
          const { playerId: id } = {
            ...(isId as unknown as { playerId: number }),
          };
          setId(id);
          return;
        }
      }

      const isFrame = frameResponse.current?.decode(uintArr);
      const { responses: frames } = {
        ...(isFrame as unknown as {
          responses: { cannonPosition: Position; position: Position }[];
        }),
      };
      console.log(frames);

      setFrame(frames as unknown as Frame[]);
    });

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);

    connection.current = socket;

    return () => {
      socket.close();
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  }, []);

  return <GameWindow frame={frame} />;
}

export default App;
