# Copilot Workspace Instructions: LLM-Wiki Architecture Loop

Purpose: Keep a living architecture knowledge base in arch based on the current codebase, and use that knowledge base to guide implementation quality.

## Global operating model

Treat this repo as having two zones:
- Code zone: workspace root, excluding arch
- Knowledge zone: arch directory

Primary behavior:
1. Observe the code zone.
2. Update and maintain architecture knowledge in the knowledge zone.
3. Use the knowledge zone as architecture guardrails when generating code.
4. Critique the architecture for weak spots and propose improvements.

## Scope rules

When building architecture knowledge from code:
- Include: all files in workspace root and subdirectories.
- Exclude: arch/**.
- Exclude generated or dependency folders when present (for example: node_modules, dist, build, .next, .venv, __pycache__).

When enforcing architecture during implementation:
- Read and follow docs in arch/** first.
- If architecture and requested change conflict, call out the conflict and propose the smallest architecture-compliant path.
- If no compliant path exists, propose an Architecture Decision Record update before implementing a violating design.

## Architecture wiki ownership in arch

Copilot should maintain these files in arch when missing, and keep them updated over time:
- arch/README.md: wiki index and navigation.
- arch/system-overview.md: goals, boundaries, context diagram narrative, and major runtime concerns.
- arch/components.md: component catalog with responsibilities, dependencies, and ownership.
- arch/data-flow.md: key data flows, interfaces, and integration boundaries.
- arch/decisions.md: lightweight ADR log (decision, status, date, rationale, consequences).
- arch/risks.md: architecture risks, severity, likelihood, mitigation, and owner.
- arch/drift.md: mismatches between intended and observed architecture.
- arch/change-log.md: dated summary of wiki updates.

If a file already exists, update in place and preserve human-authored sections whenever possible.

## Update triggers

Trigger architecture refresh when:
- Asked to update, sync, generate, or review architecture.
- Core code changes are made in root (new modules, changed boundaries, new dependencies, data model/API changes).
- Significant implementation tasks are completed.

For normal implementation requests, do a lightweight architecture conformance check before producing code.

## Required workflow

For architecture sync tasks:
1. Scan code outside arch to detect structure, boundaries, dependencies, and runtime patterns.
2. Update the architecture wiki files in arch.
3. Record key changes in arch/change-log.md with date and short rationale.
4. Record any uncertainty or inferred assumptions explicitly.

For code generation tasks:
1. Read arch/README.md and relevant architecture docs first.
2. Generate code that aligns with documented boundaries and constraints.
3. If introducing a new component or boundary, update architecture docs in the same change.

For architecture weakness analysis tasks:
1. Read architecture docs in arch.
2. Compare against observed code (excluding arch).
3. Report weaknesses as a prioritized list with:
   - Weak area
   - Why it matters
   - Evidence
   - Suggested remediation
   - Suggested owner and urgency
4. Add or update entries in arch/risks.md and arch/drift.md when requested or when making architecture updates.

## Quality bar

Architecture wiki content should be:
- Evidence-based: grounded in actual code and current repository state.
- Actionable: concrete guidance, not generic statements.
- Traceable: include references to concrete modules or paths.
- Incremental: prefer small, frequent updates over large rewrites.

When unsure:
- Mark assumptions clearly.
- Ask for clarification only when ambiguity blocks safe progress.
- Avoid inventing architecture not supported by code or explicit decisions.

## LLM-wiki style constraints

- Keep architecture docs concise, navigable, and up to date.
- Prefer stable, low-entropy summaries over verbose prose.
- Treat arch as the architecture source of truth for implementation guidance.
- Treat code outside arch as the source of truth for what currently exists.
- Continuously reconcile differences through documented decisions, risks, and drift notes.
