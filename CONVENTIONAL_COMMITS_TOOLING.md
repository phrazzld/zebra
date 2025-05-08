# Conventional Commits Enforcement Tools - Research & Recommendations

## Overview

This document presents research findings and recommendations for implementing tooling to enforce the [Conventional Commits](https://www.conventionalcommits.org/) standard in the ZebraSign project. Conventional Commits provide a standardized format for commit messages, enabling automated versioning, changelog generation, and improved code history readability.

## Requirements

For the ZebraSign project, we need tooling that:

1. Validates commit messages against the Conventional Commits specification
2. Works well with our existing pre-commit hooks infrastructure
3. Integrates with our CI/CD pipeline to enforce standards
4. Provides clear guidance to contributors when violations occur
5. Has active maintenance and good documentation

## Research Findings

### 1. commitlint

**Website**: [https://commitlint.js.org/](https://commitlint.js.org/)

**Description**: A popular tool specifically designed to lint commit messages against defined conventions, including Conventional Commits.

**Key Features**:
- Configurable rules
- Shareable configurations
- Detailed error messages
- Well-maintained (active development)
- Extensive documentation
- Integrates with husky for git hook integration

**Implementation Options**:
- Use with husky for pre-commit or commit-msg git hooks
- Can be run in CI environments
- Supports configuration through `.commitlintrc.js` or `commitlint.config.js`

**Example Configuration**:
```js
// commitlint.config.js
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    // Customize or add project-specific rules
    'body-max-line-length': [2, 'always', 100]
  }
};
```

### 2. pre-commit-hooks (Conventional Commits)

**Website**: [https://github.com/alessandrojcm/commitlint-pre-commit-hook](https://github.com/alessandrojcm/commitlint-pre-commit-hook)

**Description**: A pre-commit hook specifically for using commitlint with pre-commit.

**Key Features**:
- Integrates commitlint with pre-commit
- Fits well with existing pre-commit infrastructure
- Simple configuration
- Minimal dependencies

**Implementation Options**:
- Add as a hook in `.pre-commit-config.yaml`

**Example Configuration**:
```yaml
- repo: https://github.com/alessandrojcm/commitlint-pre-commit-hook
  rev: v9.5.0  # Use the latest version
  hooks:
    - id: commitlint
      stages: [commit-msg]
      additional_dependencies: ['@commitlint/config-conventional']
```

### 3. GitHooks-Based Solution (DIY)

**Description**: Custom git hook scripts using the built-in git hooks mechanism.

**Key Features**:
- Maximum flexibility
- No external dependencies
- Can be customized for project-specific requirements
- Simple implementation for basic validation

**Implementation Options**:
- Create a custom `commit-msg` hook script in `.git/hooks/`
- Use pattern matching or regex to validate commit messages

**Example Implementation**:
```bash
#!/bin/bash
# .git/hooks/commit-msg

commit_msg=$(cat "$1")
commit_regex='^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([a-z0-9-]+\))?: .+'

if ! [[ "$commit_msg" =~ $commit_regex ]]; then
  echo "ERROR: Commit message does not follow Conventional Commits format."
  echo "Format: <type>[optional scope]: <description>"
  echo "Example: feat(api): add user profile endpoint"
  exit 1
fi
```

### 4. commitizen

**Website**: [https://commitizen-tools.github.io/commitizen/](https://commitizen-tools.github.io/commitizen/)

**Description**: Provides a command-line interface for creating properly formatted commit messages and can also validate messages.

**Key Features**:
- Interactive commit message creation
- Validation capabilities
- Changelog generation
- Versioning automation
- Cross-platform (Python-based)
- Can be integrated with pre-commit

**Implementation Options**:
- Install as a development dependency
- Configure via `pyproject.toml` or `.cz.toml`
- Set up as a pre-commit hook

**Example Configuration**:
```yaml
# .pre-commit-config.yaml
- repo: https://github.com/commitizen-tools/commitizen
  rev: v3.10.0  # Use the latest version
  hooks:
    - id: commitizen
      stages: [commit-msg]
```

## Analysis and Tradeoffs

### Comparison Matrix

| Tool | Ease of Setup | Integration with pre-commit | Maintenance | Customizability | CI Integration |
|------|---------------|----------------------------|-------------|-----------------|---------------|
| commitlint | Medium | Via plugin or husky | Active | High | Good |
| pre-commit-hooks | High | Native | Active | Medium | Good |
| GitHooks (DIY) | Medium | Manual | Self-maintained | Very High | Custom implementation needed |
| commitizen | Medium | Native | Active | Medium | Good |

### Pros and Cons

**commitlint**:
- ✅ Industry standard
- ✅ Highly configurable
- ✅ Well-documented
- ❌ Requires Node.js

**pre-commit-hooks**:
- ✅ Seamless integration with existing pre-commit setup
- ✅ Simple configuration
- ✅ Consistent with current workflow
- ❌ Less direct control over configuration

**GitHooks (DIY)**:
- ✅ No external dependencies
- ✅ Maximum control
- ❌ Requires maintenance
- ❌ More complex distribution to team

**commitizen**:
- ✅ Interactive UI for creating commits
- ✅ Versioning capabilities
- ✅ Python-based (may align with other tools)
- ❌ May be more than needed if versioning is handled separately

## Recommendation

Based on the research, the **pre-commit-hooks with commitlint** approach is recommended for ZebraSign:

1. **Primary Tool**: Use the commitlint-pre-commit-hook for validation
2. **Configuration**: Extend the `@commitlint/config-conventional` ruleset
3. **Implementation Pattern**:
   - Add to existing `.pre-commit-config.yaml`
   - Configure with project-specific rules as needed
   - Set up in CI pipeline to catch violations at the PR level

### Implementation Plan

1. **Install Dependencies**:
   ```bash
   npm install --save-dev @commitlint/cli @commitlint/config-conventional
   ```

2. **Create Configuration**:
   ```bash
   # commitlint.config.js
   echo "module.exports = {extends: ['@commitlint/config-conventional']};" > commitlint.config.js
   ```

3. **Update Pre-commit Configuration**:
   ```yaml
   # .pre-commit-config.yaml (add this entry)
   - repo: https://github.com/alessandrojcm/commitlint-pre-commit-hook
     rev: v9.5.0
     hooks:
       - id: commitlint
         stages: [commit-msg]
         additional_dependencies: ['@commitlint/config-conventional']
   ```

4. **Add CI Validation**:
   ```yaml
   # Add to .github/workflows/pr_validation.yml
   - name: Check Commit Messages
     run: npx commitlint --from ${{ github.event.pull_request.base.sha }} --to ${{ github.event.pull_request.head.sha }} --verbose
   ```

5. **Documentation**:
   - Add section to CONTRIBUTING.md with details on the Conventional Commits format
   - Include examples of good commit messages
   - Link to the Conventional Commits specification

This approach leverages existing infrastructure, adds minimal dependencies, and ensures consistent enforcement of the Conventional Commits standard throughout the development workflow.

## References

1. [Conventional Commits Specification](https://www.conventionalcommits.org/)
2. [commitlint Documentation](https://commitlint.js.org/)
3. [pre-commit-hooks with commitlint](https://github.com/alessandrojcm/commitlint-pre-commit-hook)
4. [Automated Versioning with Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary)
