import { useCallback, useEffect, useMemo, useState } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
  status: number;
}

function getCenter(vehicle: RampageVehicleProps) {
  return { centerX: vehicle.x + 25 / 2, centerY: vehicle.y + 25 / 2 };
}

function DrawRampageVehicle(props: RampageVehicleProps) {
  const [flicker, setFlickerColor] = useState(0xff3300);
  const [cannonFlicker, setCannonFlickerColor] = useState(0xffffff);
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY, props.status]);

  useEffect(() => {
    if (props.status == 1) {
      const intervalId = setInterval(() => {
        setFlickerColor((prev) => (prev === 0x0 ? 0xff3300 : 0x0));
        setCannonFlickerColor((prev) => (prev === 0x0 ? 0xffffff : 0x0));
      }, 200);

      return () => {
        clearInterval(intervalId);
      };
    } else if (props.status == 2) {
      const intervalId = setInterval(() => {
        setFlickerColor((prev) => (prev === 0x0 ? 0xff3300 : 0x0));
        setCannonFlickerColor((prev) => (prev === 0x0 ? 0xffffff : 0x0));
      }, 100);

      return () => {
        clearInterval(intervalId);
      };
    }
  }, [props.status]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY, centerX, centerY, status } = {
        ...drawProps,
        ...getCenter(drawProps),
      };

      g.clear()
        .beginFill(status ? flicker : 0xff3300)
        .drawRect(x, y, 25, 25)
        .endFill()
        .moveTo(centerX, centerY)
        .lineStyle(2, status ? cannonFlicker : 0xffffff)
        .lineTo(cannonX, cannonY);
    },
    [drawProps, flicker]
  );

  return <GraphicsComponent draw={draw} />;
}

export default DrawRampageVehicle;
