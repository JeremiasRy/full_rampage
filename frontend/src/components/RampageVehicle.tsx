import { useCallback, useEffect, useMemo, useState } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
  dead: boolean;
}

function getCenter(vehicle: RampageVehicleProps) {
  return { centerX: vehicle.x + 25 / 2, centerY: vehicle.y + 25 / 2 };
}

function DrawRampageVehicle(props: RampageVehicleProps) {
  const [flicker, setFlickerColor] = useState(0xff3300);
  const [cannonFlicker, setCannonFlickerColor] = useState(0xffffff);
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY, props.dead]);

  useEffect(() => {
    if (props.dead) {
      const intervalId = setInterval(() => {
        setFlickerColor((prev) => (prev === 0x0 ? 0xff3300 : 0x0));
        setCannonFlickerColor((prev) => (prev === 0x0 ? 0xffffff : 0x0));
      }, 100);

      return () => {
        clearInterval(intervalId);
      };
    }
  }, [props.dead]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY, centerX, centerY, dead } = {
        ...drawProps,
        ...getCenter(drawProps),
      };

      g.clear()
        .beginFill(dead ? flicker : 0xff3300)
        .drawRect(x, y, 25, 25)
        .endFill()
        .moveTo(centerX, centerY)
        .lineStyle(2, dead ? cannonFlicker : 0xffffff)
        .lineTo(cannonX, cannonY);
    },
    [drawProps, flicker]
  );

  return <GraphicsComponent draw={draw} />;
}

export default DrawRampageVehicle;
