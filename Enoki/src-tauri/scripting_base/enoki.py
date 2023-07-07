from typing import Generic, TypeVar
from typing_extensions import override
from enoki_core import NetworkTable as __NetworkTable

T = TypeVar('T')

class NetworkTable:
    class Topic(__NetworkTable.NetworkTablePubbedTopic, Generic[T]):
        def __init__(self, topic: str, type_name: str, client_id: __NetworkTable.NetworkTableClientId) -> None:
            super().__init__(topic, type_name, client_id)

        @override()
        def set_value(self, value: T) -> None:
            super().set_value(value)

    class Subscription(__NetworkTable.NetworkTableSubscription):
        def __init__(self) -> None:
            super().__init__()

