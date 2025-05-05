# T005 · Chore · P0: configure rustfmt settings

## Analysis
This task involves setting up the rustfmt configuration for the project. The requirements are to:
1. Create or update `.rustfmt.toml` in the repository root
2. Configure the Rust edition and max line width
3. Add any other useful formatting settings

## Implementation Plan
1. Check if a `.rustfmt.toml` file already exists
2. Create/update the file with the required settings:
   - Set `edition = "2021"` as specified
   - Set `max_width = 100` as specified
   - Research and add other useful standard rustfmt settings that improve code readability
3. Verify the configuration works by running `cargo fmt --all --check` on a sample
4. Document the settings and their purpose in the file
5. Update the task in TODO.md

## Specific Settings to Include
- `edition = "2021"` - Specifies the Rust edition to use for formatting
- `max_width = 100` - Sets the maximum line width before wrapping
- Other settings that might be useful:
  - `tab_spaces` - Number of spaces per indentation level
  - `newline_style` - Line ending style
  - `use_small_heuristics` - Controls specifics of formatting behavior

## Expected Output
A `.rustfmt.toml` file in the repository root with the required settings that passes verification when running `cargo fmt --all --check`.