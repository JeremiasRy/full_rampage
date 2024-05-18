import { useCallback, useMemo } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface RampageVehicleProps {
  x: number;
  y: number;
  cannonX: number;
  cannonY: number;
  client: boolean;
}

export function getCenter(vehicle: RampageVehicleProps) {
  return { centerX: vehicle.x + 40 / 2, centerY: vehicle.y + 40 / 2 };
}

function DrawRampageVehicle(props: RampageVehicleProps) {
  const color = props.client ? 0x3300ff : 0xff3300;
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY, centerX, centerY } = {
        ...drawProps,
        ...getCenter(drawProps),
      };

      const tankWidth = 40;
      const tankHeight = 40;
      const turretRadius = 7;
      const trackWidth = 5;
      const trackHeight = tankHeight;
      const bodyWidth = tankWidth - 2 * trackWidth;
      const bodyHeight = tankHeight;
      const bodyOffsetX = x + trackWidth;
      const bodyOffsetY = y;

      g.clear();

      g.beginFill(0x444444);
      g.drawRect(x, y, trackWidth, trackHeight); // Left track
      g.drawRect(x + tankWidth - trackWidth, y, trackWidth, trackHeight); // Right track

      g.lineStyle(1, 0x222222);
      for (let i = y; i < y + trackHeight; i += 5) {
        g.moveTo(x, i).lineTo(x + trackWidth, i); // Left track details
        g.moveTo(x + tankWidth - trackWidth, i).lineTo(x + tankWidth, i); // Right track details
      }
      g.endFill();

      g.beginFill(color)
        .drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight)
        .endFill();

      g.beginFill(0x0000ff, 0.2);
      g.drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight / 2);
      g.endFill();

      g.beginFill(0xffffff, 0.1);
      g.drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight / 4);
      g.endFill();

      g.beginFill(0x555555, 0.8)
        .drawCircle(centerX + 2, centerY + 2, turretRadius)
        .endFill();
      g.beginFill(0x888888)
        .drawCircle(centerX, centerY, turretRadius)
        .endFill();

      g.lineStyle(2, 0xffffff)
        .moveTo(centerX, centerY)
        .lineTo(cannonX, cannonY);

      g.lineStyle(1, 0xaaaaaa)
        .moveTo(centerX - turretRadius / 2, centerY)
        .lineTo(centerX + turretRadius / 2, centerY)
        .moveTo(centerX, centerY - turretRadius / 2)
        .lineTo(centerX, centerY + turretRadius / 2);

      g.lineStyle(1, 0xaaaaaa)
        .moveTo(bodyOffsetX + 5, bodyOffsetY + 5)
        .lineTo(bodyOffsetX + bodyWidth - 5, bodyOffsetY + 5)
        .moveTo(bodyOffsetX + 5, bodyOffsetY + bodyHeight - 5)
        .lineTo(bodyOffsetX + bodyWidth - 5, bodyOffsetY + bodyHeight - 5);
    },
    [drawProps]
  );
  return <GraphicsComponent draw={draw} />;
}

export default DrawRampageVehicle;
