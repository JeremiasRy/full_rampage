import { useMemo } from "react";
import { Container, Graphics as GraphicsComponent } from "@pixi/react";
import { rampageVehicle, turret } from "../utils/drawFunctions";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
  rotation: number;
  client: boolean;
}

function DrawRampageVehicle(props: RampageVehicleProps) {
  const color = props.client ? 0x3300ff : 0xff3300;
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY]);

  return (
    <>
      <Container
        angle={props.rotation}
        position={{ x: drawProps.x + 20, y: drawProps.y + 20 }}
        pivot={{ x: 20, y: 20 }}
      >
        <GraphicsComponent draw={(g) => rampageVehicle(g, color)} />
      </Container>
      <GraphicsComponent
        draw={(g) =>
          turret(
            g,
            drawProps.x,
            drawProps.y,
            20,
            20,
            drawProps.cannonX,
            drawProps.cannonY
          )
        }
      />
    </>
  );
}

export default DrawRampageVehicle;
