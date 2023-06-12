"use client";

import { invoke } from "@tauri-apps/api/tauri";

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
  port: number
): void {
  invoke("start_network_table_handler", { address, port }).catch(console.error);
}

/**
 * Checks if a network table client is connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
 * @return a boolean representing whether or not the client is connected
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: Front end does not work for some fudging reason
 */
export const DoesNetworkTableHandlerExist = async (
  address: number[],
  port: number
): Promise<boolean> => {
  return (await invoke("does_network_table_handler_exist", {
    address,
    port,
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
export function StopNetworkTableHandler(address: number[], port: number): void {
  invoke("stop_network_table_handler", { address, port }).catch(console.error);
}

/**
 * Subscribes to a topic
 * @param topic the topic to subscribe to
 * @param periodic the period to update the value of the topic at
 * @param all whether or not to subscribe to all entries in the topic
 * @param prefix whether or not to subscribe to all topics with the same prefix
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function Subscribe(
  topic: String,
  periodic?: number,
  all?: boolean,
  prefix?: boolean
): void {
  invoke("subscribe_to_topic", { topic, periodic, all, prefix }).catch(
    console.error
  );
}

/**
 * Publishes a value to a topic
 * @param topic the topic to publish to
 * @param message the value to publish
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function Publish(topic: String, message: any): void {
  invoke("publish_topic", { topic, message }).catch(console.error);
}

/**
 * Unsubscribes from a topic
 * @param topic the topic to unsubscribe from
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function Unsubscribe(topic: String): void {
  invoke("unsubscribe_from_topic", { topic }).catch(console.error);
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
  invoke("get_entry", { topic })
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
export function SetInteger(topic: String, value: number): void {
  invoke("set_integer_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of an integer array topic
 * @param topic the topic to set the value of
 * @param value the integer array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetIntegerArray(topic: String, value: number[]): void {
  invoke("set_integer_arr_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a floating point topic, which is a f32 in rust
 * @param topic the topic to set the value of
 * @param value the floating point value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetFloat(topic: String, value: number): void {
  invoke("set_float_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a floating point array topic, which is a f32 in rust
 * @param topic the topic to set the value of
 * @param value the floating point array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetFloatArray(topic: String, value: number[]): void {
  invoke("set_float_arr_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a double topic, which is a f64 in rust
 * @param topic the topic to set the value of
 * @param value the double value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetDouble(topic: String, value: number): void {
    invoke("set_double_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a double array topic, which is a f64 in rust
 * @param topic the topic to set the value of
 * @param value the double array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetDoubleArray(topic: String, value: number[]): void {
    invoke("set_double_arr_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a boolean topic
 * @param topic the topic to set the value of
 * @param value the boolean value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetBoolean(topic: String, value: boolean): void {
  invoke("set_boolean_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a boolean array topic
 * @param topic the topic to set the value of
 * @param value the boolean array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetBooleanArray(topic: String, value: boolean[]): void {
  invoke("set_boolean_arr_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a byte array topic
 * @param topic the topic to set the value of
 * @param value the byte array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetByteArray(topic: String, value: any[]): void {
  invoke("set_byte_arr_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a string topic
 * @param topic the topic to set the value of
 * @param value the string value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetString(topic: String, value: String): void {
  invoke("set_string_entry", { topic, value }).catch(console.error);
}

/**
 * Sets the value of a string array topic
 * @param topic the topic to set the value of
 * @param value the string array value to set the topic to
 *
 * This function calls on the native backend and may result in a crash.
 * TODO: backend must be implemented
 */
export function SetStringArray(topic: String, value: String[]): void {
  invoke("set_string_arr_entry", { topic, value }).catch(console.error);
}
