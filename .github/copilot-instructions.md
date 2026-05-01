# Copilot Workspace Instructions

Purpose: Keep these instructions always-on and lightweight. Put detailed, optional workflows in skills so they load only when relevant.

## Always-on behavior

1. Start from user and business outcomes before technical implementation details.
2. Keep responses concrete and concise, with plain language first.
3. Preserve architecture boundaries documented in .arch when implementing code.
4. If code change likely affects architecture boundaries, contracts, persistence/auth/session logic, deployment topology, or build/release pipeline, run an architecture conformance check.
5. If no architecture-compliant path exists, call out the conflict and propose the smallest compliant option plus an ADR update path.

## Use the architecture skill when relevant

Load and follow .github/skills/llm-wiki-architecture/SKILL.md when the user asks to:
- sync, update, generate, or maintain architecture docs
- review, critique, or assess architecture quality
- perform risk, drift, NFR, threat-model, or deployability analysis
- validate architecture after major code changes

## Architecture source of truth

1. Treat .arch as the source of truth for intended architecture.
2. Treat code outside .arch as the source of truth for current implementation.
3. Reconcile mismatches through explicit risk, drift, and decision updates.

## Style defaults

1. Lead with customer/business impact, then technical evidence.
2. Define acronyms on first use.
3. Avoid inventing architecture not supported by repository evidence.
