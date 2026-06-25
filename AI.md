# AI.md: local agent doc for the goose fork

Canonical agent doc for **this machine's** checkout of the goose fork
(`jlmalone/goose`, forked from `aaif-goose/goose`). Upstream engineering
practice (build, test, lint, style, structure) lives in `AGENTS.md`; read that
first. This file holds the **local-only** workflow that must never leak into an
upstream PR, which is why it lives on the local integration branch
(`local/debug-synthesis`) and is deliberately absent from feature/PR branches.

Up one level: the "Goose" section of `~/AI.md`.

---

## 🔁 Keep `local/debug-synthesis` current · part of every local build · non-negotiable

The daily-driver binary (`/opt/homebrew/bin/goose` -> `target/release/goose`,
run via the `goose_dangerously` alias) is built from **`local/debug-synthesis`**,
the integration branch that combines the local-only work (image-path
refinements, the debug-build banner, the `/status` command) with the active
feature branches under review.

**Before building that binary, bring `local/debug-synthesis` up to date.** Merge
current `origin/main` and any in-flight feature branch whose code the binary
exercises, then build. A synthesis branch that has drifted ships stale behavior
under a fresh-looking banner: on 2026-06-25 a bare `/model` run hit an old
picker (mandatory search box, Esc tore down the TUI) that had already been fixed
on `feat/model-cross-provider-picker`, because synthesis predated the fix.

Build process:

```bash
git checkout local/debug-synthesis
git merge origin/main feat/<active-feature-branch>   # bring in latest before building
# resolve conflicts favouring upstream for shared code; KEEP the local-only
# work (image-path refinements, banner, /status). Watch for clean-but-wrong
# merges: renamed Session fields, duplicated help lines.
cargo check -p goose-cli -p goose                    # confirm the merge compiles
cargo build --release -p goose-cli                   # updates target/release/goose
```

When a feature branch you are reviewing gets new commits, merge them into
synthesis before relying on the binary to test them. Never test against a
binary you have not just rebuilt from current synthesis.

---

## Local-only work carried on `local/debug-synthesis`

Preserve these across every merge; they are not upstreamed:

- **Image-path refinements** on top of upstream #9387 (quote terminators,
  URL-boundary guards, longest-existing-path preference) in
  `crates/goose-providers/src/images.rs`.
- **Debug-build banner** (build number + sha/branch/time) in
  `crates/goose-cli/src/session/output.rs`.
- **`/status`** session command (model / provider / mode / token usage).
