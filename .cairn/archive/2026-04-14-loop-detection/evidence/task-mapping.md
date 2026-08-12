# Draft → landed mapping

## Original intent

Original change text described natural-loop detection in semantic terms:
back edges by dominance, loop bodies by path-style natural-loop membership,
single-entry/header-dominance/nesting/exit properties, and loop nesting as a
forest.

## Landed shape

The landed module keeps the same domain but splits proof and test roles more
explicitly:

- exact verified helpers over supplied analyses
  - `find_back_edges(edges, dom)`
  - `compute_loop_body(edges, reachable, dom, header, source)`
  - `has_single_entry(edges, reachable, header, body)`
  - `loop_exits(edges, body)`
- randomized tests compare the computed reachable non-self loop bodies against
  the brute-force path oracle derived from CFG paths
- nesting-forest requirement stays at “valid strict-superset parent link”, while
  deterministic immediate-parent selection is implemented and covered by tests

## Why scope wording changed

The review failure showed that the first archive draft overclaimed exact proof of
semantic path-style loop bodies and convenience-style single-entry checking.
Those claims were narrowed to match what is actually proved:
exact helper contracts over reachability/dominator views, plus explicit test
coverage for the stronger path-oracle correspondence.
