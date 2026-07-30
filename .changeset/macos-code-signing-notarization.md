---
"centy-desktop": patch
---

Wire up Apple code signing and notarization in the release workflow, and add the required entitlements, so macOS Gatekeeper no longer blocks the app with an "app is damaged" error on unsigned builds (#2). Signing remains a no-op until Apple developer secrets are provisioned in CI.
