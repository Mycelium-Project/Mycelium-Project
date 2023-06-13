"use client";

import { invoke } from "@tauri-apps/api/tauri";

//create a global union for network table types
type NetworkTableTypes =
  | Number
  | String
  | Boolean
  | Number[]
  | String[]
  | Boolean[]
  | Uint8Array;

export class NetworkTableHandlerId {
  ip: number[];
  port: number;
  identity: string;
  constructor(ip: number[], port: number, identity: string) {
    this.ip = ip;
    this.port = port;
    this.identity = identity;
  }

  /**
   * Checks if the network table client associated with this handlerId is running
   * @return a boolean representing whether the client is connected
   *
   * This function calls on the native backend and may result in a crash.
   */
  public async doesNetworkTableHandlerExist(): Promise<boolean> {
    return DoesNetworkTableHandlerExist(this);
  }

  /**
   * Ends the network table client associated with this handlerId
   *
   * This function calls on the native backend and may result in a crash.
   */
  public stopNetworkTableHandler(): void {
    StopNetworkTableHandler(this);
  }

  /**
   * Subscribes to a topic on the network table client associated with this handlerId
   * @param topic the topic to subscribe to
   * @param periodic the period to update the value of the topic at
   * @param all whether or not to subscribe to all entries in the topic
   * @param prefix whether or not to subscribe to all topics with the same prefix
   *
   * This function calls on the native backend and may result in a crash.
   */
  public subscribe(
    topic: String,
    periodic?: number,
    all?: boolean,
    prefix?: boolean
  ): void {
    Subscribe(this, topic, periodic, all, prefix);
  }

  public unsubscribe(topic: String): void {
    Unsubscribe(this, topic);
  }

  /**
   * Sets the value of a topic on the network table client associated with this handlerId.
   * infers the type of the value and calls the appropriate function
   * @param topic the topic to set the value of
   * @param value the value to set the topic to
   */
  public setEntry(topic: String, value: NetworkTableTypes): void {
    if (value instanceof Number) {
      if (value.valueOf() % 1 === 0) {
        SetInteger(this, topic, value);
      } else {
        SetDouble(this, topic, value);
      }
    } else if (value instanceof String) {
      SetString(this, topic, value);
    } else if (value instanceof Boolean) {
      SetBoolean(this, topic, value);
    } else if (value instanceof Uint8Array) {
      SetByteArray(this, topic, value);
    } else {
      if (value.length > 0) {
        if (value[0] instanceof Number) {
          if (value[0].valueOf() % 1 === 0) {
            SetIntegerArray(this, topic, value as Number[]);
          } else {
            SetDoubleArray(this, topic, value as Number[]);
          }
        } else if (value[0] instanceof String) {
          SetStringArray(this, topic, value as String[]);
        } else if (value[0] instanceof Boolean) {
          SetBooleanArray(this, topic, value as Boolean[]);
        } else if (typeof value[0] === "number") {
          if (value[0] % 1 === 0) {
            SetIntegerArray(this, topic, value as Number[]);
          } else {
            SetDoubleArray(this, topic, value as Number[]);
          }
        } else if (typeof value[0] === "string") {
          SetStringArray(this, topic, value as String[]);
        } else if (typeof value[0] === "boolean") {
          SetBooleanArray(this, topic, value as Boolean[]);
        }
      } else {
        SetDoubleArray(this, topic, []);
      }
    }
  }

  public async getEntries(): Promise<TableEntry[]> {
    return GetEntries(this);
  }

  public async getEntry(topic: String): Promise<TableEntry> {
    return GetEntry(this, topic);
  }
}

export class EntryValue {
  value: NetworkTableTypes;
  type: string;
  constructor(value: NetworkTableTypes, type: string) {
    this.value = value;
    this.type = type;
  }

  public getValue(): NetworkTableTypes {
    return this.value;
  }

  public isFloat(): boolean {
    return this.type === "Float";
  }

  public isDouble(): boolean {
    return this.type === "Double";
  }

  public isInt(): boolean {
    return this.type === "Int";
  }

  public isBoolean(): boolean {
    return this.type === "Boolean";
  }

  public isString(): boolean {
    return this.type === "String";
  }

  public isByteArray(): boolean {
    return this.type === "ByteArray";
  }

  public isFloatArray(): boolean {
    return this.type === "FloatArray";
  }

  public isDoubleArray(): boolean {
    return this.type === "DoubleArray";
  }

  public isIntArray(): boolean {
    return this.type === "IntArray";
  }

  public isBooleanArray(): boolean {
    return this.type === "BooleanArray";
  }

  public isStringArray(): boolean {
    return this.type === "StringArray";
  }

  public getAsFloat(): number {
    return this.value as number;
  }

  public getAsDouble(): number {
    return this.value as number;
  }

  public getAsInt(): number {
    return this.value as number;
  }

  public getAsBoolean(): boolean {
    return this.value as boolean;
  }

  public getAsString(): string {
    return this.value as string;
  }

  public getAsByteArray(): number[] {
    return this.value as number[];
  }

  public getAsFloatArray(): number[] {
    return this.value as number[];
  }

  public getAsDoubleArray(): number[] {
    return this.value as number[];
  }

  public getAsIntArray(): number[] {
    return this.value as number[];
  }

  public getAsBooleanArray(): boolean[] {
    return this.value as boolean[];
  }

  public getAsStringArray(): string[] {
    return this.value as string[];
  }
}

export class TableEntry {
  key: string;
  timestamp: number;
  value: EntryValue;
  constructor(key: string, timestamp: number, value: EntryValue) {
    this.key = key;
    this.timestamp = timestamp;
    this.value = value;
  }

  public getKey(): string {
    return this.key;
  }

  public getTimestamp(): number {
    return this.timestamp;
  }

  public getValue(): EntryValue {
    return this.value;
  }
}

/**
 * Starts a network table client connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
 * @param identity a string representing the identity of the client
 *
 * This function calls on the native backend and may result in a crash.
 */
export function StartNetworkTableHandler(
  address: number[],
  port: number,
  identity: string
): NetworkTableHandlerId {
  invoke("start_network_table_handler", { address, port, identity }).catch(
    console.error
  );
  return new NetworkTableHandlerId(address, port, identity);
}

/**
 * Checks if a network table client is connected to the specified NetworkTableHandlerId
 * @param handlerId the handlerId of the network table client to check
 *
 * @return a boolean representing whether the client is connected
 *
 * This function calls on the native backend and may result in a crash.
 */
export async function DoesNetworkTableHandlerExist(
  handlerId: NetworkTableHandlerId
): Promise<boolean> {
  return invoke("does_network_table_handler_exist", {
    handlerId,
  }).catch(console.error) as Promise<boolean>;
}

/**
 * Ends a network table client connected to the specified address and port
 * @param handlerId the handlerId of the network table client to stop
 *
 * This function calls on the native backend and may result in a crash.
 */
export function StopNetworkTableHandler(
  handlerId: NetworkTableHandlerId
): void {
  invoke("stop_network_table_handler", { handlerId }).catch(console.error);
}

/**
 * Subscribes to a topic on the network table client associated with the specified handlerId
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to subscribe to
 * @param periodic the period to update the value of the topic at
 * @param all whether or not to subscribe to all entries in the topic
 * @param prefix whether or not to subscribe to all topics with the same prefix
 *
 * This function calls on the native backend and may result in a crash.
 */
export function Subscribe(
  handlerId: NetworkTableHandlerId,
  topic: String,
  periodic?: number,
  all?: boolean,
  prefix?: boolean
): void {
  invoke("subscribe_to_topic", {
    handlerId,
    topic,
    periodic,
    all,
    prefix,
  }).catch(console.error);
}

/**
 * Unsubscribes from a topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to unsubscribe from
 *
 * This function calls on the native backend and may result in a crash.
 */
export function Unsubscribe(
  handlerId: NetworkTableHandlerId,
  topic: String
): void {
  invoke("unsubscribe_from_topic", { handlerId, topic }).catch(console.error);
}

/**
 * Sets the value of an integer topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the integer value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetInteger(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  let primValue: number = Math.round(value.valueOf());
  invoke("set_int_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of an integer array topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the integer array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetIntegerArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  let primValue: number[] = value.map((val: Number) =>
    Math.round(val.valueOf())
  );
  invoke("set_int_array_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a floating point topic, which is a f32 in rust
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the floating point value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetFloat(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  let primValue: number = value.valueOf();
  invoke("set_float_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a floating point array topic, which is a f32 in rust
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the floating point array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetFloatArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  let primValue: number[] = value.map((val: Number) => val.valueOf());
  //maybe should clamp to f32 range
  invoke("set_float_array_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a double topic, which is a f64 in rust
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the double value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetDouble(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  let primValue: number = value.valueOf();
  invoke("set_double_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a double array topic, which is a f64 in rust
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the double array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetDoubleArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  let primValue: number[] = value.map((val: Number) => val.valueOf());
  invoke("set_double_array_topic", {
    handlerId,
    topic,
    value: primValue,
  }).catch(console.error);
}

/**
 * Sets the value of a boolean topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the boolean value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetBoolean(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Boolean
): void {
  let primValue: boolean = value.valueOf();
  invoke("set_boolean_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a boolean array topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the boolean array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetBooleanArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Boolean[]
): void {
  let primValue: boolean[] = value.map((val: Boolean) => val.valueOf());
  invoke("set_boolean_array_topic", {
    handlerId,
    topic,
    value: primValue,
  }).catch(console.error);
}

/**
 * Sets the value of a byte array topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the byte array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetByteArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Uint8Array
): void {
  let byteArray: number[] = Array.from(value);
  invoke("set_byte_array_topic", { handlerId, topic, value: byteArray }).catch(
    console.error
  );
}

/**
 * Sets the value of a string topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the string value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetString(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: String
): void {
  let primValue: string = value.valueOf();
  invoke("set_string_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a string array topic
 * @param handlerId the handlerId of the network table client to set the value of
 * @param topic the topic to set the value of
 * @param value the string array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 */
export function SetStringArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: String[]
): void {
  let primValue: string[] = value.map((val: String) => val.valueOf());
  invoke("set_string_array_topic", {
    handlerId,
    topic,
    value: primValue,
  }).catch(console.error);
}

/**
 * Gets all the subbed topic entries for a given handlerId
 * @param handlerId the handlerId of the network table client to get the entries of
 *
 * @returns an array of TableEntry objects
 */
export async function GetEntries(
  handlerId: NetworkTableHandlerId
): Promise<TableEntry[]> {
  return invoke("get_subbed_entries_values", { handlerId }).catch(
    console.error
  ) as Promise<TableEntry[]>;
}

/**
 * Gets the value of a topic for a given handlerId and path
 * @param handlerId the handlerId of the network table client to get the value of
 * @param path the path of the topic to get the value of
 *
 * @returns the TableEntry of the topic
 */
export async function GetEntry(
  handlerId: NetworkTableHandlerId,
  path: String
): Promise<TableEntry> {
  return invoke("get_subbed_entry_value", { handlerId, path }).catch(
    console.error
  ) as Promise<TableEntry>;
}

/**
 * Gets the server timestamp of the server that the client associated with handlerId is connected to
 * @param handlerId the handlerId of the network table client to get the server timestamp of
 *
 * @returns seconds since the unix epoch
 */
export function GetServerTimestamp(handlerId: NetworkTableHandlerId): number {
  invoke("get_handler_timestamp", { handlerId }).then((timestamp) => {
    return timestamp;
  });
  return 0;
}
