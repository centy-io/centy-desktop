# Init repo

This will be the centy desktop wrapper for the centy-app, this will be a tauri desktop app that wrapes the centy-app, it should be standalone so its not require any internet

## Implementation Plan

### Architecture
Tauri v2 desktop app with git submodules structure:
```
centy-desktop/
├── src-tauri/           # Tauri Rust backend
│   ├── src/main.rs      # Entry point, sidecar management
│   ├── tauri.conf.json  # App configuration
│   └── bin/             # Sidecar binaries
├── centy-app/           # Git submodule (React frontend)
├── centy-daemon/        # Git submodule (Rust backend)
├── package.json
└── scripts/             # Build scripts
```

### Key Decisions
- **Tauri v2** - Latest version with better sidecar support
- **Git submodules** - Reference centy-app and centy-daemon
- **All platforms** - macOS, Windows, Linux from start
- **Sidecar approach** - Bundle daemon binary, Tauri manages lifecycle

### Implementation Steps

1. **Initialize Project Structure**
   - Add centy-app as git submodule
   - Add centy-daemon as git submodule
   - Create package.json with pnpm

2. **Initialize Tauri v2**
   - Configure tauri.conf.json
   - Set frontendDist to centy-app/dist
   - Configure sidecar for centy-daemon

3. **Sidecar Configuration**
   - Build daemon for each platform target
   - Configure externalBin in Tauri
   - Implement lifecycle management in main.rs

4. **Tauri Backend**
   - Start daemon on app launch
   - Monitor daemon health
   - Graceful shutdown on close
   - Handle daemon restart on crash

5. **Frontend Integration**
   - Configure VITE_DAEMON_URL for Tauri
   - Build centy-app for production

6. **Platform Bundles**
   - macOS: .dmg, .app (Universal binary)
   - Windows: .msi, .exe (NSIS)
   - Linux: .deb, .rpm, .AppImage

### Files to Create
- `package.json` - Root package with scripts
- `src-tauri/Cargo.toml` - Tauri dependencies
- `src-tauri/tauri.conf.json` - App configuration
- `src-tauri/src/main.rs` - Sidecar management
- `src-tauri/capabilities/default.json` - Permissions
- `scripts/build-sidecar.sh` - Cross-platform builds
- `.gitmodules` - Submodule definitions
