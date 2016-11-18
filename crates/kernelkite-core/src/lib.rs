//! # kernelkite-core
//!
//! Core engine for **kernelkite**, a Linux-first local developer-loop process
//! profiler. It samples process resource usage over time, assembles
//! deterministic [`ProfileBundle`](model::ProfileBundle)s, and produces
//! before/after comparisons for a build/test loop.
//!
//! ## Backends
//!
//! | Backend  | Status            | Platforms      |
//! |----------|-------------------|----------------|
//! | `proc`   | **implemented**   | Linux only     |
//! | `fixture`| **implemented**   | any OS         |
//! | `ebpf`   | *future, stubbed* | none (feature) |
