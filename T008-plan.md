# T008 Plan: Create CI job for rustfmt check

## Task Description
Create a CI job that checks code formatting using rustfmt.

## Approach
1. Check if `.github/workflows/` directory exists, create it if needed
2. Check if `rust_quality.yml` file exists, create it if needed
3. Implement a job named `format` that:
   - Runs on push and pull_request triggers
   - Uses the same rustfmt configuration (from T005)
   - Runs `cargo fmt --all --check` to verify formatting
   - Fails the CI check if formatting issues are found

## Implementation Steps
1. Create the necessary directory structure if not present
2. Write/update the workflow file with the format job
3. Verify the workflow file is valid YAML
4. Verify that the job configuration is correctly defined

## Testing Approach
Since we can't directly test the GitHub Actions workflow without pushing to GitHub, we'll:
1. Ensure the workflow file is syntactically correct
2. Follow the verification steps described in the ticket when the code is pushed
