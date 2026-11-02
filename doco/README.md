# Project documentation

These English documents describe the current `yummy_egui` implementation and maintainer workflows. Start with the [project README](../README.md) for a usage example or the [public API](https://docs.rs/yummy_egui) for type-level documentation.

| Document | Read it for |
| --- | --- |
| [Architecture](architecture.md) | Host/library boundaries, module dependencies, state and data flow |
| [Theming contract](specs/theming.md) | TOML replacement rules, validation, context application and compatibility |
| [Widgets and lists contract](specs/widgets-and-lists.md) | Input semantics, layout, selection, IDs and caller obligations |
| [Development](development.md) | Local builds, tests, native examples, CI coverage and package checks |
| [Publishing](publishing.md) | Credentials, preflight, cargo-release, manual Cargo and GitHub Actions releases |

## Documentation boundaries

- `architecture.md` records implemented structure, not future plans.
- `specs/` records current public behavior and compatibility contracts. Investigate code/spec disagreements before editing either side.
- Development and publishing guides describe operations; README translations remain entry points rather than copies of those guides.
- Add decisions only for important durable rationale. Do not invent decisions or change packages to reconstruct implementation history; use Git and release records for that history.

Update current documents when their facts or contracts change. Temporary material under `doco/tmp/` and any completed or archived change packages are not current specifications. Current guides and specs are included in the crate archive; temporary material and change packages are not.
