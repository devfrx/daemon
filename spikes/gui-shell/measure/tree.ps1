<#
SP-8 -- RSS, CPU and VRAM of a whole PROCESS TREE while the shell runs (M1, M4, M5), and the page's own
numbers read from the window title (M3, P1, P2). PowerShell 5.1.

Usage:
  powershell -NoProfile -ExecutionPolicy Bypass -File tree.ps1 -Exe <shell exe> [-ArgumentList "<args>"] [-WorkingDirectory <dir>]
      [-Seconds 40] [-IntervalMs 250] [-Emitter <core.exe>] [-EmitterAt 15] [-Csv <outside the repo>] [-KeepRunning]
  ... or -AttachPid <pid> instead of -Exe, to sample a shell that is already running (task 7).

The tree is rebuilt at every sample from Win32_Process.ParentProcessId, because WebView2 lives OUTSIDE the
Tauri PID (M1). RSS is the sum of WorkingSet64. CPU is the delta of the summed TotalProcessorTime over the
interval, as a percentage of ONE core: P3 is "< 25% of one core". VRAM is the sum of
'\GPU Process Memory(*)\Dedicated Usage' over the tree's PIDs, read once per second (the counter is slow);
on a machine whose webview runs on an INTEGRATED GPU, Dedicated Usage is 0 by construction and Shared Usage
is the footprint, so both counters are read and reported (errata E10 of the plan, 2026-09-10).
The emitter, if given, is started at -EmitterAt seconds: before it the samples are the REST, after it the
STREAM (the shells retry the pipe every two seconds, then 2000 messages take ten seconds, then the emitter
exits: the STREAM phase includes that wait and that tail, and the peak is what P3 judges). At the end the
tree is stopped unless -KeepRunning or -AttachPid.
#>
param(
  [string]$Exe = "",
  [string]$ArgumentList = "",
  [string]$WorkingDirectory = ".",
  [int]$AttachPid = 0,
  [int]$Seconds = 40,
  [int]$IntervalMs = 250,
  [string]$Emitter = "",
  [int]$EmitterAt = 15,
  [string]$Csv = "",
  [switch]$KeepRunning
)

function Get-Tree([int]$root) {
  $all = Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId
  $set = New-Object 'System.Collections.Generic.HashSet[int]'
  [void]$set.Add($root)
  do {
    $grew = $false
    foreach ($p in $all) {
      if ($set.Contains([int]$p.ParentProcessId) -and -not $set.Contains([int]$p.ProcessId)) {
        [void]$set.Add([int]$p.ProcessId)
        $grew = $true
      }
    }
  } while ($grew)
  return @($set)
}

if ($AttachPid -gt 0) {
  $root = $AttachPid
  Write-Output ("attached to pid {0}" -f $root)
} elseif ($Exe -ne "") {
  $startArgs = @{ FilePath = $Exe; WorkingDirectory = $WorkingDirectory; PassThru = $true }
  if ($ArgumentList -ne "") { $startArgs.ArgumentList = $ArgumentList }
  $shell = Start-Process @startArgs
  $root = $shell.Id
  Write-Output ("shell started: pid {0} ({1})" -f $root, $Exe)
} else {
  throw "give -Exe or -AttachPid"
}

$t0 = Get-Date
$end = $t0.AddSeconds($Seconds)
$emitterStarted = $false
$rows = @()
$prevCpu = $null
$prevT = $t0
$vram = 0
$vramShared = 0
$lastVram = $t0.AddSeconds(-10)
Start-Sleep -Milliseconds 500

while ((Get-Date) -lt $end) {
  $now = Get-Date
  if (($Emitter -ne "") -and -not $emitterStarted -and (($now - $t0).TotalSeconds -ge $EmitterAt)) {
    Start-Process -FilePath $Emitter | Out-Null
    $emitterStarted = $true
    Write-Output ("emitter started at {0:N1} s" -f ($now - $t0).TotalSeconds)
  }
  $pids = Get-Tree $root
  $ps = @(Get-Process -Id $pids -ErrorAction SilentlyContinue)
  $rss = ($ps | Measure-Object WorkingSet64 -Sum).Sum
  $cpu = ($ps | ForEach-Object { $_.TotalProcessorTime.TotalMilliseconds } | Measure-Object -Sum).Sum
  $pct = 0
  if ($null -ne $prevCpu) { $pct = ($cpu - $prevCpu) / ($now - $prevT).TotalMilliseconds * 100 }
  if (($now - $lastVram).TotalMilliseconds -ge 1000) {
    $samples = (Get-Counter '\GPU Process Memory(*)\Dedicated Usage', '\GPU Process Memory(*)\Shared Usage' -ErrorAction SilentlyContinue).CounterSamples
    $mine = $samples | Where-Object { ($_.InstanceName -match '^pid_(\d+)_') -and ($pids -contains [int]$Matches[1]) }
    $vram = ($mine | Where-Object { $_.Path -match 'dedicated usage' } | Measure-Object CookedValue -Sum).Sum
    if ($null -eq $vram) { $vram = 0 }
    $vramShared = ($mine | Where-Object { $_.Path -match 'shared usage' } | Measure-Object CookedValue -Sum).Sum
    if ($null -eq $vramShared) { $vramShared = 0 }
    $lastVram = $now
  }
  $title = ($ps | Where-Object { $_.MainWindowTitle } | Select-Object -First 1).MainWindowTitle
  $rows += [pscustomobject]@{
    t_s = [math]::Round(($now - $t0).TotalSeconds, 2)
    phase = $(if ($emitterStarted) { "stream" } else { "rest" })
    procs = $pids.Count
    rss_mb = [math]::Round($rss / 1MB, 1)
    cpu_pct = [math]::Round($pct, 1)
    vram_mb = [math]::Round($vram / 1MB, 1)
    vram_shared_mb = [math]::Round($vramShared / 1MB, 1)
    title = $title
  }
  $prevCpu = $cpu
  $prevT = $now
  Start-Sleep -Milliseconds $IntervalMs
}

$rows | Format-Table t_s, phase, procs, rss_mb, cpu_pct, vram_mb, vram_shared_mb -AutoSize | Out-String -Width 200 | Write-Output
foreach ($phase in @("rest", "stream")) {
  $r = @($rows | Where-Object { $_.phase -eq $phase })
  if ($r.Count -eq 0) { continue }
  Write-Output ("{0}: samples {1} | rss_mb mean {2} max {3} | cpu_pct mean {4} max {5} | vram_mb max {6} | vram_shared_mb max {7}" -f $phase, $r.Count,
    [math]::Round(($r.rss_mb | Measure-Object -Average).Average, 1), ($r.rss_mb | Measure-Object -Maximum).Maximum,
    [math]::Round(($r.cpu_pct | Measure-Object -Average).Average, 1), ($r.cpu_pct | Measure-Object -Maximum).Maximum,
    ($r.vram_mb | Measure-Object -Maximum).Maximum, ($r.vram_shared_mb | Measure-Object -Maximum).Maximum)
}
Write-Output ("last title: {0}" -f $rows[-1].title)
if ($Csv -ne "") { $rows | Export-Csv -NoTypeInformation -Path $Csv }
if (-not $KeepRunning -and $AttachPid -eq 0) { Stop-Process -Id (Get-Tree $root) -Force -ErrorAction SilentlyContinue }
