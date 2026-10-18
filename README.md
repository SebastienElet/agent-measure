# agent-measure

Local agent measurement, extracted from Arnes. Prepared for
`@sebastienelet/agent-measure@1.0.0`; no registry release is qualified yet.

Use Bun 1.4.3 to acquire the exact published version once available. Intended native
targets: macOS ARM64 and Linux x64 glibc. Installation acquires files only.
`agent-measure --help` exposes `hook`, `list`, `finish`, `feedback`, `outcome`,
`pr-verdict` and `report`. Agent hooks receive their original payload on stdin;
activation is separate. Git is used by operations that observe repository state.

The historical store remains `${XDG_STATE_HOME:-$HOME/.local/state}/dotfiles/agent-harness`.
Capture persists compact metadata and events, excluding prompts, responses, transcripts,
local paths and raw session IDs. Stop events never create successful outcomes.
Outcomes are declarations associated with a named oracle, not proof of its execution.
Version 2 runs expire after 60 days of inactivity; version 1 runs remain untouched.
PR timelines retain their separate 90-day inactivity policy. Historical `finish` and
`feedback` commands keep their version 1 contracts. Do not activate two collectors.

Cursor local plugin selection is an explicit JSON array of plugin IDs, independent of
Arnes. Supply an absolute path through `AGENT_MEASURE_CURSOR_PLUGINS`, or place the array
at `$HOME/.config/agent-measure/cursor-plugins.json`, e.g. `["example"]`. IDs must be
single path components. Only declared, confined local plugin roots are fingerprinted;
marketplace activation is not observable. Missing selection is reported as a limitation.
Capture failures remain advisory and do not block agent execution.

Contribute with Rust 1.98.1, Bun 1.4.3 and Git: `bun install --frozen-lockfile`,
`cargo test --locked`, `bun run build`, `bun test tests/launcher.test.mjs`.
Native packages include executable SHA-256 and source revision in `BUILD.json`.
Publication uses only the manual CI workflow, npm trusted publishers (OIDC) and
provenance. Configure trust for both native variants and the entry package; publish
variants before the entry package. Inspect registry state after a partial failure.
Never delete or rewrite a user's store to install or roll back an executable.
