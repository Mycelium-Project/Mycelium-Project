import { invoke } from "@tauri-apps/api/tauri";


export class Terminal {
    public id: number;

    constructor(id: number) {
        this.id = id;
    }

    public async write(data: string): Promise<void> {
        await invoke("plugin:native|write_terminal", { id: this.id, command: data });
    }

    public async kill(): Promise<void> {
        await invoke("plugin:native|kill_terminal", { id: this.id });
    }

    public async readOutTerminal(): Promise<string> {
        return invoke("plugin:native|read_out_terminal", { id: this.id });
    }

    public async readErrTerminal(): Promise<string> {
        return invoke("plugin:native|read_err_terminal", { id: this.id });
    }

    public async isRunning(): Promise<boolean> {
        return invoke("plugin:native|is_running_terminal", { id: this.id });
    }
}

export async function start_terminal(exePath: string, args: string[]): Promise<Terminal> {
    let id = await invoke<number>("plugin:native|start_terminal", { command: exePath, args: args });
    return new Terminal(id);
}
