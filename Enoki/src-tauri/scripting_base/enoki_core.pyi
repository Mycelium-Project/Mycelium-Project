
from typing import List, Tuple, TypeAlias, Union


EnokiValue: TypeAlias = Union[bool, int, float, str, List[bool], List[int], List[float], List[str], bytes]

""" A timestamp in microseconds since the epoch. """
EnokiTimestamp = int

class TimestampedEnokiValue:
    """A value with a timestamp."""
    value: EnokiValue
    timestamp: int

    def __init__(self, value: EnokiValue, timestamp: int) -> None: ...

class EnokiField:
    """A field in an Enoki Object"""
    value: EnokiValue
    timestamp: int
    key: str

    def __init__(self, value: EnokiValue, timestamp: int, key: str) -> None: ...

class EnokiObject:
    timestamp: int

    def field(self, key: str) -> EnokiField: ...

    def fields(self) -> List[EnokiField]: ...

    def field_history(self, key: str) -> List[TimestampedEnokiValue]: ...

    def field_keys(self) -> List[str]: ...

def now() -> int: ...

class NetworkTable:
    class NetworkTableClientId:
        ip: Tuple(int, int, int, int)
        port: int
        identity: str


    class NetworkTablePubbedTopic:
        def __init__(self, topic: str, type_name: str, client_id: NetworkTable.NetworkTableClientId) -> None: ...

        def topic_name(self) -> str: ...

        def topic_type(self) -> str: ...

        def set_value(self, value: EnokiValue) -> None: ...

        def unpublish(self) -> None: ...


    class NetworkTableSubscription:
        def __init__(self, topic: str, client_id: NetworkTable.NetworkTableClientId) -> None: ...

        def get_subbed_data(self) -> EnokiObject: ...

        def get_subbed_data_with_history(self, after: int) -> EnokiObject: ...

    def start_network_table_client(
        ip: Tuple(int, int, int, int),
        port: int,
        identity: str): "NetworkTableClientId"