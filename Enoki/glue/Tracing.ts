import { invoke } from "@tauri-apps/api/tauri";

export function TraceInfo(message: any, ...optionalParams: any[]): void {
    //get the line number of the caller and the file
    let stack = new Error().stack ?? "";
    let caller = stack.split("\n")[2];
    let callerParts = caller.split(" ");
    let inter = callerParts[callerParts.length - 1].split("/").pop();
    let callerFile = inter?.split(":")[0];
    let callerLine = inter?.split(":")[1].split(")")[0];
    if (optionalParams.length === 0) {
        invoke("plugin:native|tracing_frontend", { level: "info", msg: message.toString(),
            line: callerLine,
            file: callerFile});
    } else {
        invoke("plugin:native|tracing_frontend", { level: "info",
            msg: message.toString() + " " + optionalParams.map((p) => p.toString()).join(" "),
            line: callerLine,
            file: callerFile});
    }
}

export function TraceWarn(message: any, ...optionalParams: any[]): void {
    //get the line number of the caller and the file
    let stack = new Error().stack ?? "";
    let caller = stack.split("\n")[2];
    let callerParts = caller.split(" ");
    let inter = callerParts[callerParts.length - 1].split("/").pop();
    let callerFile = inter?.split(":")[0];
    let callerLine = inter?.split(":")[1].split(")")[0];
    if (optionalParams.length === 0) {
        invoke("plugin:native|tracing_frontend", { level: "warn", msg: message.toString(),
            line: callerLine,
            file: callerFile});
    } else {
        invoke("plugin:native|tracing_frontend", { level: "warn",
            msg: message.toString() + " " + optionalParams.map((p) => p.toString()).join(" "),
            line: callerLine,
            file: callerFile});
    }
}

export function TraceError(message: any, ...optionalParams: any[]): void {
    //get the line number of the caller and the file
    let stack = new Error().stack ?? "";
    let caller = stack.split("\n")[2];
    let callerParts = caller.split(" ");
    let inter = callerParts[callerParts.length - 1].split("/").pop();
    let callerFile = inter?.split(":")[0];
    let callerLine = inter?.split(":")[1].split(")")[0];
    if (optionalParams.length === 0) {
        invoke("plugin:native|tracing_frontend", { level: "error", msg: message.toString(),
            line: callerLine,
            file: callerFile});
    } else {
        invoke("plugin:native|tracing_frontend", { level: "error",
            msg: message.toString() + " " + optionalParams.map((p) => p.toString()).join(" "),
            line: callerLine,
            file: callerFile});
    }
}

/**
 * A tracing substitute for console.error. This will log the error to the native tracing pipeline.
 * @param data The data to be logged. This will be converted to a string.
 */
export function ErrorHook(data: any[]): void {
    let stack = new Error().stack ?? "";
    let caller = stack.split("\n")[2];
    let callerParts = caller.split(" ");
    let inter = callerParts[callerParts.length - 1].split("/").pop();
    let callerFile = inter?.split(":")[0];
    let callerLine = inter?.split(":")[1].split(")")[0];
    invoke("plugin:native|tracing_frontend", { level: "error", msg: data.map((p) => p.toString()).join(" "),
        line: callerLine,
        file: callerFile});
}
