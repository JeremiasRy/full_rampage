import { useCallback, useMemo } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
}

function getCenter(vehicle: RampageVehicleProps) {
  return { centerX: vehicle.x + 25 / 2, centerY: vehicle.y + 25 / 2 };
}

function drawRampageVehicle(props: RampageVehicleProps) {
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY, centerX, centerY } = {
        ...drawProps,
        ...getCenter(drawProps),
      };

      g.clear()
        .beginFill(0xff3300)
        .drawRect(x, y, 25, 25)
        .endFill()
        .moveTo(centerX, centerY)
        .lineStyle(2, 0xffffff)
        .lineTo(cannonX, cannonY);
    },
    [drawProps]
  );

  return <GraphicsComponent draw={draw} />;
}

export default drawRampageVehicle;
