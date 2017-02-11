//! Sampling abstraction.
//!
//! A [`Sampler`] produces one [`Frame`] per call to [`Sampler::sample`]. The
//! core ships three backends:
//!
//! * [`crate::proc_linux::ProcSampler`] — reads `/proc` on Linux (implemented).
//! * [`crate::fixture::FixtureSampler`] — replays recorded frames on any OS
//!   (implemented, deterministic).
