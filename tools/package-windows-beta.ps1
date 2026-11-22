param(
    [string]$Supplements = 'target/notice-supplements',
    [string]$Output
)

$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repo
try {
    $config = Get-Content -LiteralPath 'apps/desktop/src-tauri/tauri.conf.json' -Raw | ConvertFrom-Json
    $version = $config.version
    $isPrerelease = $version.Contains('-')
    if (!$Output) {
        $Output = if ($isPrerelease) { 'target/beta' } else { 'target/distribution' }
    }
    $archiveSuffix = if ($isPrerelease) { 'beta-x64' } else { 'windows-x64' }
    $installerName = "$($config.productName)_${version}_x64-setup.exe"
    $installer = Get-Item -LiteralPath (Join-Path 'target/release/bundle/nsis' $installerName)
    $installerMetadata = $installer.VersionInfo
    if ($installerMetadata.ProductVersion -cne $version -or
        $installerMetadata.ProductName -cne $config.productName) {
        throw "O instalador tem produto ou versão divergente da configuração do Tauri: $($installerMetadata.ProductName) $($installerMetadata.ProductVersion)."
    }
    $package = Get-Content -LiteralPath 'package.json' -Raw | ConvertFrom-Json
    $packageLock = Get-Content -LiteralPath 'package-lock.json' -Raw | ConvertFrom-Json -AsHashtable
    $metadataJson = cargo metadata --offline --locked --no-deps --format-version 1
    if ($LASTEXITCODE) { throw 'Falha ao consultar a versão do workspace Rust' }
    $metadata = $metadataJson | ConvertFrom-Json
    $desktop = $metadata.packages | Where-Object name -CEQ 'cm0304-desktop'
    $versions = [ordered]@{
        'package.json' = $package.version
        'package-lock.json' = $packageLock.version
        'package-lock.json (pacote raiz)' = $packageLock.packages[''].version
        'cm0304-desktop' = $desktop.version
    }
    foreach ($entry in $versions.GetEnumerator()) {
        if ($entry.Value -cne $version) {
            throw "Versão divergente em $($entry.Key). Esperada $version, encontrada $($entry.Value)."
        }
    }
    $installerHash = (Get-FileHash -LiteralPath $installer.FullName).Hash
    $outputDir = New-Item -ItemType Directory -Path $Output -Force
    $archivePath = Join-Path $outputDir.FullName "CM0304-Save-Editor-$version-$archiveSuffix.zip"
    $checksumPath = "$archivePath.sha256"
    foreach ($path in @($archivePath, $checksumPath)) {
        if (Test-Path -LiteralPath $path) { throw "O arquivo já existe em $path" }
    }

    if ($isPrerelease) {
        $documentation = Get-Content -LiteralPath 'docs/DEVELOPMENT.pt-BR.md' -Raw
        $guide = [regex]::Match($documentation, '(?ms)^### Beta em outra carreira\r?\n.+?(?=^### Evidências locais do beta)').Value
        if (!$guide) { throw 'Roteiro do beta incompleto' }
        $guideName = "ROTEIRO-BETA-$version.md"
        $title = "Beta do CM0304 Save Editor $version"
    } else {
        $guideName = "GUIDE-$version.md"
        $title = "CM0304 Save Editor $version"
        $guide = @(
            'Run the Windows x64 installer. The editor uses the Microsoft Edge WebView2 Runtime.'
            ''
            'Open an uncompressed CM0304 4.1.5 save, review your changes, and select Save validated copy.'
            'Load the copy in the game. To return to your original career, load the source save.'
        )
    }
    $signature = (Get-AuthenticodeSignature -LiteralPath $installer.FullName).Status
    $guideHeader = @(
        "# $title"
        ''
        "Installer SHA-256: $installerHash"
        "Authenticode status: $signature"
        ''
        'To verify the installer, open PowerShell in the extracted folder and run:'
        ''
        '```powershell'
        ('$actual = (Get-FileHash -LiteralPath ''{0}'' -Algorithm SHA256).Hash' -f $installerName.Replace("'", "''"))
        ('if ($actual -ne ''{0}'') {{ throw ''The installer SHA-256 does not match.'' }}' -f $installerHash)
        '```'
        ''
    )
    $notices = node tools/windows-dependency-licenses.js --release-notices $Supplements
    if ($LASTEXITCODE) { throw 'Falha ao gerar os avisos de terceiros' }

    $staging = New-Item -ItemType Directory -Path (Join-Path $outputDir.FullName ([guid]::NewGuid()))
    $guidePath = Join-Path $staging.FullName $guideName
    $noticePath = Join-Path $staging.FullName "THIRD-PARTY-NOTICES-$version.md"
    $guideHeader + $guide | Set-Content -Encoding utf8 -LiteralPath $guidePath
    $notices | Set-Content -Encoding utf8 -LiteralPath $noticePath
    $files = @($installer.FullName, $noticePath, $guidePath, (Resolve-Path -LiteralPath 'LICENSE').Path)
    $temporaryZip = Join-Path $staging.FullName 'package.zip'
    Compress-Archive -LiteralPath $files -DestinationPath $temporaryZip

    $archive = [IO.Compression.ZipFile]::OpenRead($temporaryZip)
    try {
        if ($archive.Entries.Count -ne $files.Count) { throw 'Conteúdo inesperado no pacote' }
        foreach ($file in $files) {
            $entry = $archive.GetEntry((Split-Path -Leaf $file))
            if (!$entry) { throw "Arquivo ausente no pacote: $file" }
            $stream = $entry.Open()
            try {
                $actual = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($stream))
                if ($actual -ne (Get-FileHash -LiteralPath $file).Hash) {
                    throw "Arquivo divergente no pacote: $file"
                }
            } finally { $stream.Dispose() }
        }
    } finally { $archive.Dispose() }
    $archiveHash = Get-FileHash -LiteralPath $temporaryZip -Algorithm SHA256
    $temporaryChecksum = Join-Path $staging.FullName 'package.sha256'
    "$(($archiveHash.Hash).ToLowerInvariant())  $(Split-Path -Leaf $archivePath)" |
        Set-Content -LiteralPath $temporaryChecksum -Encoding ascii
    [IO.File]::Move($temporaryChecksum, $checksumPath)
    try {
        [IO.File]::Move($temporaryZip, $archivePath)
    } catch {
        [IO.File]::Delete($checksumPath)
        throw
    }
    Get-FileHash -LiteralPath $archivePath
} finally {
    Pop-Location
}
