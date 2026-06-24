# QA and Refactoring Workflow

Analyze the current repository for code smells using the `qa` skill. Please follow these steps sequentially:

1. **Analysis:** Conduct a comprehensive quality assurance review of the codebase to identify architectural, maintainability, security, and idiomatic code issues.
2. **Reporting:** Generate a report of the identified code smells, sorted by priority (with the most critical issues first).
3. **Refactoring:** For the top 3 highest-priority code smells:
   - Create a separate git worktree outside of the main repository directory to isolate your changes.
   - Inside each worktree, create and checkout a new feature branch.
   - Implement the necessary refactorings to fix the code smell.
   - Commit the changes and open a Pull Request (PR) using the GitHub CLI for each branch.
