# NC-TEAM-001 — GitHub PR Task Planner Gate

**This is a fork of PandaCode** — https://github.com/aicodeking/pandacode.

## Intent

Every user request gets a fresh Task Checklist Planner and Team Lead quality
gate. The full rule is in [SOP NC-TEAM-001](SOP-DEVELOPMENT-RELEASE.md).

## Enforced at PR-time

- The workflow checks new PRs for a newly added Markdown planner at the
  required task path. Merely modifying a previous task does not count.
- It checks for real request context, Team Lead field, worker assignment,
  acceptance criteria, checkboxes and GO/NO-GO review status.
- PRs #1–#3 predate NC-TEAM-001 and are explicitly grandfathered from the
  new-plan formatting check only; they still need their original test
  evidence and an authorized review before merge.
- The checker uses child-process argument arrays and validates Git SHAs.
  It does not evaluate user-controlled shell code.
- GitHub Actions is read-only, checks out the trusted **base SHA** and
  fetches the PR head strictly as data for review, without executing its
  scripts, npm packages or external MCP tools.

## Important limits and activation

A Markdown validator is not proof that actual Agents were delegated or
reviewed. Team Lead must review real diffs and record a responsible
GO/NO-GO. Independent reviewers and native OS tests are separately required
at their respective release gates.

Because pull_request_target loads the workflow from the **target base
branch**, this check activates after merging its trusted source and YAML
into the target branch. Set the NC-TEAM-001 check as **Required** under the
repository's branch protection rules after the workflow is verified.
An absent check must not be reported as a pass.

No desktop apps, installers, privileged MCP servers, credentials or
production releases are involved in this change.
