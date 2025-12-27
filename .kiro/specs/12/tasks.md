# Implementation Plan: Documentation Reorganization

## Overview

This plan reorganizes the DX project's `/docs` folder from a flat structure with 80+ files into a clean, professional hierarchy. Tasks are ordered to minimize broken links and ensure no content is lost.

## Tasks

- [x] 1. Create new folder structure
  - Create all target directories
  - Create placeholder README.md files in each folder
  - _Requirements: 3.1_

- [x] 2. Archive historical and planning documents
  - [x] 2.1 Move progress reports to archive
    - Move all files from `docs/progress/` to `docs/archive/progress/`
    - Move dated reports (STATUS_REPORT.md, IMPLEMENTATION_PROGRESS.md, etc.)
    - _Requirements: 2.1, 2.2, 2.4_
  - [x] 2.2 Move planning documents to archive
    - Move NEXT_STEPS.md, THOUGHTS.md, roadmap files
    - Move WORKSPACE_PLANNING.md, GENERATOR_PLANNING.md
    - _Requirements: 2.1, 2.4_
  - [x] 2.3 Move legacy/outdated docs to archive
    - Move playground-archive contents
    - Move completion reports and victory banners
    - _Requirements: 2.2, 2.4_

- [x] 3. Consolidate serializer documentation
  - [x] 3.1 Create serializer folder and index
    - Create `docs/api/serializer/` folder
    - Create README.md with overview of all formats
    - _Requirements: 4.1, 4.2_
  - [x] 3.2 Move and rename serializer format docs
    - Move HUMAN.md → api/serializer/human-format.md
    - Move LLM.md → api/serializer/llm-format.md
    - Move MACHINE.md → api/serializer/machine-format.md
    - _Requirements: 4.1, 4.3_
  - [x] 3.3 Merge duplicate serializer docs
    - Merge dx-serializer.md and DX_SERIALIZER.md content
    - Delete redundant files after merge
    - _Requirements: 1.1, 1.2, 1.3_

- [x] 4. Organize architecture documentation
  - [x] 4.1 Move architecture files
    - Move and rename files from architecture/ folder
    - Move BIDIRECTIONAL_SYSTEM.md to architecture/
    - _Requirements: 3.1, 5.2_
  - [x] 4.2 Create architecture index
    - Create README.md linking all architecture docs
    - _Requirements: 3.3_

- [x] 5. Organize API documentation
  - [x] 5.1 Move API reference files
    - Move STACK.md → api/stack.md
    - Move CLI.md → api/cli.md
    - Move DX_WWW.md → api/dx-www.md
    - _Requirements: 3.1, 5.2_
  - [x] 5.2 Create API index
    - Create README.md linking all API docs
    - _Requirements: 3.3_

- [x] 6. Organize guides
  - [x] 6.1 Move guide files
    - Move files from guides/ folder
    - Create migration/ subfolder for migration guides
    - _Requirements: 3.1, 5.2_
  - [x] 6.2 Create guides index
    - Create README.md linking all guides
    - _Requirements: 3.3_

- [x] 7. Organize reference documentation
  - [x] 7.1 Consolidate benchmarks
    - Move all benchmark files to reference/benchmarks/
    - Keep only most recent/relevant results
    - _Requirements: 6.1, 6.2_
  - [x] 7.2 Organize comparisons
    - Create reference/comparisons/ folder
    - Move framework comparison docs
    - _Requirements: 3.1_
  - [x] 7.3 Merge coding standards
    - Merge CODE_STANDARD.md and CODING_STANDARD.md
    - Keep as reference/coding-standards.md
    - _Requirements: 1.1, 1.2, 1.3_

- [x] 8. Clean up remaining root files
  - [x] 8.1 Categorize and move remaining files
    - Review each remaining root file
    - Move to appropriate category or archive
    - _Requirements: 5.1, 5.2_
  - [x] 8.2 Delete confirmed duplicates
    - Remove files that have been merged
    - Verify no unique content is lost
    - _Requirements: 1.1_

- [x] 9. Create main documentation index
  - [x] 9.1 Rewrite root README.md
    - Create clear navigation to all sections
    - Include brief descriptions of each section
    - _Requirements: 5.3, 8.1_
  - [x] 9.2 Verify all links work
    - Check all internal links in README.md
    - Fix any broken references
    - _Requirements: 8.1, 8.2_

- [x] 10. Final verification
  - Verify no orphaned files exist
  - Verify all essential docs are accessible
  - Verify archive is complete
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

## Notes

- Tasks should be executed in order to minimize broken links
- Always verify content before deleting any file
- Archive files should maintain original names for traceability
- Each folder's README.md serves as the index for that section
