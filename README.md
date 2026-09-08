# Docklight

Docklight is a terminal user interface for understanding and safely reclaiming
Docker disk space.

Docker makes it easy to accumulate images, stopped containers, build cache,
networks, and volumes. When disk usage grows, the difficult part is not
running a cleanup command. It is knowing what is taking space, what can be
removed, and what the removal will affect.

Docklight is designed around that decision-making process. It aims to give you
a clear storage overview, rank cleanup opportunities by reclaimable space, and
show an explicit preview before any destructive operation.

> Docklight helps you understand and clean Docker. It does not try to be a
> complete Docker administration dashboard.

## Status

Docklight is currently an early Rust and Ratatui prototype. The application
can display resource categories with counts and estimated reclaimable space,
navigate the list, and exit with `q`. Docker integration and cleanup operations
are not implemented yet.

The intended product direction includes:

- A storage-first overview of Docker resources.
- Images, containers, build cache, networks, and volumes.
- Resource lists with size and reclaimability information.
- Targeted cleanup instead of broad, implicit prune operations.
- A preview of the exact resources affected by an action.
- Explicit confirmation for every destructive operation.
- Recoverable errors when Docker is unavailable or a resource disappears.

## Docklight vs. Lazy Docker

[Lazy Docker](https://github.com/jesseduffield/lazydocker) is a powerful,
general-purpose terminal UI for managing Docker and Docker Compose projects.
It provides an interactive view of containers, images, volumes, networks,
logs, stats, and lifecycle operations.

Docklight has a narrower focus: Docker storage inspection and safe cleanup.

| Area | Docklight | Lazy Docker |
| --- | --- | --- |
| Primary purpose | Understand and reclaim Docker disk space | Manage Docker and Compose resources interactively |
| Main question | “What is using space, and what can I safely remove?” | “What is running, and how do I operate it?” |
| Default perspective | Storage usage and reclaimable bytes | Containers, services, logs, and runtime state |
| Cleanup workflow | Guided, targeted, and preview-first | Resource management actions within a broader dashboard |
| Safety model | Explicitly designed around scope previews and confirmation | Offers management actions as part of its general workflow |
| Resource prioritization | Rank candidates by potential disk recovery | Browse and manage resources by type and state |
| Compose management | Not the primary goal | First-class workflow for Docker Compose projects |
| Logs and live stats | Not the primary goal | Core capabilities |

The projects are complementary rather than mutually exclusive:

- Use **Lazy Docker** when you want to inspect or operate running containers,
  services, logs, and Compose projects.
- Use **Docklight** when Docker storage has become difficult to understand and
  you want a focused, cautious cleanup workflow.

## Safety Principles

Cleanup can destroy data or remove resources that a project still needs.
Docklight therefore follows these principles:

- Never delete resources implicitly during startup, refresh, or navigation.
- Prefer selected-resource removal over broad prune commands.
- Show the resource name or ID, type, estimated reclaimed space, and scope
  before confirmation.
- Require an explicit confirmation keypress and provide a clear cancel path.
- Treat stale resources and unavailable Docker daemons as recoverable errors.
- Use Docker APIs or structured arguments rather than constructing shell
  command strings from resource names or IDs.

## Installation

Docklight requires a stable Rust toolchain and a working Docker installation.
The current prototype can be run with Cargo:

```sh
cargo run
```

Quit the prototype with `q` and move through the resource list with the arrow
keys.

## Development

Run the standard checks before submitting changes:

```sh
cargo fmt -- --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Name

**Docklight** reflects the project's goal: shine a light on Docker's hidden
disk usage before deciding what to remove.

## License

License information will be added before the first release.
