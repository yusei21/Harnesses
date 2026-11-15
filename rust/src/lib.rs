//! Rust-first, read-only orchestration; model inference delegated to a Python bridge.
//! No tool execution, MCP or remote A2A is implemented.
pub mod bridge;
pub mod core;
pub mod ops;

pub use bridge::PythonGateway;
pub use core::{
    validate_plan, Agent, Event, Gateway, GatewayFuture, ModelRequest, ModelResponse, Plan,
    Policy, Risk, Route, RunError, RunResult, Runner, Stage,
};
