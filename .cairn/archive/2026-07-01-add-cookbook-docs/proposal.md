## Why

Trellis has strong source-level patterns, but a user or contributor currently has to infer them from many modules. Cookbook recipes would shorten the path from “I have a property to model” to a checked spec/exec/proof contribution.

## What Changes

- Add cookbook documentation for common workflows.
- Include recipes for adding a primitive, writing positive and negative tests, running focused Verus checks, and using existing modules in downstream code.
- Keep commands current with repository validation scripts.

## Scope

In scope: cookbook docs, command snippets, source links, and doc validation for referenced files/commands where practical.

Out of scope:
- New primitives.
- Full tutorial website generation.
- Replacing `docs/requirements.md` as the canonical Tracey source.

## Verification

Run documentation validation, README/catalog checks, and focused command checks for cookbook snippets that are intended to be executable.
