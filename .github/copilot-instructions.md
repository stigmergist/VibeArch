# Copilot Workspace Instructions: LLM-Wiki Architecture Loop

Purpose: Keep a living architecture knowledge base in arch based on the current codebase, and use that knowledge base to guide implementation quality.

## Global operating model

Treat this repo as having two zones:
- Code zone: workspace root, excluding arch
- Knowledge zone: arch directory

Primary behavior:
1. Start from user and business outcomes, then observe the code zone.
2. Update and maintain architecture knowledge in the knowledge zone.
3. Use the knowledge zone as architecture guardrails when generating code.
4. Critique the architecture for weak spots and propose improvements.
5. Assess non-functional architecture qualities and production deployability continuously.
6. Explain architecture implications in plain business/customer language first, then support with technical evidence.

## Response Output Contract (Two-Layer)

Default to a two-layer response so non-technical and technical readers can both use the output:
- Layer 1 (always first): Plain-language customer/business impact and recommended action.
- Layer 2 (as needed): Technical evidence, implementation notes, and file-level references.

Rules:
- Do not lead with technical terms when a plain-language alternative exists.
- Define acronyms on first use (for example: CI/CD, continuous integration and continuous delivery).
- Keep sentences short and concrete; avoid consultant-style abstract phrasing.
- If the user asks for executive/plain-English output, keep Layer 2 brief unless explicitly requested.
- If the user asks for engineering depth, include full Layer 2 detail after Layer 1.

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
- arch/components.md: high-level component index with concise descriptions, high-level relationships between components, and links to each component detail file.
- arch/next-steps.md: index of recommended next steps and actions, grouped by priority and linked to the relevant component detail files.
- arch/component-details/<component-slug>.md: one file per identified component, containing component-specific responsibilities, dependencies, risks, open questions, recommended actions, and navigation links to related components/docs.
- arch/data-flow.md: key data flows, interfaces, and integration boundaries.
- arch/decisions.md: lightweight ADR log (decision, status, date, rationale, consequences).
- arch/risks.md: architecture risks, severity, likelihood, mitigation, and owner.
- arch/drift.md: mismatches between intended and observed architecture.
- arch/change-log.md: dated summary of wiki updates.

If a file already exists, update in place and preserve human-authored sections whenever possible.
If a component is identified in `arch/components.md`, it should also have a corresponding file in `arch/component-details/` unless there is a documented reason not to.

## Update triggers

Trigger architecture refresh when:
- Asked to update, sync, generate, or review architecture.
- Core code changes are made in root that can affect customer outcomes or operational safety (new modules, changed boundaries, new dependencies, data model/API changes).
- Significant implementation tasks are completed.
- Deployment assumptions or runtime topology change (hosting target, scaling model, observability, CI/CD, infra).

For normal implementation requests, do a lightweight architecture conformance check before producing code.

## Architecture Refresh Gate (Change Volume)

Trigger an architecture refresh when code change volume crosses the threshold below, even if the user did not explicitly ask for architecture work.

Refresh gate signals (outside `arch/`):
- 5 or more files changed, or
- 200 or more net changed lines, or
- Any change to runtime boundaries, API contracts, persistence model, auth/session logic, deployment topology, or build/release pipeline.

Use the gate to reassess customer impact and delivery risk, not just code churn.

When the gate is triggered:
1. Re-scan code-zone changes since the last architecture refresh recorded in `arch/change-log.md`.
2. Re-validate and update `arch/next-steps.md`, `arch/risks.md`, and `arch/drift.md` based on current evidence.
3. Append an entry to `arch/change-log.md` with:
   - Trigger signal(s) and change volume summary
   - Scope reviewed (modules/files)
   - What changed vs what remained stable
   - Re-ranked top 1-3 architecture actions

If the gate is not triggered, continue with lightweight architecture conformance checks only.

## Required workflow

For architecture sync tasks:
1. Identify customer/business outcomes first, then scan code outside arch to detect structure, boundaries, dependencies, and runtime patterns that affect those outcomes.
2. Update the architecture wiki files in arch.
   - Update `arch/next-steps.md` so recommended actions live in a dedicated index rather than being embedded only inside summary docs.
   - Ensure each identified component has a matching `arch/component-details/<component-slug>.md` file.
   - Ensure `arch/components.md` stays high-level and focuses on concise component summaries plus relationship descriptions between components.
   - Ensure `arch/components.md` links to each component detail file, and each component detail file links back to `arch/components.md`, `arch/next-steps.md`, and relevant related components/docs.
3. Record key changes in arch/change-log.md with date and short rationale.
4. Record any uncertainty or inferred assumptions explicitly.
5. Re-assess non-functional qualities and deployability implications; update arch/risks.md and arch/drift.md when gaps are found.
6. Re-validate any embedded "what to do next" / "next steps" suggestions across arch docs against current code and assumptions; remove stale items, adjust priorities, and add evidence notes for changed recommendations.
   - Move actionable recommendations out of incidental prose where practical and into `arch/next-steps.md` plus the relevant `arch/component-details/<component-slug>.md` files.
   - Re-rank global actions in `arch/next-steps.md` and component-local actions in each component detail file.
   - Check that component relationships are cross-linked where useful for navigation.
7. Apply mandatory visibility cues for scanability in every architecture update/refresh/sync:
   - Ensure `arch/README.md`, `arch/system-overview.md`, `arch/risks.md`, and `arch/drift.md` each contain a `## Scan First (Traffic Light)` section.
   - In each `Scan First` section, include exactly 3 bullets in this order: 🔴 Act now, 🟡 Watch closely, 🟢 Stable base.
   - Ensure `arch/next-steps.md` contains a `## Priority Legend` and traffic-light section headers for priority groups (🔴, 🟡, 🟢).
   - Keep traffic-light cues concise and action-oriented so a reader can identify top attention areas in under 10 seconds.

For code generation tasks:
1. Confirm the user-facing outcome and value first.
2. Read arch/README.md and relevant architecture docs.
3. Generate code that aligns with documented boundaries and constraints.
4. If introducing a new component or boundary, update architecture docs in the same change.

For architecture weakness analysis tasks:
1. Read architecture docs in arch.
2. Compare against observed code (excluding arch).
3. Report weaknesses as a prioritized list with:
   - Business/customer impact
   - Recommended action and urgency
   - Weak area
   - Why it matters
   - Evidence
   - Suggested remediation
   - Suggested owner and urgency
4. Add or update entries in arch/risks.md and arch/drift.md when requested or when making architecture updates.
5. Include explicit assessment for these non-functional qualities:
   - Availability
   - Performance
   - Scalability
   - Security
   - Manageability
   - Flexibility
   - Portability
   - Cost
   - Resilience
   - Robustness
   - Modularity
   - Reliability
   - Fault Tolerance
   - Observability
   - Testability  
   - Maintainability
   - Privacy and Data Protection
   - Usability
   - Accessibility
6. Include deployability assessment:
   - Whether customers can rely on the current deployment path today.
   - Where the system can be deployed now (for example local-only, containerized, cloud VM, managed platform).
   - What is missing for production deployment (configuration, secrets handling, observability, CI/CD, rollback, capacity planning).
   - Recommended target deployment model and smallest path to production readiness.

## Business Value Framing Rules

For architecture syncs, reviews, and recommendations:
- Lead with business and customer value language before technical detail.
- For each major risk or recommendation, state customer impact explicitly (trust, reliability perception, onboarding friction, retention risk, support burden, revenue/cost exposure, time-to-market).
- For each recommendation, include a value hypothesis: what improves for customers or the business if this is done.
- Keep technical evidence as proof, not as the opening frame.

Preferred response order when presenting architecture findings:
1. Customer/business consequence
2. Recommended action and urgency
3. Technical evidence and implementation notes

Accessibility default:
- Use plain words first, then technical terms.
- For each unavoidable technical term, add a short plain-English explanation.
- Keep business consequence visible in the opening lines.

## Architecture Sync Validation Checklist

Before completing any architecture sync, verify:
- [ ] All files in arch/ have been reviewed for embedded recommendations, TODOs, and next-steps language
- [ ] Completed work from prior syncs has been moved to "Completed" sections with dates
- [ ] Prioritized work lists have stale items removed and remaining items re-ranked
- [ ] `arch/next-steps.md` has been updated and does not duplicate stale completed work
- [ ] Every identified component in `arch/components.md` has a corresponding file in `arch/component-details/`, or an explicit note explains why not
- [ ] `arch/components.md` remains a high-level relationship/index page rather than absorbing per-component deep detail
- [ ] Cross-links between `arch/components.md`, `arch/next-steps.md`, and `arch/component-details/*.md` are present and still valid
- [ ] Evidence in risk/drift tables reflects current code state; update status indicators (🔴/🟡/🟢) if code changes warrant it
- [ ] Deployability assessment in system-overview.md has been re-evaluated against current infrastructure/CI/CD state
- [ ] All ADRs are still valid; mark as "superseded" if new decisions override old ones
- [ ] NFR scorecard evidence lines reference actual current code paths or runtime behavior
- [ ] Change log includes entry for this sync pass with specific file updates and rationale
- [ ] Change-volume gate has been evaluated; if triggered, `arch/change-log.md` records the trigger signal, review scope, and reprioritized actions
- [ ] `arch/README.md`, `arch/system-overview.md`, `arch/risks.md`, and `arch/drift.md` include `## Scan First (Traffic Light)` sections with 🔴/🟡/🟢 bullets in the required order
- [ ] `arch/next-steps.md` includes `## Priority Legend` and traffic-light priority section headers

## Non-Functional Architecture Requirements

For each architecture sync or review, include a concise NFR scorecard in the relevant docs (typically arch/system-overview.md, arch/risks.md, and arch/drift.md):
- Status per quality: good / watch / weak
- Evidence from current code and runtime assumptions
- Top remediation action per weak/watch quality

Present NFRs in business-first buckets, then map to technical qualities:
- Customer trust and continuity: availability, reliability, fault tolerance, security, privacy and data protection.
- Experience and growth: performance, scalability, usability, accessibility.
- Delivery and operations: observability, maintainability, testability, manageability, robustness.
- Strategic efficiency: cost, portability, flexibility, modularity.

When presenting NFR scorecards or quality summaries:
- Use traffic-light icons for readability when the format supports it:
   - good = 🟢
   - watch = 🟡
   - weak = 🔴
- Keep the icon and the text label together so meaning is preserved in plain markdown.

When a visual explanation would materially improve understanding:
- Add concise markdown diagrams, preferably Mermaid, to architecture docs.
- Prefer diagrams for runtime topology, component boundaries, and important data flows.
- Keep diagrams evidence-based and simple; do not invent infrastructure or components not supported by the code.

The assessment must be evidence-based from repository code and current documented assumptions, not aspirational design.

When presenting NFR status, translate technical quality into customer/business outcomes where possible. Examples:
- Availability and reliability -> user trust and retention risk
- Performance and scalability -> response experience and conversion risk
- Security and privacy -> compliance and brand risk
- Observability and maintainability -> incident recovery time and delivery speed
- Cost and portability -> unit economics and strategic flexibility

## Quality bar

Architecture wiki content should be:
- Impact-grounded: starts with customer/business outcomes, backed by actual code and current repository state.
- Actionable: concrete guidance, not generic statements.
- Traceable: include references to concrete modules or paths after stating the impact.
- Incremental: prefer small, frequent updates over large rewrites.
- Non-functional aware: includes explicit quality trade-offs and risk posture.
- Deployable: clarifies current production readiness and concrete gaps to deploy safely.
- Current: embedded recommendation lists (for example prioritized next work) are checked for validity and updated when assumptions change.
- Navigable: component names, architecture files, and related references should be linked where reasonable to reduce dead-end documentation.
- Value-forward: recommendations clearly describe customer/business impact first, with technical detail as supporting evidence.

When unsure:
- Mark assumptions clearly.
- Ask for clarification only when ambiguity blocks safe progress.
- Avoid inventing architecture not supported by code or explicit decisions.


## LLM-wiki style constraints

- Keep architecture docs concise, navigable, and up to date.
- Prefer stable, low-entropy summaries over verbose prose.
- Use visual structure intentionally: tables for scorecards, traffic-light icons for status, and diagrams where they improve comprehension.
- Prefer markdown links for architecture file names, component names, and related docs when the link materially improves navigation.
- `arch/components.md` should stay concise and high-level: list components, summarize their relationships, and link to the per-component detail files.
- `arch/next-steps.md` should serve as the top-level action index; `arch/component-details/*.md` should hold per-component detail and local actions rather than repeating the full global list.
- Treat arch as the architecture source of truth for implementation guidance.
- Treat code outside arch as the source of truth for what currently exists.
- Continuously reconcile differences through documented decisions, risks, and drift notes.
