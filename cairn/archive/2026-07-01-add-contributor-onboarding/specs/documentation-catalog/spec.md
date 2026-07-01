## ADDED Requirements

### Requirement: Contributor Onboarding Guide

The repository MUST provide public contributor onboarding documentation that explains how to add or modify verified logic while preserving requirements coverage, proof obligations, positive and negative tests, and repository validation gates.

r[documentation-catalog.contributor-onboarding]

#### Scenario: New module workflow is documented

- GIVEN a contributor wants to add a verified module
- WHEN they read the contributor guide
- THEN the guide identifies the required requirement ID, spec function, exec function with `ensures`, proof or property evidence, and positive and negative tests

#### Scenario: Core change validation is explicit

- GIVEN a contributor changes core logic or verified library code
- WHEN they read the contributor guide
- THEN the guide tells them to run relevant baseline checks before the change and final checks after the change

#### Scenario: Untracked proof gap is rejected

- GIVEN a contributor introduces a trusted boundary or weakened proof marker without updating the proof-gap inventory
- WHEN repository validation runs
- THEN validation fails or the guide directs the contributor to the required inventory/spec update before completion
