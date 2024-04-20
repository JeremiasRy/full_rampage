import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";
import Shot from "./Shot";
import { ServerOutput } from "../types/responses";

function GameWindow(props: { serverOutput: ServerOutput }) {
  const { players, shots } = { ...props.serverOutput }; // make this into context some day
  return (
    <Stage width={1200} height={800}>
      {players.map((player) => (
        <RampageVehicle
          key={
            player.cannonPosition.x +
            player.cannonPosition.y +
            player.position.x +
            player.position.y
          }
          x={player.position.x}
          y={player.position.y}
          cannonX={player.cannonPosition.x}
          cannonY={player.cannonPosition.y}
        />
      ))}
      {shots.map((shot) => (
        <Shot x={shot.position.x} y={shot.position.y} size={shot.size} />
      ))}
    </Stage>
  );
}

export default GameWindow;
