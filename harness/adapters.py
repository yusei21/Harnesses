"""Integration contracts only; no live connectors are implemented.

Implementations must enforce timeout, approvals, audit, credentials isolation,
schema validation, bounded retries and explicit cancellation.
"""
from typing import Any, Protocol

class ModelAdapter(Protocol):
    def generate(self, instructions: str, context: dict[str, Any],
                 schema: dict[str, Any]) -> dict[str, Any]: ...

class ToolAdapter(Protocol):
    def invoke(self, name: str, arguments: dict[str, Any],
               approval_token: str | None = None) -> dict[str, Any]: ...

class CheckpointStore(Protocol):
    def save(self, run_id: str, state: dict[str, Any]) -> None: ...
    def load(self, run_id: str) -> dict[str, Any] | None: ...

class MemoryStore(Protocol):
    def search(self, query: str, limit: int = 5) -> list[dict[str, Any]]: ...
    def propose(self, entry: dict[str, Any]) -> str: ...
