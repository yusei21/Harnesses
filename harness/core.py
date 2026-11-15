"""Typed contracts and a deliberately deterministic reference orchestrator.

This implementation does NOT call an LLM or execute external tools.
Adapters that do so must implement their own security and approval mechanisms.
"""
from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum
from typing import Any, Protocol
import hashlib
import json
import pathlib
import uuid

ROOT = pathlib.Path(__file__).resolve().parents[1]


class Risk(str, Enum):
    READ = "read"
    WRITE = "write"
    EXTERNAL = "external"
    DESTRUCTIVE = "destructive"


class Status(str, Enum):
    COMPLETED = "completed"
    APPROVAL_REQUIRED = "approval_required"
    REJECTED = "rejected"


@dataclass(frozen=True)
class Task:
    task_id: str
    goal: str
    requested_by: str = "local"
    max_steps: int = 12
    budget_usd: float = 1.0

    def __post_init__(self):
        if not self.goal.strip() or self.max_steps < 1 or self.budget_usd < 0:
            raise ValueError("Invalid task contract")


@dataclass(frozen=True)
class Assignment:
    agent: str
    purpose: str
    risk: Risk = Risk.READ


@dataclass
class Result:
    task_id: str
    status: Status
    assignments: list[Assignment] = field(default_factory=list)
    artifacts: list[dict[str, Any]] = field(default_factory=list)
    events: list[dict[str, Any]] = field(default_factory=list)
    reason: str = ""

    def as_dict(self) -> dict[str, Any]:
        return {
            "task_id": self.task_id, "status": self.status.value,
            "assignments": [{"agent": a.agent, "purpose": a.purpose, "risk": a.risk.value}
                            for a in self.assignments],
            "artifacts": self.artifacts, "events": self.events, "reason": self.reason,
        }


class Worker(Protocol):
    def run(self, task: Task, assignment: Assignment) -> dict[str, Any]: ...


class OfflineWorker:
    """Demonstration only: returns an explanatory artifact, no LLM/tool calls."""
    def run(self, task: Task, assignment: Assignment) -> dict[str, Any]:
        return {
            "kind": "offline-demonstration",
            "agent": assignment.agent,
            "summary": f"Simulação para: {assignment.purpose}",
            "limitations": "No external tools, coding or inference were performed.",
        }


class Policy:
    """Fail-closed policy. No model can self-grant approval."""
    HIGH_RISK = frozenset({Risk.WRITE, Risk.EXTERNAL, Risk.DESTRUCTIVE})

    def allowed(self, risk: Risk, approval: bool = False) -> bool:
        return risk == Risk.READ or (approval and risk in self.HIGH_RISK)


def choose_assignments(goal: str) -> list[Assignment]:
    """Deterministic heuristic for demo; real routing requires evaluated adapter."""
    s = goal.lower()
    result = [Assignment("planner", "Decompor objetivo e identificar critérios")]
    if any(word in s for word in ("pesquis", "investig", "document", "requisito")):
        result.append(Assignment("researcher", "Pesquisar evidências e contexto"))
    if any(word in s for word in ("código", "codigo", "api", "bug", "corrig", "implement", "refator")):
        result.append(Assignment("engineer", "Propor mudanças técnicas sem aplicá-las"))
        result.append(Assignment("reviewer", "Revisar riscos e qualidade"))
    if any(word in s for word in ("teste", "qualidade", "valid", "eval")):
        result.append(Assignment("evaluator", "Definir e verificar critérios de avaliação"))
    return result[:5]


class Orchestrator:
    def __init__(self, worker: Worker | None = None, policy: Policy | None = None):
        self.worker = worker or OfflineWorker()
        self.policy = policy or Policy()

    def run(self, task: Task, approval: bool = False,
            assignments: list[Assignment] | None = None) -> Result:
        plan = assignments if assignments is not None else choose_assignments(task.goal)
        if len(plan) > task.max_steps:
            return Result(task.task_id, Status.REJECTED, plan, reason="step_limit")
        events: list[dict[str, Any]] = []
        artifacts: list[dict[str, Any]] = []
        for i, assignment in enumerate(plan):
            event = {"index": i, "agent": assignment.agent, "risk": assignment.risk.value}
            if not self.policy.allowed(assignment.risk, approval):
                event["outcome"] = "blocked"
                events.append(event)
                return Result(task.task_id, Status.APPROVAL_REQUIRED, plan, artifacts,
                              events, "human_approval_required")
            try:
                artifact = self.worker.run(task, assignment)
            except Exception as exc:
                event["outcome"] = "error"
                events.append(event)
                return Result(task.task_id, Status.REJECTED, plan, artifacts, events,
                              f"worker_error:{type(exc).__name__}")
            if not isinstance(artifact, dict):
                return Result(task.task_id, Status.REJECTED, plan, artifacts, events,
                              "invalid_worker_output")
            artifacts.append(artifact)
            event["outcome"] = "completed"
            events.append(event)
        return Result(task.task_id, Status.COMPLETED, plan, artifacts, events)


def trace_envelope(result: Result) -> dict[str, Any]:
    """Exclude task prompt and artifact contents by default to avoid secret leakage."""
    body = result.as_dict()
    safe = {"task_id": body["task_id"], "status": body["status"],
            "events": body["events"], "reason": body["reason"]}
    safe["timestamp_utc"] = datetime.now(timezone.utc).isoformat()
    safe["digest"] = hashlib.sha256(json.dumps(safe, sort_keys=True).encode()).hexdigest()
    return safe


def make_task(goal: str) -> Task:
    return Task(task_id=str(uuid.uuid4()), goal=goal)
