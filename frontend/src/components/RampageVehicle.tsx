import { useCallback, useMemo } from "react";
import { Container, Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

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
  const centerX = 40 / 2;
  const centerY = 40 / 2;
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.cannonX, props.cannonY]);

  const drawCannon = useCallback(
    (g: Graphics) => {
      const { x, y, cannonX, cannonY } = { ...drawProps };
      g.clear();
      g.lineStyle(2, 0xffffff)
        .moveTo(x + centerX, y + centerY)
        .lineTo(cannonX, cannonY);
    },
    [drawProps]
  );

  const draw = useCallback(
    (g: Graphics) => {
      const tankWidth = 40;
      const tankHeight = 40;
      const turretRadius = 7;
      const trackWidth = 5;
      const trackHeight = tankHeight;
      const bodyWidth = tankWidth - 2 * trackWidth;
      const bodyHeight = tankHeight;
      const bodyOffsetX = 0 + trackWidth;
      const bodyOffsetY = 0;

      g.clear();

      g.beginFill(0x444444);
      g.drawRect(0, 0, trackHeight, trackWidth);
      g.drawRect(0, tankHeight - trackWidth, trackHeight, trackWidth);

      g.lineStyle(1, 0x222222);
      for (let i = 0; i < trackHeight; i += 5) {
        g.moveTo(i, 0).lineTo(i, trackWidth);
        g.moveTo(i, tankHeight - trackWidth).lineTo(i, tankHeight);
      }
      g.endFill();

      g.beginFill(color)
        .drawRect(bodyOffsetY, bodyOffsetX, bodyHeight, bodyWidth)
        .endFill();

      g.beginFill(0x0000ff, 0.2);
      g.drawRect(bodyOffsetY, bodyOffsetX, bodyHeight / 2, bodyWidth);
      g.endFill();

      g.beginFill(0xffffff, 0.1);
      g.drawRect(bodyOffsetY, bodyOffsetX, bodyHeight / 4, bodyWidth);
      g.endFill();

      g.beginFill(0x555555, 0.8)
        .drawCircle(centerX + 2, centerY + 2, turretRadius)
        .endFill();
      g.beginFill(0x888888)
        .drawCircle(centerX, centerY, turretRadius)
        .endFill();

      g.lineStyle(1, 0xaaaaaa)
        .moveTo(centerX - turretRadius / 2, centerY)
        .lineTo(centerX + turretRadius / 2, centerY)
        .moveTo(centerX, centerY - turretRadius / 2)
        .lineTo(centerX, centerY + turretRadius / 2);

      g.lineStyle(1, 0xaaaaaa)
        .moveTo(bodyOffsetY + 5, bodyOffsetX + 5)
        .lineTo(bodyOffsetY + bodyHeight - 5, bodyOffsetX + 5)
        .moveTo(bodyOffsetY + 5, bodyOffsetX + bodyWidth - 5)
        .lineTo(bodyOffsetY + bodyHeight - 5, bodyOffsetX + bodyWidth - 5);
    },
    [drawProps]
  );
  return (
    <>
      <Container
        angle={props.rotation}
        position={{ x: props.x + centerX, y: props.y + centerY }}
        pivot={{ x: centerX, y: centerY }}
      >
        <GraphicsComponent draw={draw} />
      </Container>
      <GraphicsComponent draw={drawCannon} />
    </>
  );
}

export default DrawRampageVehicle;
