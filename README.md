
# Git Diff Action

This GitHub Action performs a git diff and outputs the files that changed in a Pull Request, based on specified patterns.

## Inputs

### `patterns`

**Required**: A string containing file patterns to include or exclude in the diff. Patterns should be separated by commas.

- Use `*` as a wildcard for any number of characters.
- Use `**` for recursive directory matching.
- Prefix patterns with `!` to exclude files.

Examples:
- `*.rs`: All Rust files
- `src/**/*`: All files within the `src` directory and its subdirectories
- `!README.md`: Exclude the README.md file

Patterns could be a single line or multiline string.
```yaml
patterns: |
  *.rs
  src/**/*
  !README.md
```
```yaml
patterns: '*.rs,src/**/*,!README.md'
```

## Outputs

### `DIFF_FILES`

A list of files that changed in the PR, matching the specified patterns.

### `DIFF_COUNT`

The number of files that changed in the PR, matching the specified patterns.

## Example usage

```yaml
name: Git Diff Check

on:
  pull_request:
    branches: [ main ]

jobs:
  check_diff:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Get changed files
      id: diff
      uses: LuisLiraC/git-diff@master
      with:
        patterns: '*.rs,src/**/*,!README.md'

    - name: Get diff outputs
      run: |
        echo "Files: ${{ steps.diff.outputs.DIFF_FILES }}"
        echo "Count: ${{ steps.diff.outputs.DIFF_COUNT }}"
```

> [!WARNING]
> At this moment it only works with the `pull_request` event. It uses the `GITHUB_BASE_REF` environment variable to get the base branch of the PR and this it is only available in the `pull_request` event.
