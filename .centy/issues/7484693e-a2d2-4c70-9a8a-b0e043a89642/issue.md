# Support git repository check for centy init

When creating a centy folder via centy init:

1. Add support for checking if the current directory is inside a git repository
2. The daemon should support an optional flag/parameter to bypass the git check
3. Return appropriate error/warning responses when not in a git repo

This is the daemon-side support for the CLI feature that requires users to be in a git repository when running centy init.

## Acceptance Criteria
- [ ] Add git repository detection logic (if needed in daemon)
- [ ] Support bypass parameter in init/reconciliation endpoints
- [ ] Return appropriate status when not in a git repo
