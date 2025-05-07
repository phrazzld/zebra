# Branch Protection Rules for ZebraSign

This document describes the branch protection rules that should be configured for the ZebraSign repository to enforce code quality standards through CI checks.

## Overview

Branch protection rules ensure that pull requests meet certain criteria before they can be merged into the protected branch (in this case, `main`). For ZebraSign, we require that CI checks for formatting and linting pass before merging.

## Required Status Checks

The following status checks must pass before merging to the `main` branch:

1. **format** - Ensures code follows the formatting standards defined in `.rustfmt.toml`
2. **lint** - Ensures code passes the linting checks defined by our clippy configuration
3. **test** - Ensures all tests pass across the workspace, including unit and integration tests

## Configuration Steps

To set up branch protection rules, a repository administrator should follow these steps:

1. Go to the GitHub repository: [https://github.com/LoadingScreen/zebra](https://github.com/LoadingScreen/zebra)
2. Click on "Settings" tab in the top navigation bar
3. In the left sidebar, click on "Branches"
4. Under "Branch protection rules", click "Add rule"
5. In the "Branch name pattern" field, enter `main`
6. Check "Require status checks to pass before merging"
7. Check "Require branches to be up to date before merging"
8. In the search box, search for and select the following:
   - The `format` check (from the "Rust Code Quality" workflow)
   - The `lint` check (from the "Rust Code Quality" workflow)
   - The `test` check (from the "Rust Code Quality" workflow)
9. Optional but recommended settings:
   - Check "Require pull request reviews before merging"
   - Check "Require approval of the most recent reviewable push"
   - Check "Dismiss stale pull request approvals when new commits are pushed"
10. Click "Create" to save the branch protection rule

## Verification

Once the branch protection rules have been set up, they can be verified by:

1. Creating a pull request with code that violates formatting or linting rules
   - This PR should be blocked from merging until the issues are fixed
2. Creating a pull request with compliant code
   - This PR should be allowed to merge once it passes the CI checks

## Bypassing Branch Protection

In exceptional circumstances, repository administrators can bypass branch protection by:

1. Going to the pull request
2. Scrolling to the bottom of the page
3. Finding the "Merge pull request" button dropdown
4. Selecting "Merge without waiting for requirements to be met (bypass branch protections)"

This should be used only in emergency situations and should be documented with a clear explanation.
