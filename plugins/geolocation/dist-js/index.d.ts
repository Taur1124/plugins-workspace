import { type PositionOptions, type Position } from "./bindings";
export declare function watchPosition(options: PositionOptions, cb: (location: Position | string) => void): Promise<number>;
export declare const getCurrentPosition: (options: PositionOptions | null) => Promise<import("./bindings").Result<Position, never>>, clearWatch: (channelId: number) => Promise<import("./bindings").Result<null, never>>, checkPermissions: () => Promise<import("./bindings").Result<import("./bindings").PermissionStatus, never>>, requestPermissions: (permissions: import("./bindings").PermissionType[] | null) => Promise<import("./bindings").Result<import("./bindings").PermissionStatus, never>>;
export type { PermissionState, PermissionStatus, PermissionType, Position, PositionOptions, Coordinates, } from "./bindings";
