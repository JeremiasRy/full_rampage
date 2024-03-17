import { useCallback, useMemo } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
}

function drawRampageVehicle(props: RampageVehicleProps) {
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY } = { ...drawProps };
      g.clear()
        .beginFill(0xff3300)
        .drawRect(x, y, 50, 50)
        .endFill()
        .lineStyle(2, 0xffffff)
        .moveTo(x + 25, y + 25)
        .lineTo(cannonX, cannonY);
    },
    [drawProps]
  );

  return <GraphicsComponent draw={draw} />;
}

export default drawRampageVehicle;
