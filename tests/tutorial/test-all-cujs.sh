#!/bin/bash
# Comprehensive test script for all Tutorial CUJs
# Runs inside Docker container

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test result tracking
TESTS_PASSED=0
TESTS_FAILED=0
FAILED_TESTS=()

# Helper functions
print_header() {
    echo -e "\n${BLUE}════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
    ((TESTS_PASSED++))
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
    ((TESTS_FAILED++))
    FAILED_TESTS+=("$1")
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

# Cleanup function
cleanup() {
    print_info "Cleaning up test environment..."
    rm -f /home/tester/.config/devstrap/devstrap.state
    rm -f /home/tester/devstrap.lock
}

# Verify command exists
verify_command() {
    local cmd=$1
    local name=$2
    if command -v "$cmd" &> /dev/null; then
        print_success "$name is installed and in PATH"
        return 0
    else
        print_error "$name is NOT installed or not in PATH"
        return 1
    fi
}

# Verify package installed via APT
verify_apt_package() {
    local pkg=$1
    if dpkg -l "$pkg" 2>/dev/null | grep -q "^ii"; then
        print_success "APT package '$pkg' is installed"
        return 0
    else
        print_error "APT package '$pkg' is NOT installed"
        return 1
    fi
}

# Verify package NOT installed via APT
verify_apt_package_removed() {
    local pkg=$1
    if dpkg -l "$pkg" 2>/dev/null | grep -q "^ii"; then
        print_error "APT package '$pkg' should be removed but is still installed"
        return 1
    else
        print_success "APT package '$pkg' is correctly removed"
        return 0
    fi
}

# Check state file for package
check_state_file() {
    local pkg=$1
    local should_exist=$2

    if [ -f "/home/tester/.config/devstrap/devstrap.state" ]; then
        if grep -q "packages.$pkg" /home/tester/.config/devstrap/devstrap.state; then
            if [ "$should_exist" = "true" ]; then
                print_success "Package '$pkg' tracked in state file"
                return 0
            else
                print_error "Package '$pkg' should NOT be in state file"
                return 1
            fi
        else
            if [ "$should_exist" = "false" ]; then
                print_success "Package '$pkg' correctly removed from state file"
                return 0
            else
                print_error "Package '$pkg' NOT found in state file"
                return 1
            fi
        fi
    else
        print_error "State file does not exist"
        return 1
    fi
}

################################################################################
# CUJ 1: Fresh Setup on New Machine
################################################################################
test_cuj1_fresh_setup() {
    print_header "CUJ 1: Fresh Setup on New Machine"

    cleanup

    print_info "Step 1: Preview installation with --dry-run"
    if devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --dry-run > /tmp/cuj1-dryrun.log 2>&1; then
        print_success "Dry-run completed successfully"

        # Verify dry-run output
        if grep -q "This is a dry-run" /tmp/cuj1-dryrun.log || grep -q "DRY-RUN" /tmp/cuj1-dryrun.log; then
            print_success "Dry-run output indicates preview mode"
        else
            print_warning "Could not verify dry-run mode from output"
        fi
    else
        print_error "Dry-run failed"
        cat /tmp/cuj1-dryrun.log
    fi

    print_info "Step 2: Install packages with --yes"
    if devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --yes > /tmp/cuj1-install.log 2>&1; then
        print_success "Installation completed successfully"
    else
        print_error "Installation failed"
        cat /tmp/cuj1-install.log
        return 1
    fi

    print_info "Step 3: Verify all packages installed"

    # Verify core utilities
    verify_command git "git"
    verify_command curl "curl"

    # Verify development tools (Ubuntu names)
    verify_command rg "ripgrep (rg)"
    verify_command fdfind "fd-find (fdfind)"
    verify_command batcat "bat (batcat)"

    # Verify build tools
    verify_command cmake "cmake"
    verify_command make "make"

    print_info "Step 4: Verify state file created"
    if [ -f "/home/tester/.config/devstrap/devstrap.state" ]; then
        print_success "State file created"

        # Check some packages are tracked
        check_state_file "git" "true"
        check_state_file "ripgrep" "true"
    else
        print_error "State file NOT created"
    fi

    print_info "Step 5: Test idempotency (run again)"
    if devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --yes > /tmp/cuj1-second.log 2>&1; then
        print_success "Second run completed successfully"

        # Check for "in sync" or similar message
        if grep -qi "in sync\|already installed" /tmp/cuj1-second.log; then
            print_success "Idempotency confirmed (packages already in sync)"
        else
            print_warning "Could not confirm idempotency from output"
        fi
    else
        print_error "Second run failed"
        cat /tmp/cuj1-second.log
    fi
}

################################################################################
# CUJ 2: Removing Unused Packages
################################################################################
test_cuj2_removing_packages() {
    print_header "CUJ 2: Removing Unused Packages"

    cleanup

    print_info "Step 1: Install initial set with extra packages"
    if devstrap --config /home/tester/tutorial-tests/cuj2-before-removal.toml --yes > /tmp/cuj2-install.log 2>&1; then
        print_success "Initial installation completed"
    else
        print_error "Initial installation failed"
        cat /tmp/cuj2-install.log
        return 1
    fi

    print_info "Step 2: Verify extra packages installed"
    verify_command fzf "fzf"
    verify_command jq "jq"
    verify_command tree "tree"

    print_info "Step 3: Preview removal with --prune --dry-run"
    if devstrap sync --config /home/tester/tutorial-tests/cuj2-after-removal.toml --prune --dry-run > /tmp/cuj2-prune-dryrun.log 2>&1; then
        print_success "Prune dry-run completed"

        # Check output mentions packages to remove
        if grep -qi "remove\|prune" /tmp/cuj2-prune-dryrun.log; then
            print_success "Dry-run shows packages will be removed"
        else
            print_warning "Could not verify removal preview"
        fi
    else
        print_error "Prune dry-run failed"
        cat /tmp/cuj2-prune-dryrun.log
    fi

    print_info "Step 4: Execute removal with --prune --yes"
    if devstrap sync --config /home/tester/tutorial-tests/cuj2-after-removal.toml --prune --yes > /tmp/cuj2-prune.log 2>&1; then
        print_success "Prune completed successfully"
    else
        print_error "Prune failed"
        cat /tmp/cuj2-prune.log
        return 1
    fi

    print_info "Step 5: Verify packages removed"

    # fzf, jq, tree should be removed
    if ! command -v fzf &> /dev/null; then
        print_success "fzf correctly removed"
    else
        print_error "fzf still present (should be removed)"
    fi

    if ! command -v jq &> /dev/null; then
        print_success "jq correctly removed"
    else
        print_error "jq still present (should be removed)"
    fi

    if ! command -v tree &> /dev/null; then
        print_success "tree correctly removed"
    else
        print_error "tree still present (should be removed)"
    fi

    print_info "Step 6: Verify kept packages still present"
    verify_command git "git (should be kept)"
    verify_command rg "ripgrep (should be kept)"

    print_info "Step 7: Verify state file updated"
    check_state_file "fzf" "false"
    check_state_file "jq" "false"
    check_state_file "tree" "false"
    check_state_file "git" "true"
    check_state_file "ripgrep" "true"
}

################################################################################
# CUJ 4: Team Onboarding
################################################################################
test_cuj4_team_onboarding() {
    print_header "CUJ 4: Team Onboarding"

    cleanup

    print_info "Step 1: Preview team config"
    if devstrap --config /home/tester/tutorial-tests/cuj4-team-config.toml --dry-run > /tmp/cuj4-dryrun.log 2>&1; then
        print_success "Team config dry-run completed"
    else
        print_error "Team config dry-run failed"
        cat /tmp/cuj4-dryrun.log
    fi

    print_info "Step 2: Install team environment with --yes (CI mode)"
    if devstrap --config /home/tester/tutorial-tests/cuj4-team-config.toml --yes > /tmp/cuj4-install.log 2>&1; then
        print_success "Team environment installation completed"
    else
        print_error "Team environment installation failed"
        cat /tmp/cuj4-install.log
        return 1
    fi

    print_info "Step 3: Verify team tools installed"
    verify_command git "git"
    verify_command curl "curl"
    verify_command rg "ripgrep"
    verify_command fdfind "fd"
    verify_command fzf "fzf"
    verify_command cmake "cmake"
    verify_command make "make"

    print_info "Step 4: Verify system languages (compilers)"
    verify_command gcc "gcc (C compiler)"
    verify_command g++ "g++ (C++ compiler)"

    print_info "Step 5: Simulate second team member setup (idempotency)"
    if devstrap --config /home/tester/tutorial-tests/cuj4-team-config.toml --yes > /tmp/cuj4-second.log 2>&1; then
        print_success "Second team member setup completed"
    else
        print_error "Second setup failed"
    fi
}

################################################################################
# CUJ 6: Previewing Changes Safely
################################################################################
test_cuj6_dry_run_previews() {
    print_header "CUJ 6: Previewing Changes Safely"

    cleanup

    print_info "Step 1: Install baseline environment"
    if devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --yes > /tmp/cuj6-baseline.log 2>&1; then
        print_success "Baseline environment installed"
    else
        print_error "Baseline installation failed"
        cat /tmp/cuj6-baseline.log
        return 1
    fi

    print_info "Step 2: Preview adding packages (dry-run)"
    if devstrap --config /home/tester/tutorial-tests/cuj2-before-removal.toml --dry-run > /tmp/cuj6-preview-add.log 2>&1; then
        print_success "Preview of adding packages completed"

        # Verify it's a dry-run
        if grep -q "DRY-RUN\|dry-run" /tmp/cuj6-preview-add.log; then
            print_success "Confirmed dry-run mode"
        fi

        # Verify packages mentioned
        if grep -qi "fzf\|jq\|tree" /tmp/cuj6-preview-add.log; then
            print_success "New packages shown in preview"
        fi
    else
        print_error "Preview failed"
        cat /tmp/cuj6-preview-add.log
    fi

    print_info "Step 3: Verify no changes made by dry-run"
    if ! command -v fzf &> /dev/null && \
       ! command -v jq &> /dev/null && \
       ! command -v tree &> /dev/null; then
        print_success "Dry-run did not install packages (correctly)"
    else
        print_error "Dry-run installed packages (should not happen!)"
    fi

    print_info "Step 4: Preview removal (dry-run)"
    # First actually install the packages
    devstrap --config /home/tester/tutorial-tests/cuj2-before-removal.toml --yes > /dev/null 2>&1

    # Now preview removal
    if devstrap sync --config /home/tester/tutorial-tests/cuj2-after-removal.toml --prune --dry-run > /tmp/cuj6-preview-remove.log 2>&1; then
        print_success "Preview of removal completed"

        if grep -qi "remove\|prune" /tmp/cuj6-preview-remove.log; then
            print_success "Removal shown in preview"
        fi
    else
        print_error "Removal preview failed"
        cat /tmp/cuj6-preview-remove.log
    fi

    print_info "Step 5: Verify no removals made by dry-run"
    if command -v fzf &> /dev/null && \
       command -v jq &> /dev/null && \
       command -v tree &> /dev/null; then
        print_success "Dry-run did not remove packages (correctly)"
    else
        print_error "Dry-run removed packages (should not happen!)"
    fi
}

################################################################################
# Additional Integration Tests
################################################################################
test_state_file_tracking() {
    print_header "Additional Test: State File Tracking"

    cleanup

    print_info "Install packages and verify state tracking"
    devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --yes > /dev/null 2>&1

    if [ -f "/home/tester/.config/devstrap/devstrap.state" ]; then
        print_success "State file exists"

        # Check TOML structure
        if grep -q "\[packages\." /home/tester/.config/devstrap/devstrap.state; then
            print_success "State file has correct TOML structure"
        else
            print_error "State file missing expected structure"
        fi

        # Check timestamps
        if grep -q "installed_at" /home/tester/.config/devstrap/devstrap.state; then
            print_success "State file tracks installation timestamps"
        else
            print_error "State file missing timestamps"
        fi

        # Check method tracking
        if grep -q "method" /home/tester/.config/devstrap/devstrap.state; then
            print_success "State file tracks installation method"
        else
            print_error "State file missing installation method"
        fi

        # Display state file for verification
        print_info "State file contents:"
        cat /home/tester/.config/devstrap/devstrap.state
    else
        print_error "State file not created"
    fi
}

test_package_manager_update() {
    print_header "Additional Test: Package Manager Update"

    print_info "Verify APT update runs before installations"

    # Clear APT cache to ensure update is needed
    sudo rm -rf /var/lib/apt/lists/*

    # Run devstrap and check if it updates
    if devstrap --config /home/tester/tutorial-tests/cuj1-fresh-setup.toml --yes --verbose > /tmp/apt-update-test.log 2>&1; then
        print_success "Installation completed"

        # Check if APT update was mentioned in logs
        if grep -qi "apt-get update\|updating apt" /tmp/apt-update-test.log; then
            print_success "APT update was executed"
        else
            print_warning "Could not confirm APT update from logs"
        fi

        # Verify packages installed successfully
        if command -v git &> /dev/null; then
            print_success "Packages installed after update"
        fi
    else
        print_error "Installation failed"
        cat /tmp/apt-update-test.log
    fi
}

################################################################################
# Main Test Execution
################################################################################
main() {
    print_header "devstrap Tutorial CUJ Testing Suite"
    print_info "Testing all Critical User Journeys in Docker environment"
    print_info "Ubuntu 22.04 ARM64 with APT package manager"
    echo ""

    # Create test directory structure
    mkdir -p /home/tester/tutorial-tests

    # Copy test configs
    cp /home/tester/cuj*.toml /home/tester/tutorial-tests/ 2>/dev/null || true

    # Verify devstrap is available
    if ! command -v devstrap &> /dev/null; then
        print_error "devstrap not found in PATH"
        exit 1
    fi

    print_success "devstrap binary found: $(which devstrap)"
    print_info "devstrap version: $(devstrap --version)"
    echo ""

    # Run all tests
    test_cuj1_fresh_setup
    test_cuj2_removing_packages
    test_cuj4_team_onboarding
    test_cuj6_dry_run_previews
    test_state_file_tracking
    test_package_manager_update

    # Print summary
    print_header "Test Summary"
    echo -e "Total tests passed: ${GREEN}${TESTS_PASSED}${NC}"
    echo -e "Total tests failed: ${RED}${TESTS_FAILED}${NC}"

    if [ ${TESTS_FAILED} -eq 0 ]; then
        echo -e "\n${GREEN}✓ All tests passed!${NC}\n"
        exit 0
    else
        echo -e "\n${RED}✗ Some tests failed:${NC}"
        for test in "${FAILED_TESTS[@]}"; do
            echo -e "  ${RED}✗${NC} $test"
        done
        echo ""
        exit 1
    fi
}

# Run main function
main
