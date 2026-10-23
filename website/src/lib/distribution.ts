// Fork-owned distribution links only. Never direct Nexora users to upstream
// PandaCode installers, or assume an unsigned test build is a signed release.
const repoUrl = 'https://github.com/TheerasakPing/nexora-code';
const previewBuilds = `${repoUrl}/actions/workflows/nexora-r6-alpha.yml`;
export const project = {
  repo: repoUrl,
  clone: `${repoUrl}.git`,
  releases: previewBuilds, // Temporary preview build list, NOT an approved Release.
  issues: `${repoUrl}/issues`,
  security: `${repoUrl}/security/policy`,
};
// GitHub Actions artifacts are temporary and require opening the successful
// workflow run. Do not invent URLs for nonexistent, signed release assets.
export const platformDownloads = [
  { label: 'macOS', detail: 'Apple silicon · Actions preview', href: previewBuilds },
  { label: 'Windows', detail: 'x64 · Actions preview', href: previewBuilds },
  { label: 'Linux', detail: 'x64 · Actions preview', href: previewBuilds },
] as const;
export const distribution = {
  platforms: 'macOS · Windows · Linux',
  isUnsignedPreview: true,
  requirements: 'Apple silicon · macOS 12+',
  previewNotice: 'Ad-hoc signed only, NOT Apple notarized. macOS Gatekeeper will block normal opening; follow the verified owner-only Preview instructions.',
  appleSupportUrl: 'https://support.apple.com/en-us/guide/mac-help/mh40617/mac',
};
