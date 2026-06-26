## ADDED Requirements

### Requirement: GPU Write-Disjoint Program Semantics

The system MUST define finite GPU program write-region semantics where admitted output writes have positive byte spans, remain inside the declared output buffer, and pairwise distinct writes in the modeled program domain are disjoint unless explicitly admitted as the same logical output element.

r[domain-primitives.gpu.write-disjoint-program]

#### Scenario: Elementwise writes are disjoint

- GIVEN an admitted elementwise GPU program with one output element per logical index
- WHEN write-region admission is evaluated for two distinct logical indices
- THEN the two byte ranges are admitted only when their output spans do not overlap

#### Scenario: Tiled output writes stay inside buffer

- GIVEN an admitted tiled 2D GPU program and admitted output layout
- WHEN a tile-local output coordinate is projected to a write region
- THEN the region is rejected if its byte span exceeds the output buffer

#### Scenario: Aliasing writes are rejected

- GIVEN two distinct program indices project to overlapping output byte ranges
- WHEN write-disjointness admission is evaluated
- THEN the pair is rejected with no CUDA scheduling or ordering claim
