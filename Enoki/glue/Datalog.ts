import { invoke } from "@tauri-apps/api/tauri";
import {
  EnokiObject,
  EnokiValue,
  TimestampedEnokiValue,
  enokiTypeFromTsType,
} from "./EnokiTypes";

export async function readDatalogFile(path: string): Promise<EnokiObject> {
  return invoke("plugin:datalog|read_datalog_file", { path: path });
}

export async function getDaemonData(entry: string): Promise<EnokiObject> {
  return invoke("plugin:datalog|retrieve_dl_daemon_data");
}

export async function sendDatalogMark<T extends EnokiValue>(
  entry: string,
  value: T,
): Promise<void> {
  let timestamp = Date.now() * 1000;
  let timestampedVal = new TimestampedEnokiValue<T>(
    value,
    timestamp,
    enokiTypeFromTsType(typeof value),
  );
  return invoke("plugin:datalog|send_mark", {
    field: entry,
    value: timestampedVal,
  });
}
