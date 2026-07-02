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
refinements and the debug-build banner) with the in-flight PR branches
(`feat/model-cross-provider-picker` #9658, `fix/canonical-context-limit-precedence`
#10170, `feat/shell-passthrough` #10177, `feat/copy-command` #10181,
`feat/turn-completion-bell` #10182).

**Before building that binary, bring `local/debug-synthesis` up to date.** Merge
current `origin/main` and any in-flight feature branch whose code the binary
exercises, then build. A synthesis branch that has drifted ships stale behavior
under a fresh-looking banner: on 2026-06-25 a bare `/model` run hit an old
picker (mandatory search box, Esc tore down the TUI) that had already been fixed
on `feat/model-cross-provider-picker`, because synthesis predated the fix.

Build process:

```bash
git checkout local/debug-synthesis
git merge origin/main \
  feat/model-cross-provider-picker \
  fix/canonical-context-limit-precedence \
  feat/shell-passthrough \
  feat/copy-command \
  feat/turn-completion-bell                          # latest master + in-flight PRs
# resolve conflicts favouring upstream for shared code; KEEP the local-only
# work (image-path refinements, banner). Watch for clean-but-wrong merges: a
# textually-clean merge can still break the build when upstream changes an API
# the local code calls. Seen 2026-07-01: the provider refactor dropped the
# session_id arg from Provider::complete and removed Provider::get_model_config,
# so the picker probe and /status compiled on the PR branches but not on
# freshly-merged synthesis until adapted.
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

`/status` (model / provider / mode / token usage) and the image-path base landed
upstream (#9845, #9387); only the refinements and banner above stay local-only.
When merging `origin/main`, favour upstream's `/status` over the old preserved
WIP so the two do not drift.
