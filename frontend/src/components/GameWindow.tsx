import { Stage } from "@pixi/react";
import RampageVehicle from "./RampageVehicle";

function GameWindow() {
  // have a state for all the renderable items in the map: const [renderUs, setRenderUs] = useState<RenderableObject>[]([])
  // Listen a web socket and always update when a new frame is delivered by the backend.
  return (
    <Stage width={1200} height={800}>
      <RampageVehicle x={100} y={100} cannonX={175} cannonY={175} />
    </Stage>
  );
}

export default GameWindow;
