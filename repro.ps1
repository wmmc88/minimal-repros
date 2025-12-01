# PowerShell reproduction script

param(
    [string]$CargoToolchain = "nightly-2025-11-21",
    [string[]]$CargoCommand = @("check")
)

$env:CARGO_TARGET_DIR = "$PWD/shared-target"
Write-Host "Set CARGO_TARGET_DIR to $env:CARGO_TARGET_DIR"

Write-Host "Cleaning CARGO_TARGET_DIR..."
if (Test-Path $env:CARGO_TARGET_DIR) {
    Remove-Item -Path $env:CARGO_TARGET_DIR -Recurse -Force
}


Write-Host "Starting builds..."
Write-Host "Using cargo toolchain override: $CargoToolchain"
Write-Host ("Cargo command: cargo {0}" -f ((@($CargoCommand) | Where-Object { $_ -ne $null }) -join ' '))

$workspaceRoot = (Get-Location).Path

function Start-CargoJob {
    param(
        [string]$ManifestPath,
        [string]$Label
    )

    Start-Job -ArgumentList @($ManifestPath, $Label) -ScriptBlock {
        param($manifestPath, $label)

        Set-Location $using:workspaceRoot

        $arguments = @()
        if ($using:CargoToolchain) {
            $arguments += "+$using:CargoToolchain"
        }

        $commandParts = $using:CargoCommand
        if (-not $commandParts) {
            $commandParts = @("check")
        }
        elseif ($commandParts -is [string]) {
            $commandParts = @($commandParts)
        }

        $arguments += $commandParts

        if ($manifestPath) {
            $arguments += "--manifest-path", $manifestPath
        }

        $commandLine = $arguments -join ' '
        [pscustomobject]@{
            Label   = $label
            Kind    = 'Info'
            Message = "Running: cargo $commandLine"
        }

        $output = & cargo @arguments 2>&1
        $exitCode = $LASTEXITCODE

        foreach ($line in $output) {
            [pscustomobject]@{
                Label   = $label
                Kind    = 'Line'
                Message = $line
            }
        }

        [pscustomobject]@{
            Label    = $label
            Kind     = 'ExitCode'
            ExitCode = $exitCode
        }
    }
}

$job1 = Start-CargoJob -ManifestPath (Join-Path $workspaceRoot "package-a/Cargo.toml") -Label "P1"
$job2 = Start-CargoJob -ManifestPath (Join-Path $workspaceRoot "package-b/Cargo.toml") -Label "P2"

$jobs = @($job1, $job2)

Wait-Job -Job $jobs | Out-Null

$exitCodes = @{}

foreach ($job in $jobs) {
    $messages = Receive-Job -Job $job -Keep
    foreach ($record in $messages) {
        if ($record -is [string]) {
            Write-Host $record
            continue
        }

        $kindProperty = $record.PSObject.Properties['Kind']
        $labelProperty = $record.PSObject.Properties['Label']

        if (-not $kindProperty -or -not $labelProperty) {
            Write-Host $record
            continue
        }

        $label = $labelProperty.Value

        switch ($kindProperty.Value) {
            'Info' {
                $message = $record.PSObject.Properties['Message'].Value
                Write-Host ("[{0}] {1}" -f $label, $message)
            }
            'Line' {
                $message = $record.PSObject.Properties['Message'].Value
                Write-Host ("[{0}] {1}" -f $label, $message)
            }
            'ExitCode' {
                $exitCode = $record.PSObject.Properties['ExitCode'].Value
                Write-Host ("[{0}] Exit code: {1}" -f $label, $exitCode)
                $exitCodes[$label] = [int]$exitCode
            }
            Default {
                Write-Host $record
            }
        }
    }
}

$jobs | Remove-Job

$exitP1 = if ($exitCodes.ContainsKey('P1')) { $exitCodes['P1'] } else { 'n/a' }
$exitP2 = if ($exitCodes.ContainsKey('P2')) { $exitCodes['P2'] } else { 'n/a' }

Write-Host ("Builds finished. Exit codes -> P1: {0}, P2: {1}" -f $exitP1, $exitP2)
