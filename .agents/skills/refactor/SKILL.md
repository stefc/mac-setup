---
name: refactor
description: Skill for refactoring the mac-setup Rust application
---
This skill helps refactor the `mac-setup` Rust application. Follow these steps to maintain code quality and consistency.

### General Principles
- **Modularity**: Keep different parts of the system decoupled.
- **Clarity**: Write code that is easy to understand.
- **Testability**: Ensure that components can be tested in isolation.

### Refactoring Steps
1.  **Identify the Target**: Analyze the user's request to pinpoint the code that needs refactoring.
2.  **Examine the Code**: Read the relevant files (`.rs` files) to understand the current implementation.
3.  **Extract Generic Logic**:
    - If you find logic that can be reused, extract it into a new function.
    - Place generic, reusable functions in the `src/common/utils.rs` module.
4.  **Update Module Exports**:
    - Make the new function public (`pub fn`).
    - Export it from the common module by adding a `pub use` statement in `src/common/mod.rs`.
5.  **Add Unit Tests**:
    - For any new function added to `src/common/utils.rs`, create a corresponding unit test.
    - Place the tests inside a `#[cfg(test)]` block at the end of the file.
6.  **Update the Original Code**: Modify the original file to use the new, refactored function.
7.  **Verify Changes**: After refactoring, ensure the application still builds and tests pass. Although you cannot run `cargo` commands yourself, remind the user to do so.
