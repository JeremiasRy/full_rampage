import { Sprite, Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";
import Shot from "./Shot";
import {
  GameControllerStatus,
  InGameOutput,
  InGameStatus,
} from "../types/responses";
import DrawRespawningVehicle from "./RespawningVehicle";

function GameWindow(props: {
  inGameOutput: InGameOutput;
  currentClient: number;
  status: GameControllerStatus;
  countdown: number;
}) {
  const { players, shots, explosions } = { ...props.inGameOutput }; // make this into context some day
  const centerForLogoX = (1200 - 1024 * 0.8) / 2;
  const centerForLogoY = (800 - 1024 * 0.8) / 2;

  const getAlpha = () => {
    const status = props.status;
    if (status === GameControllerStatus.Stopped) {
      return 0.5;
    }
    if (status === GameControllerStatus.Countdown) {
      return (props.countdown + 60) / 600;
    }
    return 0.1;
  };

  return (
    <Stage width={1200} height={800} className="game-window">
      <Sprite
        image={"./logo.webp"}
        x={centerForLogoX}
        y={centerForLogoY}
        scale={0.8}
        alpha={getAlpha()}
      />
      {players.map((player) =>
        player.inGameStatus === InGameStatus.Alive ? (
          <RampageVehicle
            key={player.id}
            x={player.position.x}
            y={player.position.y}
            cannonX={player.cannonPosition.x}
            cannonY={player.cannonPosition.y}
            rotation={player.tankRotation}
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
              rotation: player.tankRotation,
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
