
export type EnokiValue =
    | number
    | string
    | boolean
    | number[]
    | string[]
    | boolean[]
    | Uint8Array;

export function enokiTypeFromTsType<T extends EnokiValue>(ts_type: string): string {
    let ts_type_lower = ts_type.toLowerCase();
    if (ts_type_lower === "number") {
        return "Double";
    } else if (ts_type_lower === "string") {
        return "String";
    } else if (ts_type_lower === "boolean") {
        return "Boolean";
    } else if (ts_type_lower === "number[]") {
        return "DoubleArray";
    } else if (ts_type_lower === "string[]") {
        return "StringArray";
    } else if (ts_type_lower === "boolean[]") {
        return "BooleanArray";
    } else if (ts_type_lower === "uint8array") {
        return "ByteArray";
    } else {
        throw new Error("Invalid type");
    }
}

export type EnokiTimestamp = number;

export class TimestampedEnokiValue<T extends EnokiValue> {
    type: string;
    value: T;
    timestamp: number;

    constructor(value: T, timestamp: number, type: string) {
        this.value = value;
        this.timestamp = timestamp;
        this.type = type;
    }

    public getValue(): T {
        return this.value;
    }

    /**
     * @returns The timestamp of the value in microseconds since the epoch.
     */
    public getTimestamp(): number {
        return this.timestamp;
    }
}

export class EnkokiField<T extends EnokiValue> {
    value: TimestampedEnokiValue<T>;
    key: string;

    constructor(value: TimestampedEnokiValue<T>, key: string) {
        this.value = value;
        this.key = key;
    }

    public getValue(): T {
        return this.value.getValue();
    }

    /**
     * @returns The timestamp of the value in microseconds since the epoch.
     */
    public getTimestamp(): number {
        return this.value.getTimestamp();
    }

    public getKey(): string {
        return this.key;
    }
}

export class EnokiObject {
    fields: Array<EnkokiField<EnokiValue> | undefined>;
    history: Array<Array<TimestampedEnokiValue<EnokiValue>> | undefined>;
    paths: Map<string, number>;
    timestamp: number;

    constructor() {
        this.fields = [];
        this.history = [];
        this.paths = new Map();
        this.timestamp = 0;
    }

    public getField<T extends EnokiValue>(key: string): EnkokiField<T> | undefined {
        let index = this.paths.get(key);
        if (index === undefined) {
            return undefined;
        }
        let field =  this.fields[index];
        if (field === undefined) {
            return undefined;
        }
        return field as EnkokiField<T>;
    }

    public getFields(): Array<EnkokiField<EnokiValue>> {
        return this.fields.filter((field) => field !== undefined) as Array<EnkokiField<EnokiValue>>;
    }

    public getFieldHistory<T extends EnokiValue>(key: string): Array<TimestampedEnokiValue<T>> | undefined {
        let index = this.paths.get(key);
        if (index === undefined) {
            return undefined;
        }
        return this.history[index] as Array<TimestampedEnokiValue<T>>;
    }

    public getTimestamp(): number {
        return this.timestamp;
    }

    public getFieldKeys(): Array<string> {
        return Array.from(this.paths.keys());
    }
}