import { useCallback, useEffect, useState } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";
import { RampageVehicleProps } from "./RampageVehicle";

interface DrawRespawningVehicleProps {
  vehicleProps: RampageVehicleProps;
  flickerInterval: number;
}

export function getCenter(vehicle: RampageVehicleProps) {
  return { centerX: vehicle.x + 40 / 2, centerY: vehicle.y + 40 / 2 };
}

function DrawRespawningVehicle(props: DrawRespawningVehicleProps) {
  const color = props.vehicleProps.client ? 0x3300ff : 0xff3300;
  const [alpha, setAlpha] = useState(1);

  useEffect(() => {
    const interval = setInterval(() => {
      setAlpha((prev) => (prev === 1 ? 0 : 1));
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

      g.beginFill(0x444444, alpha);
      g.drawRect(x, y, trackWidth, trackHeight); // Left track
      g.drawRect(x + tankWidth - trackWidth, y, trackWidth, trackHeight); // Right track

      g.lineStyle(1, 0x222222, alpha);
      for (let i = y; i < y + trackHeight; i += 5) {
        g.moveTo(x, i).lineTo(x + trackWidth, i); // Left track details
        g.moveTo(x + tankWidth - trackWidth, i).lineTo(x + tankWidth, i); // Right track details
      }
      g.endFill();

      g.beginFill(color, alpha)
        .drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight)
        .endFill();

      g.beginFill(color, alpha);
      g.drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight / 2);
      g.endFill();

      g.beginFill(0xffffff, alpha);
      g.drawRect(bodyOffsetX, bodyOffsetY, bodyWidth, bodyHeight / 4);
      g.endFill();

      g.beginFill(0x555555, alpha)
        .drawCircle(centerX + 2, centerY + 2, turretRadius)
        .endFill();
      g.beginFill(0x888888, alpha)
        .drawCircle(centerX, centerY, turretRadius)
        .endFill();

      g.lineStyle(2, 0xffffff, alpha)
        .moveTo(centerX, centerY)
        .lineTo(cannonX, cannonY);

      g.lineStyle(1, 0xaaaaaa, alpha)
        .moveTo(centerX - turretRadius / 2, centerY)
        .lineTo(centerX + turretRadius / 2, centerY)
        .moveTo(centerX, centerY - turretRadius / 2)
        .lineTo(centerX, centerY + turretRadius / 2);

      g.lineStyle(1, 0xaaaaaa, alpha)
        .moveTo(bodyOffsetX + 5, bodyOffsetY + 5)
        .lineTo(bodyOffsetX + bodyWidth - 5, bodyOffsetY + 5)
        .moveTo(bodyOffsetX + 5, bodyOffsetY + bodyHeight - 5)
        .lineTo(bodyOffsetX + bodyWidth - 5, bodyOffsetY + bodyHeight - 5);
    },
    [alpha]
  );
  return <GraphicsComponent draw={draw} />;
}

export default DrawRespawningVehicle;
