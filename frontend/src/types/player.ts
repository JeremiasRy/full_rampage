export type CannonAngle = number
export function isValidCannonAngle(angle: number): angle is CannonAngle {
    return angle >= 0 && angle <= 359;
}

export type RampageVehicleState = {
    x: number,
    y: number,
    cannonAngle: CannonAngle
}

export type Player = {
    alive: boolean,
    kills: number,
    exp: number,
    level: number,
    vehicleState: RampageVehicleState
}