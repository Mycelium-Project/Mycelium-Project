export type EnokiValue =
  | number
  | string
  | boolean
  | number[]
  | string[]
  | boolean[]
  | Uint8Array;

export function enokiTypeFromTsType<T extends EnokiValue>(
  ts_type: string,
): string {
  let ts_type_lower: string = ts_type.toLowerCase();
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

export class EnokiField<T extends EnokiValue> {
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
  fields: Array<EnokiField<EnokiValue> | undefined>;
  history: Array<Array<TimestampedEnokiValue<EnokiValue>> | undefined>;
  paths: Map<string, number>;
  timestamp: number;

  constructor() {
    this.fields = [];
    this.history = [];
    this.paths = new Map();
    this.timestamp = 0;
  }

  public getField<T extends EnokiValue>(
    key: string,
  ): EnokiField<T> | undefined {
    let index: number | undefined = this.paths.get(key);
    if (index === undefined) {
      return undefined;
    }
    let field: EnokiField<EnokiValue> | undefined = this.fields[index];
    if (field === undefined) {
      return undefined;
    }
    return field as EnokiField<T>;
  }

  public getFields(): Array<EnokiField<EnokiValue>> {
    return this.fields.filter((field): boolean => field !== undefined) as Array<
      EnokiField<EnokiValue>
    >;
  }

  public getFieldHistory<T extends EnokiValue>(
    key: string,
  ): Array<TimestampedEnokiValue<T>> | undefined {
    let index: number | undefined = this.paths.get(key);
    if (index === undefined) {
      return undefined;
    }
    return this.history[index] as Array<TimestampedEnokiValue<T>>;
  }

  public mergeHistory(other: EnokiObject): void {
    for (let key of other.paths.keys()) {
      let index_other: number | undefined = other.paths.get(key);
      let index_self: number | undefined = this.paths.get(key);
      if (index_other === undefined || index_self === undefined) {
        continue;
      }
      let history_other: TimestampedEnokiValue<EnokiValue>[] | undefined =
        other.history[index_other];
      let history_self: TimestampedEnokiValue<EnokiValue>[] | undefined =
        this.history[index_self];
      if (history_other === undefined) {
        continue;
      }
      if (history_self === undefined) {
        this.history[index_self] = history_other;
        continue;
      }
      let merged_history: TimestampedEnokiValue<EnokiValue>[] =
        history_self.concat(history_other);
      merged_history.sort((a, b) => a.getTimestamp() - b.getTimestamp());
      this.history[index_self] = merged_history;
    }
  }

  public getTimestamp(): number {
    return this.timestamp;
  }

  public getFieldKeys(): Array<string> {
    return Array.from(this.paths.keys());
  }
}
