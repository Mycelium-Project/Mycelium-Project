'use client'

import { invoke } from '@tauri-apps/api/tauri'

/**
 * Starts a network table client connected to the specified address and port
 * @param address an array of 4 numbers representing the ipv4 address of the server
 * formatted as [0-255, 0-255, 0-255, 0-255] and interpreted as [a, b, c, d] -> a.b.c.d
 * @param port a number representing the port of the server, must be between 0 and 65535
 *
 * This function calls on the native backend and may result in a crash.
 */
function StartNetworkTableHandler(address: number[], port: number): void {
    invoke('start_network_table_handler', { address, port })
        .catch(console.error)
}

// TODO: backend must be implemented
function StopNetworkTableHandler(address: number[], port: number): void {
    invoke('stop_network_table_handler', { address, port })
        .catch(console.error)
}

// TODO: backend must be implemented
function Subscribe(topic: String, periodic?: number, all?: boolean, prefix?: String): void {
    invoke('subscribe_to_topic', { topic, periodic, all, prefix })
        .catch(console.error)
}

// TODO: backend must be implemented
function Publish(topic: String, message: String): void {
    invoke('publish_topic', { topic, message })
        .catch(console.error)
}

// TODO: backend must be implemented
function Unsubscribe(topic: String): void {
    invoke('unsubscribe_from_topic', { topic })
        .catch(console.error)
}

// TODO: backend must be implemented, consider swapping to type safe return
function GetEntry(topic: String): any {
    invoke('get_entry', { topic })
        .then((entry) => { return entry })
        .catch(console.error)
}

// TODO: backend must be implemented
function SetInteger(topic: String, value: number): void {
    invoke('set_integer_entry', { topic, value })
        .catch(console.error)
}

function SetIntigerArray(topic: String, value: number[]): void {
    invoke('set_integer_arr_entry', { topic, value })
        .catch(console.error)
}

function SetFloat(topic: String, value: number): void {
    invoke('set_float_entry', { topic, value })
        .catch(console.error)
}

function SetFloatArray(topic: String, value: number[]): void {
    invoke('set_float_arr_entry', { topic, value })
        .catch(console.error)
}

function SetBoolean(topic: String, value: boolean): void {
    invoke('set_boolean_entry', { topic, value })
        .catch(console.error)
}

function SetBooleanArray(topic: String, value: boolean[]): void {
    invoke('set_boolean_arr_entry', { topic, value })
        .catch(console.error)
}

function SetByteArray(topic: String, value: any[]): void {
    invoke('set_byte_arr_entry', { topic, value })
        .catch(console.error)
}

function SetString(topic: String, value: String): void {
    invoke('set_string_entry', { topic, value })
        .catch(console.error)
}

function SetStringArray(topic: String, value: String[]): void {
    invoke('set_string_arr_entry', { topic, value })
        .catch(console.error)
}