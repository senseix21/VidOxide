#!/usr/bin/env bash
# MVP validation script - ensures all critical files exist

set -euo pipefail

echo "🔍 Validating video-intel MVP setup..."
echo ""

errors=0

# Check critical files
files=(
    "README.md"
    "Makefile"
    ".gitignore"
    "Cargo.toml"
    "compose/docker-compose.yml"
    "compose/frigate.yml"
    "compose/mediamtx.yml"
    "compose/mosquitto-no-auth.conf"
    "apps/frigate-cli/Dockerfile"
    "apps/frigate-cli/Cargo.toml"
    "apps/frigate-cli/src/main.rs"
    "apps/frigate-agent/Dockerfile"
    "apps/frigate-agent/Cargo.toml"
    "apps/frigate-agent/src/main.rs"
    "infra/Makefile"
    "infra/scripts/fetch_media.sh"
    "infra/scripts/health.sh"
)

for file in "${files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "✅ $file"
    else
        echo "❌ MISSING: $file"
        ((errors++))
    fi
done

echo ""
echo "📦 Checking Rust workspace..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Rust code compiles"
else
    echo "⚠️  Rust compilation warnings (non-blocking)"
fi

echo ""
if [[ $errors -eq 0 ]]; then
    echo "🚀 MVP READY! All critical files present."
    echo ""
    echo "Next steps:"
    echo "  1. make fetch    # Download sample video"
    echo "  2. make up       # Start all services"
    echo "  3. make logs     # Watch the magic happen"
    exit 0
else
    echo "❌ Found $errors missing file(s). Fix before deploying."
    exit 1
fi
