# Archive Closeout

## Archive move
- Source: `openspec/changes/store-buffer-forwarding`
- Destination: `openspec/changes/archive/2026-04-23-store-buffer-forwarding/store-buffer-forwarding`
- Result: moved successfully on 2026-04-23

## Post-archive active change listing
Captured with:

```sh
find openspec/changes -maxdepth 1 -mindepth 1 -type d | sort
```

Result:
- `openspec/changes/archive`
- `openspec/changes/cache-prefetch-validity`
- `openspec/changes/debruijn-substitution`
- `openspec/changes/directory-coherence`
- `openspec/changes/inclusive-exclusive-hierarchy`
- `openspec/changes/register-allocation`
- `openspec/changes/stack-frame-layout`
- `openspec/changes/tlb-shootdown`
- `openspec/changes/wasm-primitives`

Active non-archive changes remaining: 8
