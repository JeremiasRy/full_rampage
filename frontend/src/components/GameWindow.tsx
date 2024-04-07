import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";
import { Frame } from "../types/responses";

function GameWindow(props: { frame: Frame[] }) {
  const { frame } = { ...props };

  return (
    <Stage width={1200} height={800}>
      {frame.map((fr) => (
        <RampageVehicle
          x={fr.position.x}
          y={fr.position.y}
          cannonX={fr.cannonPosition.x}
          cannonY={fr.cannonPosition.y}
        />
      ))}
    </Stage>
  );
}

export default GameWindow;
