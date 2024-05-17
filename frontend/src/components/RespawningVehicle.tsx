import { useCallback, useEffect, useState } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";
import { RampageVehicleProps, getCenter } from "./RampageVehicle";

interface DrawRespawningVehicleProps {
  vehicleProps: RampageVehicleProps;
  flickerInterval: number;
}

function DrawRespawningVehicle(props: DrawRespawningVehicleProps) {
  const color = props.vehicleProps.client ? 0x3300ff : 0xff3300;
  const [flicker, setFlickerColor] = useState(color);
  const [cannonFlicker, setCannonFlickerColor] = useState(0xffffff);

  useEffect(() => {
    const interval = setInterval(() => {
      setFlickerColor((prev) => (prev === 0x0 ? color : 0x0));
      setCannonFlickerColor((prev) => (prev === 0x0 ? 0xffffff : 0x0));
    }, props.flickerInterval);

    return () => {
      clearInterval(interval);
    };
  }, [props.flickerInterval]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY, centerX, centerY } = {
        ...props.vehicleProps,
        ...getCenter(props.vehicleProps),
      };

      g.clear()
        .beginFill(flicker)
        .drawRect(x, y, 25, 25)
        .endFill()
        .moveTo(centerX, centerY)
        .lineStyle(2, cannonFlicker)
        .lineTo(cannonX, cannonY);
    },
    [flicker]
  );
  return <GraphicsComponent draw={draw} />;
}

export default DrawRespawningVehicle;
