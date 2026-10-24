# Test-only GUI readiness on a disposable GitHub-hosted Windows runner.
param(
  [Parameter(Mandatory = $true)][string]$AppPath,
  [int]$TimeoutSeconds = 40
)
$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted' -or
    -not [System.IO.File]::Exists($AppPath)) {
  throw 'Windows visible-window smoke requires a hosted runner and installed app'
}
$process = Start-Process -FilePath $AppPath -WorkingDirectory (Split-Path -Parent $AppPath) -PassThru
try {
  $deadline = [DateTime]::UtcNow.AddSeconds($TimeoutSeconds)
  while ([DateTime]::UtcNow -lt $deadline) {
    $process.Refresh()
    if ($process.HasExited) { throw 'Installed app exited before a native GUI window appeared' }
    if ($process.MainWindowHandle -ne [IntPtr]::Zero -and
        $process.MainWindowTitle -like '*Nexora Code*') {
      $handle = $process.MainWindowHandle
      Start-Sleep -Seconds 2
      $process.Refresh()
      if ($process.HasExited -or $process.MainWindowHandle -ne $handle -or
          $process.MainWindowTitle -notlike '*Nexora Code*') {
        throw 'Installed app native window did not survive the stability check'
      }
      Write-Output "WINDOWS_NATIVE_WINDOW_PASS pid=$($process.Id) title=$($process.MainWindowTitle) handle=$handle"
      $found = $true
      break
    }
    Start-Sleep -Milliseconds 500
  }
  if (-not $found) { throw "No visible Nexora Code main window within $TimeoutSeconds seconds" }
} finally {
  $process.Refresh()
  if (-not $process.HasExited) {
    # Kill descendants while the owned root is provably alive. Never kill a reaped PID.
    & taskkill.exe /PID $process.Id /T /F 2>$null | Out-Null
    if ($LASTEXITCODE -ne 0) {
      $process.Refresh()
      if (-not $process.HasExited) { throw 'Hosted installed-app process cleanup failed' }
    }
  }
  $process.Dispose()
}
