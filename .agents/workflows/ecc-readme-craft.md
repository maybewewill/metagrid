---
description: Generate or improve a GitHub README using readme-craft's 3-tier layout strategy, SVG wordmark generator, and 45-point quality audit.
argument-hint: "[--fresh | path/to/project | blank for current project]"
---

# readme-craft Workflow

Execute the `readme-craft` skill on the target repository.

## Execution Steps

1. **Scan Project**: Scan the repository structure, `package.json`/build files, source code, and existing `README.md`.
2. **Determine Mode**:
   - **Mode A (New Project / Fresh)**: Create from scratch using the 3-Tier layout (Above fold pitch, Scan zone, Deep content).
   - **Mode B (Codebase Scan)**: Extract features, commands, installation, and environment variables from code.
   - **Mode C (Improve Existing)**: Run the 45-point quality checklist and apply targeted layout fixes.
3. **Generate Assets**: Generate light/dark mode SVG wordmarks in `.github/` if needed.
4. **Format & Verify**: Apply GitHub-native formatting (badges, tables, alerts, picture tags) and run the 45-point quality audit.
