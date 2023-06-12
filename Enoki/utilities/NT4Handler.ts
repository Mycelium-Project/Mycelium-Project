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
   * @return a boolean representing whether or not the client is connected
   *
   * This function calls on the native backend and may result in a crash.
   */
  public DoesNetworkTableHandlerExist(): boolean {
    DoesNetworkTableHandlerExist(this).then((result) => {
      return result;
    });
    return false;
  }

  /**
   * Ends the network table client associated with this handlerId
   *
   * This function calls on the native backend and may result in a crash.
   */
  public StopNetworkTableHandler(): void {
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
  public Subscribe(
    topic: String,
    periodic?: number,
    all?: boolean,
    prefix?: boolean
  ): void {
    Subscribe(this, topic, periodic, all, prefix);
  }

  public Unsubscribe(topic: String): void {
    Unsubscribe(this, topic);
  }

  public SetEntry(topic: String, value: NetworkTableTypes): void {
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
    } else if (value instanceof Array) {
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
        }
      } else {
        SetDoubleArray(this, topic, []);
      }
    }
  }
}

/**
 * Starts a network table client connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
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
 * Checks if a network table client is connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
 * @return a boolean representing whether or not the client is connected
 *
 * This function calls on the native backend and may result in a crash.
 */
export const DoesNetworkTableHandlerExist = async (
  handlerId: NetworkTableHandlerId
): Promise<boolean> => {
  return (await invoke("does_network_table_handler_exist", {
    handlerId,
  })) as boolean;
};

/**
 * Ends a network table client connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
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
 * @param topic the topic to unsubscribe from
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function Unsubscribe(
  handlerId: NetworkTableHandlerId,
  topic: String
): void {
  invoke("unsubscribe_from_topic", { handlerId, topic }).catch(console.error);
}

/**
 * Gets the value of a topic
 * @param topic the topic to get the value of
 * @return the value of the topic
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function GetEntry(topic: String): any {
  invoke("get_topic", { topic })
    .then((entry) => {
      return entry;
    })
    .catch(console.error);
}

/**
 * Sets the value of an integer topic
 * @param topic the topic to set the value of
 * @param value the integer value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetInteger(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  var primValue = Math.round(value.valueOf());
  invoke("set_int_topic", { handlerId, topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of an integer array topic
 * @param topic the topic to set the value of
 * @param value the integer array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetIntegerArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  var primValue = value.map((val) => Math.round(val.valueOf()));
  invoke("set_int_array_topic", { topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a floating point topic, which is a f32 in rust
 * @param topic the topic to set the value of
 * @param value the floating point value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetFloat(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  var primValue = value.valueOf();
  invoke("set_float_topic", { topic, value: primValue }).catch(console.error);
}

/**
 * Sets the value of a floating point array topic, which is a f32 in rust
 * @param topic the topic to set the value of
 * @param value the floating point array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetFloatArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  var primValue = value.map((val) => val.valueOf());
  //maybe should clamp to f32 range
  invoke("set_float_array_topic", { topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a double topic, which is a f64 in rust
 * @param topic the topic to set the value of
 * @param value the double value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetDouble(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number
): void {
  var primValue = value.valueOf();
  invoke("set_double_topic", { topic, value: primValue }).catch(console.error);
}

/**
 * Sets the value of a double array topic, which is a f64 in rust
 * @param topic the topic to set the value of
 * @param value the double array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetDoubleArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Number[]
): void {
  var primValue = value.map((val) => val.valueOf());
  invoke("set_double_array_topic", { topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a boolean topic
 * @param topic the topic to set the value of
 * @param value the boolean value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetBoolean(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Boolean
): void {
  var primValue = value.valueOf();
  invoke("set_boolean_topic", { topic, value: primValue }).catch(console.error);
}

/**
 * Sets the value of a boolean array topic
 * @param topic the topic to set the value of
 * @param value the boolean array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetBooleanArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Boolean[]
): void {
  var primValue = value.map((val) => val.valueOf());
  invoke("set_boolean_array_topic", { topic, value: primValue }).catch(
    console.error
  );
}

/**
 * Sets the value of a byte array topic
 * @param topic the topic to set the value of
 * @param value the byte array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetByteArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: Uint8Array
): void {
  var byteArray: number[] = Array.from(value);
  invoke("set_byte_array_topic", { topic, value: byteArray }).catch(
    console.error
  );
}

/**
 * Sets the value of a string topic
 * @param topic the topic to set the value of
 * @param value the string value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetString(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: String
): void {
  var primValue = value.valueOf();
  invoke("set_string_topic", { topic, value: primValue }).catch(console.error);
}

/**
 * Sets the value of a string array topic
 * @param topic the topic to set the value of
 * @param value the string array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetStringArray(
  handlerId: NetworkTableHandlerId,
  topic: String,
  value: String[]
): void {
  var primValue = value.map((val) => val.valueOf());
  invoke("set_string_array_topic", { topic, value: primValue }).catch(
    console.error
  );
}
