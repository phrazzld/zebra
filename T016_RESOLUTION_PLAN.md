# T016 Resolution Plan: Fix dioxus dependency conflicts

## Problem Overview
The desktop application has dependency conflicts between multiple versions of `dioxus_core`:

1. `dioxus` and `dioxus-desktop` are pinned to commit `2e65e7a91352e29f966c0f74be0f3e6bde88edc4`
2. `dioxus-free-icons` brings in a different version of the dioxus dependencies

This conflict causes clippy errors because multiple trait implementations are colliding:
```
error[E0277]: the trait bound `fn(IconProps<_>) -> Option<VNode> {Icon::<_>}: ComponentFunction<_, _>` is not satisfied
```

## Proposed Solutions

### Option 1: Use patch in Cargo.toml (Recommended)
Add a patch section to the root Cargo.toml to ensure all dioxus dependencies use the same version:

```toml
[patch."https://github.com/DioxusLabs/dioxus"]
dioxus = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
dioxus-core = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
dioxus-core-macro = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
dioxus-hooks = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
dioxus-html = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
dioxus-html-internal-macro = { git = "https://github.com/DioxusLabs/dioxus.git", rev = "2e65e7a91352e29f966c0f74be0f3e6bde88edc4" }
```

### Option 2: Vendor the icons
Create a very simple implementation of the needed icons to avoid the dependency conflict:

```rust
// Create a simplified icon implementation in the desktop app
pub mod icons {
    pub mod go_icons {
        use dioxus::prelude::*;

        #[component]
        pub fn GoCheck() -> Element {
            rsx!{ "✓" }
        }

        // Define other icons...
    }
}

#[component]
pub fn Icon(
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    width: Option<u32>,
    height: Option<u32>,
    fill: Option<String>,
    icon: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: class,
            onclick: onclick,
            style: {
                let mut style = String::new();
                if let Some(fill) = &fill {
                    style.push_str(&format!("color: {};", fill));
                }
                if let Some(width) = width {
                    style.push_str(&format!("width: {}px;", width));
                }
                if let Some(height) = height {
                    style.push_str(&format!("height: {}px;", height));
                }
                style
            },
            icon.unwrap_or(rsx!{ "🔣" })
        }
    }
}
```

### Option 3: Modify dioxus-free-icons dependency
Update the dioxus-free-icons dependency in zebra_desktop/Cargo.toml to suppress default features and explicitly specify which dioxus version to use:

```toml
dioxus-free-icons = {
    git = "https://github.com/SimonBaars/dioxus-free-icons.git",
    rev = "111acbfea05ef66f3c12f3da6d82913c87aa6586",
    features = ["octicons"],
    default-features = false
}
```

## Implementation Steps

1. Create a branch for T016
2. Implement Option 1 (patch approach)
3. Run clippy to verify the conflicts are resolved
4. Test the desktop app functionality
5. Create a PR with the fix

## Verification

1. Run `cargo clippy --all-targets --all-features --workspace` to verify no errors
2. Run `cargo run -p ZebraSign` to ensure the desktop app works correctly
3. Check icons display properly

## Impact

Fixing this dependency conflict will:
1. Allow pre-commit hooks to run successfully
2. Fix clippy errors in the codebase
3. Make future development smoother by eliminating trait implementation conflicts
4. Enable other PRs (like T011) to be committed without --no-verify
