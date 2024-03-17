import GameWindow from "./components/GameWindow";
import { useEffect, useState } from "react";

function App() {
  const [keysDown, setKeysDown] = useState<Set<string>>(new Set());

  function handleKeyDown(event: KeyboardEvent) {
    setKeysDown((keys) => new Set([...keys, event.code]));
  }

  function handleKeyUp(event: KeyboardEvent) {
    setKeysDown((keys) => {
      keys.delete(event.code);
      return keys;
    });
  }

  console.log(keysDown);

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
