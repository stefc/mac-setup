---
name: refactor
description: Refactoring workflow for Rust application.
---

### Refactoring Workflow
1. **Analyze**: Pinpoint target code and examine relevant `.rs` files.
2. **Extract**: Move reusable generic logic into `src/common/utils.rs` as `pub fn`.
3. **Export**: Expose new functions via `pub use` in `src/common/mod.rs`.
4. **Test**: Add unit tests in a `#[cfg(test)]` block at the end of the concrete file.
5. **Update**: Modify original code to use the refactored functions.
6. **Verify**: Remind the user to run `cargo check`, `cargo fmt` and `cargo test`.
