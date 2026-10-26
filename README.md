# centy-desktop

Centy Desktop — a local-first issue and documentation tracker, packaged as a
[Tauri](https://tauri.app/) 2 application. The window is the
[`centy-app`](https://github.com/centy-io/centy-app) Next.js frontend; the
[`centy-daemon`](https://github.com/centy-io/centy-daemon) binary ships
alongside it as a Tauri sidecar, so the app runs without a separately installed
daemon.

## Repository layout

```
centy-app/      # git submodule — Next.js frontend (pnpm workspace package)
centy-daemon/   # git submodule — Rust daemon, built into src-tauri/bin as a sidecar
src-tauri/      # Tauri shell: Rust entrypoint, tauri.conf.json, bundle config
scripts/        # build-sidecar.sh — cross-compiles the daemon for the host target
```

Both frontend and daemon are git submodules, so clone recursively:

```bash
git clone --recurse-submodules https://github.com/centy-io/centy-desktop.git
# or, in an existing checkout:
git submodule update --init --recursive
```

## Prerequisites

- Node.js 20 or newer and pnpm 10 (see `packageManager` in `package.json`)
- A Rust toolchain, for `src-tauri/` and the daemon sidecar
- The [Tauri 2 system prerequisites](https://tauri.app/start/prerequisites/) for your platform

```bash
pnpm install
```

## Development

```bash
pnpm dev
```

This starts the `centy-app` dev server and `tauri dev` together. Tauri loads the
frontend from `http://localhost:5180`.

## Build

```bash
pnpm build
```

That runs three steps in order:

1. `pnpm build:frontend` — `next build` in `centy-app`, emitting `centy-app/out`
2. `pnpm build:sidecar` — `scripts/build-sidecar.sh` builds `centy-daemon` in
   release mode for the host target and copies it to
   `src-tauri/bin/centy-daemon-<target-triple>`
3. `pnpm tauri build` — bundles the installer for the host platform

The sidecar script detects the host target automatically and supports
`x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`,
`aarch64-unknown-linux-gnu`, and `x86_64-pc-windows-msvc`.

Individual steps are available as `pnpm build:frontend`, `pnpm build:sidecar`,
`pnpm tauri:dev`, and `pnpm tauri:build`.

## Scripts

| Script | What it does |
|---|---|
| `pnpm dev` | `centy-app` dev server plus `tauri dev` |
| `pnpm build` | frontend → sidecar → Tauri bundle |
| `pnpm build:frontend` | `next build` in `centy-app` only |
| `pnpm build:sidecar` | build and stage the `centy-daemon` sidecar binary |
| `pnpm tauri` | pass arguments straight through to the Tauri CLI |
| `pnpm tauri:dev` | `tauri dev` without starting the frontend separately |
| `pnpm tauri:build` | `tauri build` only, assuming the frontend and sidecar exist |

## Contributing

CI (`.github/workflows/ci.yml`) runs the frontend tests, frontend lint, the
frontend build, and a per-platform Tauri build check.

Every pull request that touches files outside `.changeset/` and `.github/` must
add a changeset — `.github/workflows/changesets.yml` enforces it:

```bash
npx changeset add           # with a release note
npx changeset add --empty   # for changes that need no release note
```
