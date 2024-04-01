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
      player_id: id,
      input: keysArray.reduce((a, b) => a + b, 0),
    };

    const message = inputRequest.current?.create(payload);
    if (!message) {
      return;
    }
    const buffer = inputRequest.current?.encode(message).finish();
    connection.current?.send(buffer?.buffer!);
    setSentInputs(keysArray);
  }, [keysDown]);

  useEffect(() => {
    proto.load("messages.proto", (err, root) => {
      console.log(err);
      inputRequest.current = root?.lookupType("InputRequest") || null;
      idResponse.current = root?.lookupType("PlayerId") || null;
      frameResponse.current = root?.lookupType("ServerOutput") || null;

      console.log(inputRequest.current);
      console.log(idResponse.current);
      console.log(frameResponse.current);
    });

    const socket = new WebSocket("ws://127.0.0.1:9999");
    socket.addEventListener("open", () => {
      console.log("We are open!");
    });

    socket.addEventListener("error", (e) => {
      console.log("things went south  ", e);
    });

    socket.addEventListener("message", (event): void => {
      const isId = idResponse.current?.verify(event.data);
      if (isId) {
        setId(parseInt(isId));
        return;
      }

      const isFrame = frameResponse.current?.verify(event.data);

      setFrame(isFrame as unknown as Frame[]);
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
