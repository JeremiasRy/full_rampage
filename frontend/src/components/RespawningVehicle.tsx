import { useEffect, useState } from "react";
import { Container, Graphics as GraphicsComponent } from "@pixi/react";
import { RampageVehicleProps } from "./RampageVehicle";
import { rampageVehicle, turret } from "../utils/drawFunctions";

interface DrawRespawningVehicleProps {
  vehicleProps: RampageVehicleProps;
  flickerInterval: number;
}

function DrawRespawningVehicle(props: DrawRespawningVehicleProps) {
  const color = props.vehicleProps.client ? 0x3300ff : 0xff3300;
  const { rotation, x, y, cannonX, cannonY } = { ...props.vehicleProps };
  const [alpha, setAlpha] = useState(1);

  useEffect(() => {
    const interval = setInterval(() => {
      setAlpha((prev) => (prev === 1 ? 0 : 1));
    }, props.flickerInterval);

    return () => {
      clearInterval(interval);
    };
  }, [props.flickerInterval]);

  return (
    <>
      <Container
        angle={rotation}
        position={{ x: x + 20, y: y + 20 }}
        pivot={{ x: 20, y: 20 }}
      >
        <GraphicsComponent draw={(g) => rampageVehicle(g, color, alpha)} />
      </Container>
      <GraphicsComponent
        draw={(g) => turret(g, x, y, 20, 20, cannonX, cannonY, alpha)}
      />
    </>
  );
}

export default DrawRespawningVehicle;
