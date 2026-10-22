# SOP — Nexora Code Development, Multi-Agent, MCP & Production Release

> **Standard Operating Procedure (SOP)**
> โครงการ: **Nexora Code** — Fork ของ [PandaCode](https://github.com/aicodeking/pandacode)
> รหัสเอกสาร: `NC-SOP-001` · ฉบับ: `1.1` · วันที่: `2026-10-22` · สถานะ: **Engineering / ใช้ควบคุมการพัฒนา**
> เจ้าของเอกสาร: Project Owner / Integration Lead · รอบทบทวน: ทุก Release Candidate หรือเมื่อเปลี่ยน Security Policy

**ข้อกำหนดสำคัญ:** *This is a fork of PandaCode* — https://github.com/aicodeking/pandacode. ต้องรักษา LICENSE และเครดิตตามข้อกำหนดต้นฉบับไว้เสมอ เอกสารนี้กำหนดวิธีทำงาน ไม่ใช่หลักฐานว่าโปรแกรมผ่าน Production แล้ว

## 1. วัตถุประสงค์และขอบเขต

กำหนดวิธีมาตรฐานในการพัฒนา ตรวจสอบ รวมโค้ดและเผยแพร่ Nexora Code ซึ่งใช้ **Rust + Tauri 2 + TypeScript** และมีเป้าหมายรองรับ **macOS Apple Silicon M4**, **Windows 11 x64** และ **Linux x64** โดยพัฒนา Advanced Multi-Agent, Workflow DAG, MCP Management และคุณสมบัติด้าน Reliability/Security

SOP ครอบคลุมงานตั้งแต่รับ Requirement → แตก Issue → สร้าง Branch/Worktree → พัฒนา → Review → Tests → PR → Merge Gate → Native Acceptance → Release → Incident/Rollback

**ไม่รวม:** การอนุญาตให้เข้าถึง Credentials, สั่งรัน Tool ที่มีผลข้างเคียง, อนุมัติค่าใช้จ่ายหรือจัดการเครื่องของผู้ใช้ ทั้งหมดนี้ต้องมีสิทธิ์และนโยบายอนุมัติแยกต่างหาก

## 2. กฎการทำงานเมื่อผู้ใช้ไม่อยู่หน้าเครื่อง (Unattended Mode)

**อนุญาตโดยปริยายภายใน Workspace โครงการ:** อ่าน Source, ออกแบบ, เขียน/แก้เอกสารและ Tests, วิเคราะห์ Static Code, รัน Unit/Contract Tests แบบไม่เชื่อมต่อบริการจริง, แก้ Bug ภายในขอบเขต, สร้าง Local Branch/Worktree, Commit งานแยกส่วน และจัดทำรายงาน/หลักฐาน

**ต้องได้รับคำสั่งหรืออนุมัติที่ชัดเจนก่อน:**

- ติดตั้ง/ถอนแอป, เปิด GUI หรือจำลองคลิกในแอป, เปลี่ยน System Settings และ Keychain
- รัน MCP Server ภายนอก, เปิดการเชื่อมต่อที่ถือ Credentials, เชื่อม Cloud หรือใช้งาน API มีค่าใช้จ่าย
- ติดตั้ง Global Packages, ยกระดับสิทธิ์ด้วย `sudo`, แก้ Firewall/Network/Tunnel ของเครื่อง
- Push Branch/เปิด PR บน Remote ที่ไม่ได้ยืนยันปลายทาง, Merge `main`, Publish Release, สร้าง Signed Update, Deploy Production
- ลบข้อมูลจริง, Force Reset/Force Push, ย้าย/เขียนทับไฟล์นอก Workspace, Import Secrets หรือ User Data

**สิ่งที่ให้ทำแทน:** เตรียม Patch, Scripts, Automated Tests, Release Checklist และ Source Commit ไว้ก่อน; แยกงานที่ต้องใช้เครื่องจริงเป็น `PENDING_NATIVE_ACCEPTANCE` พร้อมผู้รับผิดชอบและคำสั่งตรวจสอบ ห้ามระบุว่าทดสอบ GUI/Windows/Linux ผ่านเพียงเพราะ Build หรือ Unit Tests ผ่านบน Mac

## 3. Roles, Ownership และการอนุมัติ

| บทบาท | ขอบเขต | ข้อจำกัด |
|---|---|---|
| Project Owner | ยืนยัน Requirement, Product Priority, Release Approval | อนุมัติงานที่มีผลกระทบสูง |
| Integration Lead | ออกแบบ Contracts, จัดลำดับ Dependency, Review และ Merge Gate | ห้ามยกเว้น Critical Gates โดยไม่มีหลักฐาน |
| Rust/Runtime Worker | Process Supervisor, Storage, Provider, IPC | ห้ามเปลี่ยน Shared Contract โดยไม่ตกลงก่อน |
| MCP Worker | Registry, Transport, Discovery, Permission Policy | ห้ามเปิด Tools/Network โดยถือว่า `enabled=true` คือ Approval |
| Multi-Agent/Workflow Worker | Scheduler, DAG, Journal, Recovery | ห้าม Auto-Replay ผลลัพธ์ `UNKNOWN` ที่อาจเปลี่ยนข้อมูล |
| UI/QA Worker | Accessibility, Playwright, Unit/Integration Tests | ห้ามแทน Native Test ด้วย Mock แล้วติดป้ายผ่านจริง |
| Release/Security Reviewer | Threat Modeling, Supply Chain, Signing, 3-OS Evidence | ต้องแยกผู้ตรวจออกจากผู้สร้าง Artifact เมื่อเหมาะสม |

**One-writer rule:** แต่ละ Task/Agent ต้องใช้ Branch และ Worktree แยกกัน; Shared Contracts เช่น Tauri IPC, Database Schema, Event IDs และ Tool Policy ต้องให้ Integration Lead ประสานลำดับการเปลี่ยน

## 3.1 กฎบังคับ NC-TEAM-001 — ทุกงานต้องมี Checklist Planner และ Team Lead Review

**MUST / ทุกคำสั่งให้ทำงาน:** ไม่ว่างานเล็กหรือใหญ่, Code, Documentation,
Tests, Bugfix หรือคำสั่งว่า "ทำต่อ" ก่อนเริ่มลงมือแก้ไขไฟล์ต้องสร้าง
**Task Checklist Planner ใหม่หนึ่งไฟล์ต่อหนึ่งคำสั่งงาน** ที่
docs/nexora/tasks/YYYY-MM-DD-task-slug.md โดยใช้
[แม่แบบ Checklist Planner](templates/TASK-CHECKLIST.md) งานเล็กให้ทำแผนสั้น
ได้ แต่ห้ามข้ามขั้นตอน ห้ามเอา Planner เก่ามาเปลี่ยนสถานะหลอกว่าเป็นงานใหม่

**Team Lead / Integration Lead เป็นผู้รับผิดชอบการส่งมอบสุดท้าย** และต้อง
ทำตามลำดับต่อไปนี้:

1. **Intake:** รับโจทย์, ยืนยัน Scope/Expected Outputs, Priority, Risk,
   OS, Dependencies, Acceptance Criteria และ Approval Boundaries ลง Planner
2. **Decompose:** แบ่งเป็นงานย่อย ID ชัดเจน, ระบุ Input/Output/DoD, กำหนด
   Critical Path และเลือก Worker ตามความถนัด (Rust, MCP, UI, Tests,
   Security, Docs) โดย Team Lead เป็นคนจัดสรรงานและตรวจความพร้อม
3. **Assign:** จัด Agent ตัวจริงให้แต่ละงาน พร้อมบันทึก Worker ID,
   Worktree/Branch, ไฟล์ที่รับผิดชอบ, Dependencies, Test Gate และ Deadline
   ถ้ามี ห้าม Agent สองตัวแก้ Shared Files พร้อมกันโดยไม่มี Lead ประสาน
4. **Implement & Handoff:** Workers ทำงานเฉพาะขอบเขต ส่ง Diff/Commit,
   Test Logs, ความเสี่ยง, Known Issues และ Checklist สถานะจริงให้ Lead
   Worker ไม่มีสิทธิ์ประกาศว่าผ่าน Review หรือส่งมอบแทน Lead
5. **Lead Review:** หัวหน้าทีมต้องตรวจ Diff จริงของ **ทุกงานย่อย**
   รวมถึง Functional Logic, Security, Side Effects, Cleanup, Compatibility,
   Tests, Ignored Tests, CI และ Native E2E; หากพบข้อผิดพลาดให้ส่งกลับแก้,
   ตรวจซ้ำก่อนรับงาน ห้ามเชื่อคำว่า "Done" โดยไม่มีหลักฐาน
6. **GO/NO-GO & Delivery:** บันทึก Identity ของผู้ตรวจ, Test SHA, OS,
   PASS/FAIL/IGNORED, CI URL, Pending Gates, Final Decision และอนุมัติ
   GO ก่อนแจ้งส่งมอบ หรือ NO-GO พร้อมงานที่ค้าง; Merge main/Deploy/Sign/
   Release/ติดตั้ง/สั่งรัน External MCP ต้องได้รับสิทธิ์แยกตาม SOP

**กรณีไม่มีเครื่องมือสร้างหรือมอบหมาย Agent จริง:** ใช้
TEAM_LEAD_SERIAL เป็น Worker ใน Planner, ทำงานตามลำดับ, รีวิวจริงด้วยตนเอง,
และรายงานตรงไปตรงมาว่าไม่มี Independent Reviewer อย่าประดิษฐ์ Agent หรือ
ปลอมผลรีวิว ถ้าต้องการ Reviewer อิสระเป็น Release Gate ให้ใช้ NO-GO/BLOCKED
จนกว่ามีผู้ตรวจจริง

**Definition of Done:** งานทุกชิ้นมี Planner, Decomposition, Assignment,
Worker Handoff, Team Lead Review, Evidence และ GO/NO-GO ก่อนส่งมอบ
PR ต้องลิงก์ Planner และบันทึกผลรีวิวของ Lead การเขียนกฎไม่ได้หมายความว่า
ระบบ R3 Multi-Agent Scheduler พัฒนาเสร็จแล้ว

**GitHub CI Gate:** [Nexora Task Planner Gate](../../.github/workflows/nexora-planner-gate.yml)
เรียก [governance-pr-gate.mjs](../../scripts/nexora/governance-pr-gate.mjs)
เพื่อตรวจว่า PR มี Planner ใหม่จริง ชื่อไฟล์ถูกต้อง มี Assignment,
Acceptance Criteria, Evidence และ Team Lead verdict เป็น GO สำหรับการส่ง PR
(ไม่ใช่อนุมัติ Merge) ถ้าไม่ครบให้ CI Failed แบบปฏิเสธไว้ก่อน
**ข้อจำกัด:** Automated Check ตรวจโครงสร้างเอกสารเท่านั้น ไม่สามารถพิสูจน์ว่า
ผู้ตรวจเป็นคนละคน ไม่สามารถแทน Required Reviewer หรือป้องกันการแก้ไข Rule
ด้วยตัวเอง ต้องเปิด Branch Protection, Required Checks และ Code Owner Review
แยกก่อนใช้เป็นการบล็อก Merge จริง

## 4. Repository / Branch / Worktree Policy

Repository เป้าหมาย: `TheerasakPing/nexora-code` · Upstream: `aicodeking/pandacode` · Source ตั้งต้น PandaCode `v0.3.16`, commit `30f9c823e3c38f472f89e8f8405575ecfe7173ee`

| Branch | วัตถุประสงค์ | กฎ |
|---|---|---|
| `main` | Stable / Production | Protected, ต้องผ่าน Release Gates |
| `develop` | Integration | PR + CI + Reviewer; ยังไม่ถือเป็น Production |
| `feat/rN-...` | ฟีเจอร์แต่ละเฟส | แยก Worktree และ Commit ตามขอบเขต |
| `fix/...`, `test/...`, `docs/...` | Bug, Tests, Documents | แยก Diff ให้เล็กและตรวจได้ |
| `release/vX.Y.Z` | Release Candidate | ห้ามรวมฟีเจอร์ใหม่ที่ไม่เกี่ยวข้องหลัง Freeze |

**ขั้นตอนเริ่มงาน:**

1. ยืนยันว่าทำงานใน `/Volumes/Data/workspace/git/nexora-code` หรือ Workspace ที่ Project Owner กำหนด **ไม่ใช่** `nexora-studio` หรือ `ORG2`.
2. ตรวจ `git status --short --branch`, `git log -3 --oneline` และ `git remote -v`.
3. ตรวจความสะอาด Working Tree และจัดการงานค้างโดยไม่ทิ้งหรือเขียนทับของผู้อื่น.
4. สร้าง Issue/Task ระบุ Goal, Scope, Dependencies, File Ownership, Acceptance Criteria, Risk และ Test Plan.
5. สร้าง Branch/Worktree แยก; ห้ามให้สอง Workers แก้ไฟล์เดียวกันพร้อมกันโดยไม่มีกติกา Merge.
6. ทำแพตช์เล็กที่สุดที่ตอบ Requirement; หลีกเลี่ยง Rewrite UI/Rust Core ทั้งก้อนโดยไม่จำเป็น.
7. หลังทดสอบให้ Commit ด้วยข้อความเช่น `feat(mcp): ...`, `fix(runtime): ...`, `test(workflow): ...`, `docs(sop): ...`.
8. ถ้ามี Remote ที่ยืนยันและได้รับอนุญาต จึง Push/เปิด PR; มิฉะนั้นเก็บ Local Commit และบันทึก `REMOTE_PENDING`.

## 5. ขั้นตอนมาตรฐานสำหรับ Task และ PR (Execution SOP)

### 5.1 Intake / Design

- ยืนยันปัญหา, ผลลัพธ์ที่ต้องการ, OS ที่กระทบ, Compatibility และ Data/Security Impact
- กำหนด Severity: `P0 Security/Data Loss`, `P1 Reliability`, `P2 Feature`, `P3 Polish`
- แยก Public Contracts, Internal Interfaces และ Migrations ให้ชัดเจน
- ทำ Threat Model สั้น ๆ สำหรับ Task ที่แตะ Filesystem, Process, IPC, MCP, OAuth, Git และ Auto-update
- นิยามเกณฑ์ผ่านก่อนเริ่มเขียนโค้ด พร้อม Test Cases ปกติ/ขอบเขต/ล้มเหลว

### 5.2 Implement

- Rust เป็นตัวควบคุม Native Side Effects; Renderer ส่ง Typed Requests ผ่าน IPC เท่านั้น
- ตรวจข้อมูลทุก Field ฝั่ง Rust ก่อนอ่าน/เขียนไฟล์ เปิด Process หรือร้องขอ Network
- เพิ่ม Limits สำหรับ Output, Timeouts, Queues, Concurrency และ Resource Usage
- บันทึก Cancellation, Deadlines, Spawn/Exit และ Error States ในจุดตรวจสอบได้
- Secrets อยู่ใน OS Secure Store หรือ Provider Credential Layer ไม่อยู่ใน JSON Config, Logs, Prompt หรือ Frontend State
- เปลี่ยน Schema แบบ Versioned Migration ที่ทดสอบข้อมูลจากเวอร์ชันเดิมได้
- รักษาความสามารถต้นฉบับและ Licensing; หลีกเลี่ยงผลกระทบต่อ Session ที่มีอยู่

### 5.3 Review / Merge

- PR ต้องมี: Summary, Motivation, Scope, Files Touched, Risk, Test Evidence, Screenshots (เฉพาะงาน UI), Rollback Steps และ Known Limitations
- แสดง `git diff --check`; ตรวจ Secret/Token/Private Key และไฟล์ Output ที่อาจถูก Commit โดยไม่ตั้งใจ
- Review ต้องแยก Functional Correctness, Security Boundaries, Process/Resource Lifetime และ Platform Compatibility
- **ห้าม** Merge ด้วยเหตุผลว่า CI ไม่เริ่มรัน; ต้องแยกปัญหา Runner/Billing/Provisioning กับ Failure ใน Code และบันทึก Evidence
- Merge ได้เมื่อ Required Checks และ Reviewer Approval ผ่านเท่านั้น; ไม่ Force Push Protected Branch

## 6. Standard Test Gate — รันจาก Source โดยไม่ติดตั้งแอป

จาก Root ของ Repository ที่ผ่านการตรวจสอบแล้ว:

```bash
# ใช้ Node.js 22 เป็นเป้าหมาย CI; ติดตั้ง Dependencies เฉพาะในโปรเจกต์เมื่อได้รับอนุญาต
npm ci
npm run build
npm test
npm run test:release
npm run test:nexora:r0

# Rust (ใช้ Lockfile ตรงกัน)
cargo test --manifest-path src-tauri/Cargo.toml --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings

# ตรวจ Diff ก่อน Commit
git diff --check
```

**หลักเกณฑ์รายงาน:** ต้องระบุวันที่, Host/OS/Arch, Node/Rust Version, Commit SHA, คำสั่งจริง, จำนวน PASS/FAIL/IGNORED และ Exit Code. Tests ที่ Skip/Ignored ต้องเปิดเผย ไม่ถือว่าผ่าน. Vite Warning เป็น Warning ไม่ใช่ Test Failure แต่ต้องบันทึกไว้สำหรับ Performance Review

**ระดับการทดสอบ:**

| Gate | Test | ผ่านเมื่อ |
|---|---|---|
| G0 | Static/Format/Lint | Build + Clippy + Diff Check ไม่มี Errors |
| G1 | Unit / Contract | Rust, Node และ Protocol Fixtures ผ่าน |
| G2 | Integration | Temporary Repo/Mock MCP/Session/Storage ผ่าน; ไม่มี Side Effects ต่อข้อมูลจริง |
| G3 | Native E2E | แอปเปิดบน OS จริง, Terminal, Provider, MCP/Approval และ UI ใช้งานได้ |
| G4 | Reliability & Security | Crash, Cancellation, TOCTOU, Credential Redaction, SSRF, No-Replay ผ่าน |
| G5 | Release | Signed/Notarized/Install/Upgrade/Rollback พร้อม Evidence ครบ |

**เมื่อ Test ไม่ผ่าน:** หยุดการอ้างว่า Pass; เก็บ Log โดยตัด Secrets, ทำ Minimal Reproduction, ระบุว่าเป็น Source Regression/Environment/Flaky, แก้สาเหตุหรือ Mark Blocked พร้อม Owner. ห้ามแก้ด้วยการปิด Test หรือลด Security Check เพื่อให้ผ่านโดยไม่มีเหตุผลและ Review

## 7. Advanced MCP SOP (R2)

**Lifecycle ที่ตั้งใจให้เกิดเมื่อพัฒนาครบ:** `DISCOVERED → CONFIGURED → APPROVAL_REQUIRED → CONNECTING → READY → DEGRADED/FAILED → STOPPED`

- Registry เป็น **Configuration Only**. `enabled=true` ไม่ได้แปลว่าผู้ใช้ยอมให้รัน Process หรือให้สิทธิ์ Tool
- Transport ที่วางแผนคือ stdio และ Streamable HTTP; ต้องทำ Version/Capability Negotiation จริงก่อนใช้งาน
- MCP `initialize`, `tools/list`, `resources/list`, `prompts/list` ต้องมี Deadline, Output Cap, Pagination Cap และ Error Handling
- `tools/call` ที่อ่านข้อมูลและที่แก้ข้อมูลต้องมี Permission Classes ต่างกัน; Mutating Tools ต้อง Approval ตาม Policy
- HTTP Client ต้องตรวจ HTTPS/Loopback, DNS, Redirects, Rebinding, Proxy, Private/Internal IP และผลจากการเปลี่ยน Network ก่อน Connect จริง; **การตรวจ URL ก่อนบันทึกอย่างเดียวไม่เพียงพอ**
- stdio ต้องมี Executable Allowlist/Approval, Environment Allowlist, Sandbox/Least Privilege, Process Tree Cleanup และ Windows Job Object
- Credential Reference ใช้ OS Secure Store เท่านั้น ไม่ส่ง Secret เข้า Renderer, Config Export หรือ Diagnostic Bundle
- Test ต้องครอบคลุม Server Hang/Crash, Auth Failure, Tool Namespace Collision, Malformed Frames, Oversized Responses, Rate/Concurrency Limits, Cancel และ Secret Leakage
- หากผล Side Effect ไม่แน่ชัด ให้บันทึก `UNKNOWN`, ห้าม Retry อัตโนมัติจนกว่าจะ Reconcile กับ External State

**ขอบเขตที่มีอยู่ ณ SOP v1.0:** มี Rust MCP Registry และ Typed IPC แต่ยังไม่ถือว่ามี MCP Client Connection Manager, Transport Supervisor, GUI Inspector หรือ Credential Vault พร้อมใช้งานจริง

## 8. Advanced Multi-Agent SOP (R3)

1. Architect แปลง Goal เป็น Tasks พร้อม Inputs/Outputs, Dependencies, File Ownership และ Definition of Done
2. Scheduler คำนวณ DAG, Priority, Bounded Concurrency และ CPU/RAM/Token Budget; เริ่ม Worker เฉพาะ Task ที่ Ready
3. Worker แต่ละตัวใช้ Process Identity/Generation, Session และ Git Worktree ของตนเอง; ทำงานในขอบเขต Policy ที่ได้รับอนุมัติ
4. Mailbox/Handoff ต้องระบุ Sender, Receiver, Task ID, Generation, Content Digest และผล Ack; ไม่รับข้อความจาก Generation เก่า
5. Reviewer ตรวจ Diff, Conflicts, Tests, Security และ Provenance ก่อน Merge Worktrees
6. Stop/Pause/Resume ต้องหยุด Process Tree, ปิด Handles, บันทึก Checkpoint และไม่ Reuse Approval จาก Run ก่อน
7. งานที่ทำ Side Effects ต้องมี Effect Fence/Lease เพื่อไม่ให้ Worker เก่าทำต่อหลัง Handoff
8. หาก Result เป็น `UNKNOWN` หรือ Context เสียหาย ให้ Block Dependent Tasks และแจ้งให้ Operator Reconcile

**Acceptance ตัวอย่าง:** 4 Workers พร้อมกันโดยไม่มี Deadlock/File Collision ที่ข้าม Review; ทดสอบ 1/4/8/16 Workers โดย Scheduler Queue/Reject เมื่อเกิน Budget. Worktree **ไม่ใช่** OS Security Sandbox ต้องมี Sandbox จริงก่อนเปิดใช้ Untrusted Tools

## 9. Workflow Engine SOP (R4)

Node ประเภทเป้าหมาย: `agent`, `mcp_tool`, `condition`, `approval`, `test`, `review`, `git`, `notification`, `artifact`.

State Machine: `PENDING → READY → RUNNING → SUCCEEDED | FAILED | CANCELLED | UNKNOWN`.

- ตรวจ DAG Acyclic, Required Inputs, Permissions, Schema Version และ Resume Rules ก่อนเริ่ม
- เขียน Intent ลง Durable Journal ก่อน Dispatch; บันทึก Outcome พร้อม Tool Call ID / Idempotency Key ถ้าระบบปลายทางรองรับ
- Retries ทำได้เฉพาะ Read-only/Idempotent Operations ที่พิสูจน์ได้; ไม่ Retry `UNKNOWN` ของ Mutating Calls
- Checkpoint ต้อง Crash-Consistent และใช้ Transaction ในการอัปเดต Nodes/Dependencies
- Resume เฉพาะ Nodes ที่ปลอดภัย; Failed/Unknown ที่ต้อง Reconcile ห้ามปลดล็อกอัตโนมัติ
- Workflow Template ต้องมี Version, Import Validation และ Permission Diff
- Release Gate ต้องทดสอบ Force Kill ระหว่าง Tool, DB Crash, Stale Generation, Tool Timeout และ Replay Prevention

## 9.1 Rebranding SOP — Brand, UI, Compatibility, Release

- ใช้ [Rebranding Master Plan](REBRANDING-PLAN.md) เป็น Mandatory Workstream BR-01–BR-07 ตลอด R0–R8; ชื่อ Nexora Code เป็น Working Name จนกว่า Owner อนุมัติ
- BR-01 ต้องตรวจชื่อ/โลโก้/สัญลักษณ์/Trademark ก่อนประกาศชื่อจริงหรือใช้ Asset แบบ Public
- BR-02 และ BR-03 สร้าง Logo/Icons/Tokens/Light-Dark/Accessibility โดยไม่ทำให้ UX และความเร็วของ Agent Workbench ถดถอย
- BR-04 ตรวจ About, User Copy, Bundle/Executable/Installer Metadata, Website และ Support Links; ต้องมีข้อความ Attribution และ LICENSE ต้นฉบับ
- BR-05 ต้องทำ Data/Keychain/DB/Executable/Tool-Protocol Compatibility Tests และ Backup/Rollback ก่อนย้ายหรือเปลี่ยนชื่อของเดิม; ห้าม Global Replace ชื่อภายในทั้งโปรเจกต์
- BR-06/BR-07 ต้องมี Native UI และ Installer Evidence บน Mac M4, Windows x64, Linux x64 และ Nexora-owned Signing/Updater Keys ก่อน Stable
- เปลี่ยน Tauri productName หรือ App ID สำเร็จไม่เท่ากับ Rebranding Complete: ต้องมี Owner Approval, Visual Design, Compatibility, Legal และ 3-OS Gates
- เมื่อผู้ใช้ไม่อยู่หน้าเครื่อง ทำเฉพาะ Specs/Source/Unit Tests/Docs/Local Commits; ห้าม Install, Launch GUI, Push, Sign หรือ Publish โดยพลการ

## 10. Cross-Platform Native Acceptance

| รายการ | macOS M4 | Windows 11 x64 | Linux x64 |
|---|---|---|---|
| Target | `aarch64-apple-darwin` | `x86_64-pc-windows-msvc` | `x86_64-unknown-linux-gnu` |
| Webview | WKWebView | WebView2 | WebKitGTK |
| Terminal | PTY | ConPTY | PTY |
| Secret Store | Keychain | Credential Manager | Secret Service |
| Installer | DMG | NSIS EXE | AppImage (และ DEB เมื่อพร้อม) |
| Native test | เครื่อง M4 จริง | Windows 11 จริง/VM ที่ได้รับอนุมัติ | Ubuntu x64 จริง/VM ที่ได้รับอนุมัติ |

**Native Acceptance Checklist:** Install → First Launch → PATH/CLI Discovery → Provider Authentication → Terminal → MCP Consent → Multiple Sessions → Stop/Resume → Close/Reopen → Update/Rollback → Uninstall/Data Preservation Policy

บน macOS ต้องทดสอบเปิดจาก Finder ที่ PATH ไม่มี Homebrew Tools; Windows ต้องตรวจ WebView2, ConPTY และ Process Job Objects; Linux ต้องตรวจ X11/XWayland, WebKitGTK, Secret Service และ FUSE/AppImage. ห้ามนำผล Cross-build ไปอ้างเป็น Native GUI Acceptance

## 11. Performance & Stability Thresholds (เป้าหมาย ไม่ใช่ผลทดสอบ)

- Cold Start และ Idle RAM ไม่แย่กว่าต้นฉบับเกิน **20%** บนเครื่องและ Scenario เดียวกัน
- UI Input Latency P95 ไม่เกิน **100 ms** ภายใต้ Standard Streaming Fixture
- Stop/Cleanup Process Tree ภายใน **10 วินาที** ภายใต้ Failure Fixture
- Soak Test **8 Workers / 2 ชั่วโมง** ไม่มี Deadlock, Orphan Process หรือ Memory Growth ไม่จำกัด
- Cancel/Crash/Resume **50 รอบต่อ OS** โดยไม่พบ Duplicate Mutation หรือ Committed Journal Loss
- วัด Baseline และ Release Candidate ที่ **Source SHA เดียวกัน** พร้อมเก็บคำสั่ง เครื่อง รุ่น OS และไฟล์ผลลัพธ์

เกณฑ์อาจเปลี่ยนได้โดย Project Owner ผ่าน Revision ของ SOP และต้องบันทึกเหตุผล ห้ามแก้ย้อนหลังเพื่อให้ Build ผ่าน

## 12. Release SOP — Alpha → Beta → RC → Production

### Pre-release

1. Freeze Branch/Commit SHA และตรวจว่า Merge Gates ผ่านครบ
2. เรียกดู Test Report ที่ตรง SHA; ถ้า Runner ไม่เริ่มรัน ต้องแยก Infrastructure Error กับ Code Failure
3. สร้าง Artifact บน Native Runner ของแต่ละ OS ด้วย Locked Toolchain/Dependencies
4. เก็บ SBOM, Dependency Scan, License Notices, Checksums และ Provenance
5. ตรวจ Installer บน Clean Environment จริงก่อนอนุญาตเผยแพร่
6. ตรวจ Windows Publisher Signing, macOS Developer ID + Hardened Runtime + Notarization และ Linux Integrity/Distribution Trust
7. ตรวจ Tauri Updater Manifest ใช้ **Nexora-owned Keys/Endpoints เท่านั้น**; ปัจจุบัน Updater ถูก Disable ไว้จนผ่าน Gate นี้
8. บันทึก Release Approval โดย Project Owner และ Release/Security Reviewer

### Go/No-Go

**GO** เมื่อ G0–G5 ครบทุกระบบ, ไม่มี Critical/High ที่ไม่รับความเสี่ยง, ไม่พบ Unknown Mutating Effects, มี Recovery/Rollback Plan และ Release Notes ตรง Artifact SHA

**NO-GO** หาก Build ไม่ผ่าน, ไม่มี Native Evidence, Code Signing/Updater Key ไม่ถูกต้อง, Credential Leak, Data Loss, Crash Recovery ไม่ปลอดภัย หรือยังมี Unresolved Critical Incident

### Post-release

- ส่งไปที่ `alpha`/`beta`/`stable` ตาม Channel เท่านั้น
- เฝ้าตรวจ Crash/Support Reports โดยคุ้มครองข้อมูลผู้ใช้และไม่เปิด Telemetry ใหม่โดยพลการ
- หาก Incident รุนแรง ให้หยุด Rollout, ปิด Channel ที่มีปัญหาและเริ่ม Incident SOP

## 13. Incident, Recovery, Rollback

1. ตั้งสถานะ `INCIDENT`, กำหนด Severity/Owner และหยุดการ Publish/Auto-Deploy ที่ได้รับผลกระทบ
2. รวบรวม Source SHA, OS, Version, Reproduction Steps และ Log ที่ลบ Secrets แล้ว
3. แยกประเภท: Session Hung, Process Leak, MCP/Provider Timeout, Data Integrity, Permission Bypass, Update Failure
4. หยุด Side Effects ที่ไม่แน่ชัด; **ไม่กด Replay หรือ Restart Mutating Tasks อัตโนมัติ**
5. กู้คืนจาก Durable Checkpoint / Verified Backup และ Reconcile กับ External State ก่อน Resume
6. ออก Fix ใน Branch แยก, เพิ่ม Regression Test, ผ่าน Gates และ Release Approval เช่นเดิม
7. ทบทวน Root Cause และเพิ่ม Preventive Checks ภายใน SOP/Tests

**Rollback:** ย้อนกลับ Binary/App Release ได้ แต่การย้อน Database Schema ต้องใช้ Migration/Restore ที่ผ่านการทดสอบ; ห้ามสันนิษฐานว่าสามารถใช้ไฟล์ Database ใหม่กับ App เก่าได้เสมอ

## 14. Evidence, Handoff และ Definition of Done

ทุก Commit/PR ที่ส่งมอบต้องมีบันทึกขั้นต่ำ:

```text
Task/Issue:
Branch/Commit SHA:
Scope and Changed Files:
Host OS / Architecture / Toolchain:
Commands executed:
Tests PASS / FAIL / IGNORED / NOT RUN:
Security Review / Risks:
Impact on other platforms:
Rollback procedure:
Approval required / pending:
Next owner and next safe action:
```

**Definition of Done:** ตรง Requirement, No Unapproved Side Effects, Unit/Contract Tests ครบ, CI/Native Gates ตาม Scope ผ่าน, Diff Review สำเร็จ, เอกสาร/Changelog อัปเดต, Data/Migration/Permissions ปลอดภัย และระบุ Known Limitations ชัดเจน

## 15. ขั้นตอนดำเนินงานปัจจุบันและลำดับถัดไป

- **R0:** Fork Identity, Attribution, Updater Isolation และ Source Tests ได้รับการพัฒนาใน Local Branch แล้ว แต่ไม่ใช่ Production Acceptance
- **R1:** Path Hardening และ DMG Engineering Alpha มีหลักฐานเฉพาะ Mac; ยังติดค้าง TOCTOU/Native UI/3-OS Acceptance
- **R2:** MCP Registry/Typed Native IPC ถูกพัฒนาและมี Unit Tests; ยังไม่เชื่อม Transport หรือเปิด MCP Server จริง
- **R3–R8:** ต้องเปิดงาน/ตรวจ Gate ต่อเนื่องตาม [Engineering Roadmap](ROADMAP.md)
- **Remote GitHub:** ตรวจสิทธิ์และยืนยัน Repository จริงก่อน Push; ห้ามกล่าวว่า PR/CI ทำงานแล้วหากมีแต่ Local Commit
- **โหมดตอนผู้ใช้ไม่อยู่หน้าเครื่อง:** ให้ทำ Source/Tests/Docs/Local Commits ต่อได้ แต่ **ไม่ติดตั้งแอป**, ไม่เปิด GUI, ไม่ Push/Merge/Publish โดยปราศจากสิทธิ์หรือคำสั่งที่เหมาะสม

## 16. References

- [Nexora Code Roadmap](ROADMAP.md)
- [Rebranding & Product Identity Master Plan](REBRANDING-PLAN.md)
- [R0 Acceptance](R0-ACCEPTANCE.md)
- [R1 Workspace Path Hardening](R1-WORKSPACE-PATH-HARDENING.md)
- [R2 MCP Registry Foundation](R2-MCP-REGISTRY-FOUNDATION.md)
- [R2 Source Verification](R2-SOURCE-VERIFICATION.md)
- [Security Decisions](SECURITY-DECISIONS.md)
- [PandaCode upstream](https://github.com/aicodeking/pandacode)

---

**Approval record**

| Version | วันที่ | การเปลี่ยนแปลง | ผู้อนุมัติ |
|---|---|---|---|
| 1.0 | 2026-10-22 | ฉบับเริ่มต้นสำหรับ R0–R8, Multi-Agent, MCP และ Production Gates | *Pending Project Owner review* |
| 1.1 | 2026-10-22 | เพิ่ม Mandatory Rebranding BR-01–BR-07, Brand Identity Approval และ Migration/Native Release Gates | *Pending Project Owner review* |
