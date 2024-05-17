import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";
import Shot from "./Shot";
import { InGameOutput, InGameStatus } from "../types/responses";
import DrawRespawningVehicle from "./RespawningVehicle";

function GameWindow(props: {
  inGameOutput: InGameOutput;
  currentClient: number;
}) {
  const { players, shots, explosions } = { ...props.inGameOutput }; // make this into context some day
  return (
    <Stage width={1200} height={800}>
      {players.map((player) =>
        player.inGameStatus === InGameStatus.Alive ? (
          <RampageVehicle
            key={player.id}
            x={player.position.x}
            y={player.position.y}
            cannonX={player.cannonPosition.x}
            cannonY={player.cannonPosition.y}
            client={player.id === props.currentClient}
          />
        ) : (
          <DrawRespawningVehicle
            key={player.id}
            vehicleProps={{
              x: player.position.x,
              y: player.position.y,
              cannonX: player.cannonPosition.x,
              cannonY: player.cannonPosition.y,
              client: player.id === props.currentClient,
            }}
            flickerInterval={
              player.inGameStatus === InGameStatus.Dead ? 200 : 100
            }
          />
        )
      )}
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
