#!/bin/bash
# Simple direct test runner for tutorial CUJs

set -e

echo "=========================================="
echo "CUJ 1: Fresh Setup"
echo "=========================================="

echo "Step 1: Dry-run"
devstrap --config /home/tester/tests/cuj1-fresh-setup.toml --dry-run | head -20

echo -e "\nStep 2: Install"
devstrap --config /home/tester/tests/cuj1-fresh-setup.toml --yes

echo -e "\nStep 3: Verify"
which rg fdfind batcat cmake make git curl

echo -e "\nStep 4: Second run (idempotency)"
devstrap --config /home/tester/tests/cuj1-fresh-setup.toml --yes | tail -5

echo -e "\n✓ CUJ 1 PASSED"

echo -e "\n=========================================="
echo "CUJ 2: Package Removal"
echo "=========================================="

# Clean state
rm -rf /home/tester/.config/devstrap

echo "Step 1: Install with extra packages"
devstrap --config /home/tester/tests/cuj2-before-removal.toml --yes

echo -e "\nStep 2: Verify extra packages"
which fzf jq htop

echo -e "\nStep 3: Dry-run removal"
devstrap sync --config /home/tester/tests/cuj2-after-removal.toml --prune --dry-run | grep -A5 "remove\|prune" || echo "No removal section found"

echo -e "\nStep 4: Execute removal"
devstrap sync --config /home/tester/tests/cuj2-after-removal.toml --prune --yes

echo -e "\nStep 5: Verify removal"
if ! command -v fzf >/dev/null 2>&1; then
  echo "✓ fzf removed"
else
  echo "✗ fzf still present"
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "✓ jq removed"
else
  echo "✗ jq still present"
  exit 1
fi

if ! command -v htop >/dev/null 2>&1; then
  echo "✓ htop removed"
else
  echo "✗ htop still present"
  exit 1
fi

echo -e "\nStep 6: Verify kept packages"
which rg batcat cmake

echo -e "\n✓ CUJ 2 PASSED"

echo -e "\n=========================================="
echo "CUJ 4: Team Onboarding"
echo "=========================================="

# Clean state
rm -rf /home/tester/.config/devstrap

echo "Step 1: Install team config"
devstrap --config /home/tester/tests/cuj4-team-config.toml --yes

echo -e "\nStep 2: Verify tools"
which git curl rg fdfind fzf cmake make gcc g++

echo -e "\nStep 3: Verify compilers"
gcc --version | head -1
g++ --version | head -1

echo -e "\n✓ CUJ 4 PASSED"

echo -e "\n=========================================="
echo "CUJ 6: Dry-run Previews"
echo "=========================================="

# Clean state
rm -rf /home/tester/.config/devstrap

echo "Step 1: Dry-run shows changes"
devstrap --config /home/tester/tests/cuj1-fresh-setup.toml --dry-run | grep -E "DRY-RUN|dry-run" | head -3

echo -e "\nStep 2: Verify nothing installed"
if ! command -v batcat >/dev/null 2>&1; then
  echo "✓ Dry-run did not install packages"
else
  echo "✗ Dry-run installed packages (should not happen!)"
  exit 1
fi

echo -e "\n✓ CUJ 6 PASSED"

echo -e "\n=========================================="
echo "ALL TESTS PASSED!"
echo "=========================================="
