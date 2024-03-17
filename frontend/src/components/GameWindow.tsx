import { Stage } from '@pixi/react';
import RampageVehicle from './RampageVehicle';

function GameWindow() {

  return (
    <Stage width={1200} height={800} >

      <RampageVehicle x={100} y={100} cannonX={175} cannonY={175} />
    </Stage>
  );
};

export default GameWindow;