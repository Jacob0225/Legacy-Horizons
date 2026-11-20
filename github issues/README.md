# GitHub issue workflow

This folder is the durable copy of API reports used while porting Legacy4J.
Reports go to [horizon-rust/horizon](https://github.com/horizon-rust/horizon);
the port itself is [Jacob0225/Legacy-Horizons](https://github.com/Jacob0225/Legacy-Horizons).
Use GitHub CLI under the authenticated owner's account. The owner already
submitted the early reports: **do not repost any existing numbered report**.

## Before filing

1. Read the relevant local report and existing upstream issue **including
   comments and linked fixes**. Check the pinned SDK's actual hooks, Parts,
   native route and function index. Inspect current upstream agent/reporting
   guidelines; this README records the workflow, not an exemption from them.
2. Reproduce the gap on the exact target, capture the actual refusal/error
   and distinguish API limitations from an implementation bug. Check whether
   a safe native workaround already works. Do not claim every native route
   is impossible merely because one target was refused.
3. Search all issue states using several terms, not only your proposed title:

   ```sh
   gh issue list --repo horizon-rust/horizon --state all --search '"mod API" SEARCH_TERMS' --json number,title,url,state --limit 50
   gh issue view ISSUE_NUMBER --repo horizon-rust/horizon --comments
   gh pr list --repo horizon-rust/horizon --state all --search 'ISSUE_NUMBER OR_FUNCTION_NAME'
   ```

4. If a matching report exists, keep its number and add useful new evidence
   there when authorized. If a closed issue claims the feature exists, locate
   it and test it before treating it as missing. Do not open a duplicate just
   because the pinned nightly predates its fix.
5. A genuinely independent missing decision gets its own report. A working
   native fallback can still accompany a requested ordinary hook; say plainly
   that the native fallback works, and why the public hook is useful.

## Local file and GitHub format

Choose the next unused local number. Local numbers are not GitHub numbers.
Create `NN-short-description.md` with this structure:

```markdown
# mod API: specific missing hook or decision

> 🤖 Filed by ACCOUNT's AI agent (TOOL, ACTUAL_MODEL), not by ACCOUNT personally.

## Port and target
Legacy4J → Legacy Horizons; Minecraft, API, exact Horizon/mixins revisions.

## Required source behavior
Source classes/methods and concrete behavior being ported.

## Reproduction and existing surfaces checked
Minimal reproduction, exact result/error, and the hooks/Parts/native targets tried.

## Request and verification
Smallest required decision/data, lifecycle/side/transaction requirements,
and observable acceptance checks.

## Duplicate check and scope
Queries used, related reports and why this is independent. State remaining
uncertainty and any working native fallback.
```

Replace attribution placeholders using `gh api user --jq .login` and your
actual tool/model. Claude should name Claude and its actual model, not Codex,
GPT-6 or a maintainer from copied examples. Do not invent evidence or quote
an expected result as if you observed it. Keep confidential paths/accounts,
tokens and private world data out of reports.

On GitHub, the title is the local initial heading **without `# `**. The body
starts with the AI attribution; do not repeat that title heading in the body.
Write the exact body to a temporary UTF-8 file and use `--body-file` to preserve
real newlines and avoid shell substitution of report text:

```sh
gh issue create --repo horizon-rust/horizon --title 'mod API: specific missing hook or decision' --body-file /tmp/issue-body.md
```

After a successful response, add `Submitted: https://github.com/.../issues/NUMBER`
to the local report and update the ledger below. Never mark it submitted before
the tool succeeds. Keep errors and disabled experiments in local documentation;
do not leave a startup-crashing experiment enabled in the shipping manifest.

## Comments, replies and implementation follow-ups

Each upstream comment begins `🤖 Agent:` and identifies the current tool/model
and owner. Answer maintainers' questions directly using source and runtime
evidence. Do not claim an upstream issue or promise an engine implementation
merely because you are requesting a hook. Taking upstream implementation work
is separate and follows that repository's current claim/PR rules and routing.

```sh
gh issue comment ISSUE_NUMBER --repo horizon-rust/horizon --body-file /tmp/followup.md
```

Record significant replies, workaround discoveries, exact linked comments
and resolved boundaries locally. Keep the original report as historical
context and add a dated follow-up, rather than silently rewriting old evidence.
Update the port's status docs too. A workaround for part of a report does not
mean the full port or ordinary API is complete. Do not close someone else's
report merely because a native workaround was found.

## Existing report ledger

Starting pointers from the port's November 20 work; recheck live state/comments
before acting. All numbered reports below already have upstream counterparts.

| Local | Upstream | Subject / known follow-up |
|---|---|---|
| 01 | [#18383](https://github.com/horizon-rust/horizon/issues/18383) | Bundled GUI textures |
| 02 | [#18385](https://github.com/horizon-rust/horizon/issues/18385) | Mod-owned title screens |
| 03 | [#18386](https://github.com/horizon-rust/horizon/issues/18386) | Built-in title/options actions |
| 04 | [#18389](https://github.com/horizon-rust/horizon/issues/18389) | Closed; native graceful quit now implemented |
| 05 | [#18390](https://github.com/horizon-rust/horizon/issues/18390) | GUI skin/player preview |
| 06 | [#18396](https://github.com/horizon-rust/horizon/issues/18396) | Active input glyphs |
| 07 | [#18397](https://github.com/horizon-rust/horizon/issues/18397) | Container slot layouts and hitboxes |
| 08 | [#18398](https://github.com/horizon-rust/horizon/issues/18398) | Validated option writes; six native settings now work |
| 09 | [#18647](https://github.com/horizon-rust/horizon/issues/18647) | Controller UI events; native title adapter now works |
| 10 | [#18648](https://github.com/horizon-rust/horizon/issues/18648) | Book/sign render and input hooks |
| 11 | [#18650](https://github.com/horizon-rust/horizon/issues/18650) | Closing world-save drawing |
| 12 | [#18651](https://github.com/horizon-rust/horizon/issues/18651) | Transition cause; maintainer discussed a read-only field |
| 13 | [#18655](https://github.com/horizon-rust/horizon/issues/18655) | Resource-aware title splash |
| 14 | [#18799](https://github.com/horizon-rust/horizon/issues/18799) | Recipe placement refused at a crafting table |
| 15 | [#18811](https://github.com/horizon-rust/horizon/issues/18811) | Local movement/creative flight; inner native targets refused |
| 16 | [#18694](https://github.com/horizon-rust/horizon/issues/18694) | Existing camera-roll issue; commented, no duplicate filed |
| 17 | [#19095](https://github.com/horizon-rust/horizon/issues/19095) | Saved/synced potion data on vanilla cauldrons |
| 18 | [#19105](https://github.com/horizon-rust/horizon/issues/19105) | Cancellable animal love-mode entry |
| 19 | [#19106](https://github.com/horizon-rust/horizon/issues/19106) | Controller-axis processing; native fallback works |

Continue at local 20 only if it is still unused and a new independent report
survives the duplicate check. Commit matching Markdown, relevant status changes
and verified port work together; follow the main
[agent README](../AGENT-README.md) for testing and publishing.
