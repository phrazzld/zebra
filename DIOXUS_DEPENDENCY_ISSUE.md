# Dioxus Dependency Issue

During implementation of the T011 task (Implementing line parsing in Rust/Wasm), it was noted that the `zebra_desktop` app has issues when running clippy. The error output indicates there are multiple versions of `dioxus_core` in the dependency graph, causing trait implementation conflicts:

```
error[E0277]: the trait bound `fn(IconProps<_>) -> Option<VNode> {Icon::<_>}: ComponentFunction<_, _>` is not satisfied
```

This appears to be related to the dependency changes made in T001 where `dioxus-free-icons` was pinned to a specific commit from the `SimonBaars` fork. The desktop app now has conflicting versions of dioxus core dependencies.

## Potential Solutions

1. Update all dioxus-related dependencies to compatible versions
2. Pin all dioxus dependencies to specific compatible versions
3. Consider vendoring the icons needed to avoid the external dependency entirely

## Recommended Path Forward

Create a new task to address this dependency conflict in the desktop application. The current task (T011) was completed successfully, with all tests passing for the wasm implementation. The errors in the desktop app are unrelated to our changes but should be fixed in a subsequent task.