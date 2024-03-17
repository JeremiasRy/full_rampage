import GameWindow from "./components/GameWindow";
import { isValidInput } from "./utils/helpers";
import { useEffect, useState } from "react";

function App() {
  const [keysDown, setKeysDown] = useState<Set<string>>(new Set());

  function handleKeyDown(event: KeyboardEvent) {
    if (isValidInput(event.code)) {
      setKeysDown((keys) => new Set([...keys, event.code]));
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (isValidInput(event.code)) {
      setKeysDown((keys) => {
        keys.delete(event.code);
        return keys;
      });
    }
  }

  // Send users inputs from keysDown to backend for processing
  useEffect(() => {
    console.log("There is an input!!!");
    console.log(keysDown);
  }, [keysDown]);

  useEffect(() => {
    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  }, []);

  return <GameWindow />;
}

export default App;
