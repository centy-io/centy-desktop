# Add macOS code signing and notarization

## Problem

macOS displays 'Centy is damaged and can't be opened' when launching the app because it's not code-signed or notarized.

## Solution

Implement Apple code signing and notarization in the GitHub Actions release workflow.

## Prerequisites

- Sign up for Apple Developer Program ($99/year)
- Create Developer ID Application certificate
- Export certificate to Base64 format
- Generate App-Specific Password for notarization
- Add GitHub Secrets: APPLE_CERTIFICATE, APPLE_CERTIFICATE_PASSWORD, APPLE_SIGNING_IDENTITY, APPLE_ID, APPLE_PASSWORD, APPLE_TEAM_ID

## Code Changes

1. Update .github/workflows/release.yml - Add Apple signing env vars to tauri-action
2. Update src-tauri/tauri.conf.json - Add entitlements and hardenedRuntime
3. Create src-tauri/Entitlements.plist - Define app capabilities for notarization

## References

- https://v2.tauri.app/distribute/sign/macos/
- https://v2.tauri.app/distribute/macos-application-bundle/
