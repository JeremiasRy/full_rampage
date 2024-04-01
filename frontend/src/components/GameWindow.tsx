import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";

function GameWindow(props: { frame: Frame[] }) {
  const { frame } = { ...props };
  // have a state for all the renderable items in the map: const [renderUs, setRenderUs] = useState<RenderableObject>[]([])
  // Listen a web socket and always update when a new frame is delivered by the backend.
  return (
    <Stage width={1200} height={800}>
      {frame.map((fr) => (
        <RampageVehicle
          x={fr.position.x}
          y={fr.position.y}
          cannonX={fr.cannon_position.x}
          cannonY={fr.cannon_position.y}
        />
      ))}
    </Stage>
  );
}

export default GameWindow;
