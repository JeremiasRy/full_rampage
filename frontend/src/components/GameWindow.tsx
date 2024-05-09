import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";
import Shot from "./Shot";
import { ServerOutput } from "../types/responses";

function GameWindow(props: { serverOutput: ServerOutput }) {
  const { players, shots, explosions } = { ...props.serverOutput }; // make this into context some day
  return (
    <Stage width={1200} height={800}>
      {players.map((player) => (
        <RampageVehicle
          key={player.id}
          x={player.position.x}
          y={player.position.y}
          cannonX={player.cannonPosition.x}
          cannonY={player.cannonPosition.y}
          dead={player.dead}
        />
      ))}
      {shots.map((shot) => (
        <Shot
          key={shot.id}
          x={shot.position.x}
          y={shot.position.y}
          size={shot.size}
        />
      ))}
      {explosions.map((shot) => (
        <Shot
          key={shot.id}
          x={shot.position.x}
          y={shot.position.y}
          size={shot.size}
        />
      ))}
    </Stage>
  );
}

export default GameWindow;
