# Systems engineering mono repo

Capstone projects for an 11-module Rust systems roadmap: lock-free structures, async I/O, storage engines, performance, parallel query execution, transactions, consensus, an EVM, P2P gossip, a RISC-V kernel, and deployment. One directory per capstone. Each capstone implements a paper or spec and has an objective check (test suite, external tool, or benchmark).

Plans, progress and handoff notes are kept outside this repo. Never commit specs, plans or session notes here.

## Roles

The user writes the code. Claude guides: specs, milestones, tests and checks, reviews, hints. Do not hand over finished implementations of learning-critical parts unless asked. Apply existing tools and libraries; do not rebuild them (Module 1 EBR is the one exception).

## Layout

| Dir | Modules | Notes |
|---|---|---|
| `lockfree/` | 1 | Harris list + EBR |
| `kv/` | 2, 3, 4, 6, 7 | One growing system: server, LSM, YCSB driver, MVCC, Raft. Cargo workspace, one crate per layer |
| `tpch/` | 5 | Standalone |
| `evm/` | 8 | |
| `p2p/` | 9 | Part B uses `evm/` |
| `kernel/` | 10 | `no_std`, RISC-V target |
| `deploy/` | 11 | CI, container, k8s manifests for `kv/` |

There is no root Cargo workspace: the kernel needs its own target and build settings. Each directory is its own Cargo project or workspace. Create a directory when its module starts, not before.

## Environment

- macOS host for editing and most tests.
- Linux ARM VM for `perf`, `rr`, `bpftrace`, sanitizers. Hardware performance counters may not be exposed in VMs on Apple Silicon; use bare-metal Linux for counter work if `perf stat` reports `<not supported>`.
