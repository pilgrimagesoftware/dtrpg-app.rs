## ADDED Requirements

### Requirement: Rust frontend baseline implementation MUST satisfy shared desktop library layout behavior
The Rust frontend MUST implement the shared desktop library baseline layout and interaction behavior defined by the app meta-repository.

#### Scenario: Rendering baseline library layout in Rust
- **WHEN** the Rust app renders the library screen in baseline mode
- **THEN** it presents the shared top-level layout regions and interactions defined by shared app specs

### Requirement: Rust baseline implementation MUST use stubbed backend adapters
The Rust frontend baseline implementation MUST keep backend communication stubbed while exercising list/detail/filter/refresh flows.

#### Scenario: Loading library data in Rust baseline mode
- **WHEN** the Rust frontend loads or refreshes library content in baseline phase
- **THEN** it uses stubbed adapters and no live backend SDK calls
