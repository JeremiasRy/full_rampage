import { Graphics } from "@pixi/graphics";

const tankWidth = 40;
const tankHeight = 40;
const turretRadius = 7;
const trackWidth = 5;
const trackHeight = tankHeight;
const bodyWidth = tankWidth - 2 * trackWidth;
const bodyHeight = tankHeight;
const bodyOffsetX = 0 + trackWidth;
const bodyOffsetY = 0;
const centerX = 40 / 2;
const centerY = 40 / 2;

export function rampageVehicle(g: Graphics, color: number, alpha: number = 1) {
  g.clear();

  g.beginFill(0x444444, alpha);
  g.drawRect(0, 0, trackHeight, trackWidth);
  g.drawRect(0, tankHeight - trackWidth, trackHeight, trackWidth);

  g.lineStyle(1, 0x222222, alpha);
  for (let i = 0; i < trackHeight; i += 5) {
    g.moveTo(i, 0).lineTo(i, trackWidth);
    g.moveTo(i, tankHeight - trackWidth).lineTo(i, tankHeight);
  }
  g.endFill();

  g.beginFill(color, alpha)
    .drawRect(bodyOffsetY, bodyOffsetX, bodyHeight, bodyWidth)
    .endFill();

  g.beginFill(color, alpha);
  g.drawRect(bodyOffsetY, bodyOffsetX, bodyHeight / 2, bodyWidth);
  g.endFill();

  g.beginFill(color, alpha);
  g.drawRect(bodyOffsetY, bodyOffsetX, bodyHeight / 4, bodyWidth);
  g.endFill();

  g.beginFill(0x555555, alpha)
    .drawCircle(centerX + 2, centerY + 2, turretRadius)
    .endFill();
  g.beginFill(0x888888, alpha)
    .drawCircle(centerX, centerY, turretRadius)
    .endFill();

  g.lineStyle(1, 0xaaaaaa, alpha)
    .moveTo(centerX - turretRadius / 2, centerY)
    .lineTo(centerX + turretRadius / 2, centerY)
    .moveTo(centerX, centerY - turretRadius / 2)
    .lineTo(centerX, centerY + turretRadius / 2);

  g.lineStyle(1, 0xaaaaaa, alpha)
    .moveTo(bodyOffsetY + 5, bodyOffsetX + 5)
    .lineTo(bodyOffsetY + bodyHeight - 5, bodyOffsetX + 5)
    .moveTo(bodyOffsetY + 5, bodyOffsetX + bodyWidth - 5)
    .lineTo(bodyOffsetY + bodyHeight - 5, bodyOffsetX + bodyWidth - 5);
}

export function turret(
  g: Graphics,
  x: number,
  y: number,
  centerX: number,
  centerY: number,
  cannonX: number,
  cannonY: number,
  alpha: number = 1
) {
  g.clear();
  g.lineStyle(2, 0xffffff, alpha)
    .moveTo(x + centerX, y + centerY)
    .lineTo(cannonX, cannonY);
}
