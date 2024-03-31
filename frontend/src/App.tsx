import GameWindow from "./components/GameWindow";
import { validInputs } from "./types/input";
import { isValidInput } from "./utils/helpers";
import { useEffect, useRef, useState } from "react";
import isEqual from "lodash/isEqual";

function App() {
  const [keysDown, setKeysDown] = useState<Set<number>>(new Set());
  const [sentInputs, setSentInputs] = useState<number[]>([]);
  const connection = useRef<WebSocket | null>(null);
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

    if (isEqual(keysArray, sentInputs)) {
      return;
    }

    connection.current?.send(keysArray.reduce((a, b) => a + b, 0).toString());
    setSentInputs(keysArray);
  }, [keysDown]);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:9999");

    socket.addEventListener("open", () => {
      console.log("We are open!");
    });

    socket.addEventListener("error", (e) => {
      console.log("things went south  ", e);
    });

    socket.addEventListener("message", (event) => {
      console.log("Message from server ", event.data);
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

  return <GameWindow />;
}

export default App;
