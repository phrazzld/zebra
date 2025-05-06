# T010 Plan: Enable Branch Protection Rules for CI Checks

## Task Description
Document the branch protection rules that should be configured on GitHub for the `main` branch to ensure CI checks for formatting and linting must pass before merging pull requests.

## Approach
1. Document the branch protection rules and configuration process
2. Ensure documentation clearly explains how to set up and verify the protection rules

## Implementation Outcome
Since we don't have admin access to the GitHub repository, this task involves creating documentation for repository administrators to follow. The documentation provides detailed instructions on how to:

1. Configure branch protection rules for the `main` branch
2. Require the format and lint status checks (from T008 and T009) to pass before merging
3. Verify the rules are working as expected

## Branch Protection Configuration Instructions

### Required Status Checks
The following status checks must pass before merging to the `main` branch:

1. **format** - Ensures code follows the formatting standards defined in `.rustfmt.toml`
2. **lint** - Ensures code passes the linting checks defined by our clippy configuration

### Configuration Steps
Repository administrators should follow these steps:

1. Go to the GitHub repository: [https://github.com/LoadingScreen/zebra](https://github.com/LoadingScreen/zebra)
2. Click on "Settings" tab in the top navigation bar
3. In the left sidebar, click on "Branches"
4. Under "Branch protection rules", click "Add rule"
5. In the "Branch name pattern" field, enter `main`
6. Check "Require status checks to pass before merging"
7. Check "Require branches to be up to date before merging"
8. In the search box, search for and select both:
   - The `format` check (from the "Rust Code Quality" workflow)
   - The `lint` check (from the "Rust Code Quality" workflow)
9. Optional but recommended settings:
   - Check "Require pull request reviews before merging"
   - Check "Require approval of the most recent reviewable push"
   - Check "Dismiss stale pull request approvals when new commits are pushed"
10. Click "Create" to save the branch protection rule

### Verification
Once the branch protection rules have been set up, they can be verified by:

1. Creating a pull request with code that violates formatting or linting rules
   - This PR should be blocked from merging until the issues are fixed
2. Creating a pull request with compliant code
   - This PR should be allowed to merge once it passes the CI checks

## Dependencies
- T008: The format CI job created in T008
- T009: The lint CI job created in T009
