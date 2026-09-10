<# SP-8 -- M2: bytes and files of an installed folder, or the size of an installer. Usage: size.ps1 -Path <folder or file> #>
param([Parameter(Mandatory = $true)][string]$Path)
$item = Get-Item -LiteralPath $Path
if ($item.PSIsContainer) {
  $files = @(Get-ChildItem -LiteralPath $Path -Recurse -File -Force)
  $sum = ($files | Measure-Object Length -Sum).Sum
  Write-Output ("{0}: {1} files, {2} bytes, {3} MB" -f $Path, $files.Count, $sum, [math]::Round($sum / 1MB, 1))
} else {
  Write-Output ("{0}: {1} bytes, {2} MB" -f $Path, $item.Length, [math]::Round($item.Length / 1MB, 1))
}
