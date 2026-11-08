#!/usr/bin/env bash
# Build carlink, install it to ~/.local/bin, and link the agent skill into ~/.claude/skills.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
cargo build --release --locked --manifest-path "$here/Cargo.toml"
mkdir -p "$HOME/.local/bin" "$HOME/.claude/skills"
install -m 0755 "$here/target/release/carlink" "$HOME/.local/bin/carlink"
ln -sfn "$here/skills/carlink" "$HOME/.claude/skills/carlink"
echo "installed: $HOME/.local/bin/carlink; skill: $HOME/.claude/skills/carlink"
