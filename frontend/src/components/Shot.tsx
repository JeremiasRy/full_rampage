import { useCallback, useMemo } from "react";
import { Graphics as GraphicsComponent } from "@pixi/react";
import { Graphics } from "@pixi/graphics";

export interface ShotProps {
  x: number;
  y: number;
  size: number;
}
const MAX_SIZE = 20;

function DrawShot(props: ShotProps) {
  const drawProps = useMemo(() => {
    return { ...props };
  }, [props.x, props.y, props.size]);

  const draw = useCallback(
    (g: Graphics) => {
      const { x, y, size } = {
        ...drawProps,
      };

      g.clear()
        .beginFill(0xff3300)
        .drawCircle(x, y, MAX_SIZE * (size / 100))
        .endFill();
    },
    [drawProps]
  );

  return <GraphicsComponent draw={draw} />;
}

export default DrawShot;
