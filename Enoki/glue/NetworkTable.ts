
import { invoke } from "@tauri-apps/api/tauri";
import { EnokiObject, EnokiValue, TimestampedEnokiValue } from "./EnokiTypes";

export class NetworkTableClientId {
    ip: number[];
    port: number;
    identity: string;
    constructor(ip: number[], port: number, identity: string) {
        this.ip = ip;
        this.port = port;
        this.identity = identity;
    }
}

export class NetworkTablePubbedTopic<T extends EnokiValue> {
    topic: string;
    type: string;
    clientId: NetworkTableClientId;

    constructor(topic: string, type: string, clientId: NetworkTableClientId) {
        this.topic = topic;
        this.type = type;
        this.clientId = clientId;
    }

    public getTopicName(): string {
        return this.topic;
    }

    public setValue(value: T): void {
        let timestamp = Date.now() * 1000;
        let timestampedVal = new TimestampedEnokiValue<T>(value, timestamp, this.type);
        invoke("plugin:nt|set_topic_value", {clientId: this.clientId, topic: this.topic, value: timestampedVal})
    }

    public setValueTimestamp(value: T, timestamp: number): void {
        let timestampedVal = new TimestampedEnokiValue<T>(value, timestamp, this.type);
        invoke("plugin:nt|set_topic_value", {clientId: this.clientId, topic: this.topic, value: timestampedVal})
    }

    unpublish(): void {
        invoke("plugin:nt|unpublish_topic", {clientId: this.clientId, topic: this.topic})
    }

    public getType(): string {
        return this.type;
    }
}

export class NetworkTableSubscription {
    clientId: NetworkTableClientId;
    topic: string;
    cached_history: EnokiObject;

    constructor(clientId: NetworkTableClientId, topic: string) {
        this.clientId = clientId;
        this.topic = topic;
        this.cached_history = new EnokiObject();
    }

    public async getSubbedData(): Promise<EnokiObject> {
        return invoke("plugin:nt|get_subbed_data", {clientId: this.clientId, topic: this.topic});
    }

    public async getSubbedDataWithHistory(): Promise<EnokiObject> {
        let data = await invoke<EnokiObject>(
            "plugin:nt|get_subbed_data_with_history",
            {clientId: this.clientId, topic: this.topic, after: this.cached_history.timestamp});
        data.mergeHistory(this.cached_history);
        this.cached_history = data;
        this.cached_history.fields = [];
        return data;
    }

    public clearCache(): void {
        this.cached_history = new EnokiObject();
    }
}

export async function start_network_table_client(ip: number[], port: number, identity: string): Promise<NetworkTableClient> {
    return new NetworkTableClient(
        await invoke<NetworkTableClientId>
            ("plugin:nt|start_network_table_client", {ip: ip, port: port, identity: identity}));
}

export class NetworkTableClient {
    topicMap: Map<string, NetworkTablePubbedTopic<any>>;
    subMap: Map<string, NetworkTableSubscription>;
    clientId: NetworkTableClientId;

    constructor(clientId: NetworkTableClientId) {
        this.clientId = clientId;
        this.topicMap = new Map();
        this.subMap = new Map();
    }

    public createBoolTopic(topic: string): NetworkTablePubbedTopic<boolean> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<boolean>(topic, "Bool", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createDoubleTopic(topic: string): NetworkTablePubbedTopic<number> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number>(topic, "Double", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createFloatTopic(topic: string): NetworkTablePubbedTopic<number> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number>(topic, "Float", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createIntTopic(topic: string): NetworkTablePubbedTopic<number> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number>(topic, "Int", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createStringTopic(topic: string): NetworkTablePubbedTopic<string> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<string>(topic, "String", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createRawTopic(topic: string): NetworkTablePubbedTopic<Uint8Array> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<Uint8Array>(topic, "Raw", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public creatBoolArrayTopic(topic: string): NetworkTablePubbedTopic<boolean[]> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<boolean[]>(topic, "BoolArray", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createDoubleArrayTopic(topic: string): NetworkTablePubbedTopic<number[]> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number[]>(topic, "DoubleArray", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createFloatArrayTopic(topic: string): NetworkTablePubbedTopic<number[]> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number[]>(topic, "FloatArray", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createIntArrayTopic(topic: string): NetworkTablePubbedTopic<number[]> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<number[]>(topic, "IntArray", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public createStringArrayTopic(topic: string): NetworkTablePubbedTopic<string[]> {
        if (this.topicMap.has(topic)) {
            throw new Error("Topic already exists");
        }
        let topic_obj = new NetworkTablePubbedTopic<string[]>(topic, "StringArray", this.clientId);
        this.topicMap.set(topic, topic_obj);
        return topic_obj;
    }

    public unpublishTopic(topic: NetworkTablePubbedTopic<any>): void {
        topic.unpublish();
        this.topicMap.delete(topic.topic);
    }

    public getPubbedTopicNames(): string[] {
        return Array.from(this.topicMap.keys());
    }

    /**
     * Subscribes to a topic on the network table client associated with this clientId
     * @param topic the topic to subscribe to
     * @param periodic the period to update the value of the topic at
     * @param all whether or not to subscribe to all entries in the topic
     * @param prefix whether or not to subscribe to all topics with the same prefix
     */
    public subscribe(
        topic: String,
        periodic?: number,
        all?: boolean,
        prefix?: boolean
    ): void {
        invoke("plugin:nt|subscribe_to_topic", {
            clientId: this.clientId,
            topic,
            periodic,
            all,
            prefix,
        });
    }

    /**
     * Remnants of the subscription will still remain in the snapshots
     * @param topic the topic to unsubscribe from
     */
    public unsubscribe(topic: String): void {
        invoke("plugin:nt|unsubscribe_from_topic", {
            clientId: this.clientId,
            topic,
        });
    }

    public stop(): void {
        invoke("plugin:nt|stop_network_table_client", {clientId: this.clientId})
    }

    public isStopped(): Promise<boolean> {
        return invoke<boolean>("plugin:nt|is_network_table_client_stopped", {clientId: this.clientId})
    }
}


