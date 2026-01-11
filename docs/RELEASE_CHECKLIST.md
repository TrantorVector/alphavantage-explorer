# Release Checklist for v0.2.0

## Pre-Release Verification

### Code Quality
- [ ] All tests pass locally: `cargo test --workspace --all-features`
- [ ] Clippy clean: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Formatting verified: `cargo fmt --all -- --check`
- [ ] Release build succeeds: `cargo build --release`
- [ ] Binary works: `./target/release/alphavantage_cli --version` shows "0.2.0"

### Documentation
- [ ] README.md updated with new features
- [ ] CHANGELOG.md updated with v0.2.0 changes
- [ ] docs/manual/ pages reflect new functionality
- [ ] API documentation updated (`cargo doc --no-deps`)
- [ ] Release notes prepared (RELEASE_NOTES.md)

### Version Control
- [ ] Version numbers bumped to 0.2.0 in:
  - [ ] workspace Cargo.toml
  - [ ] All crate Cargo.toml files (via workspace)
- [ ] All changes committed to main branch
- [ ] Branch up-to-date with origin/main
- [ ] No uncommitted changes: `git status` is clean

### Testing
- [ ] Live API integration test (at least one endpoint)
- [ ] Bulk mode test: `cargo run --release -- --symbols AAPL`
- [ ] Granular mode test: `cargo run --release -- overview --symbol NVDA`
- [ ] Mock mode verification
- [ ] CSV endpoint test (e.g., earnings-calendar)

## Release Steps

### 1. Create Git Tag
```bash
git tag -a v0.2.0 -m "Release v0.2.0: Granular API Commands"
```

### 2. Push Tag
```bash
git push origin v0.2.0
```

### 3. Create GitHub Release
- Go to: https://github.com/TrantorVector/alphavantage-explorer/releases/new
- Select tag: `v0.2.0`
- Release title: `v0.2.0 - Granular API Commands`
- Description: Copy from RELEASE_NOTES.md
- Mark as "Latest release"

### 4. Build Release Binaries
```bash
# Build for current platform
cargo build --release

# Binary location
ls -lh target/release/alphavantage_cli
```

### 5. Attach Binaries to GitHub Release
- Upload `alphavantage_cli` binary
- Add checksums:
  ```bash
  sha256sum target/release/alphavantage_cli > alphavantage_cli.sha256
  ```

### 6. Publish Announcement
- [ ] Update repository README badge (if any)
- [ ] Tweet/announce (optional)
- [ ] Update project documentation site (if any)

## Post-Release Verification

- [ ] GitHub release visible at: https://github.com/TrantorVector/alphavantage-explorer/releases/tag/v0.2.0
- [ ] CI/CD passing on main branch
- [ ] Binary downloadable from releases page
- [ ] Tag shows correct version: `git describe --tags`

## Rollback Plan (If Needed)

If critical issues are discovered:

1. **Delete the tag** (if not yet widely used):
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```

2. **Create hotfix branch**:
   ```bash
   git checkout -b hotfix/v0.2.1 v0.2.0
   # Fix issues
   # Tag as v0.2.1
   ```

3. **Mark GitHub release as "Pre-release"** until issues resolved

## Notes

- Ensure GitHub Actions CI passes before creating release
- This is a **non-breaking release** - fully backward compatible
- Migration guide available in RELEASE_NOTES.md for users upgrading from v0.1.0
