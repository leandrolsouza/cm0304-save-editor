$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $root
try {
    $tauri = Get-Content -LiteralPath 'apps/desktop/src-tauri/tauri.conf.json' -Raw | ConvertFrom-Json
    $version = $tauri.version
    $product = $tauri.productName
    $tag = "v$version"
    $repository = $env:GITHUB_REPOSITORY
    if ($version -cnotmatch '^\d+\.\d+\.\d+$' -or $env:GITHUB_REF_NAME -cne $tag) {
        throw "A release estável precisa usar a tag $tag, correspondente à versão do aplicativo."
    }
    if ($repository -cnotmatch '^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$' -or
        !$env:TAURI_UPDATER_PUBLIC_KEY -or !$env:TAURI_SIGNING_PRIVATE_KEY -or !$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD) {
        throw 'Configure o repositório, a chave pública, a chave privada e a senha de assinatura do atualizador.'
    }

    if (Test-Path -LiteralPath 'target/release-assets') { throw 'A pasta target/release-assets já existe.' }
    $staging = New-Item -ItemType Directory -Path ('target/release-staging-' + [guid]::NewGuid())
    $configPath = Join-Path $staging.FullName 'tauri-release.json'
    $releaseBase = "https://github.com/$repository/releases"
    @{
        bundle = @{ createUpdaterArtifacts = $true }
        plugins = @{ updater = @{
            pubkey = $env:TAURI_UPDATER_PUBLIC_KEY.Trim()
            endpoints = @("$releaseBase/latest/download/latest.json")
            windows = @{ installMode = 'passive' }
        } }
    } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $configPath -Encoding utf8

    npm run build:ui
    if ($LASTEXITCODE -ne 0) { throw 'Falha na compilação da interface.' }
    Push-Location -LiteralPath 'apps/desktop'
    try {
        node ../../node_modules/@tauri-apps/cli/tauri.js build --bundles nsis --ci --config $configPath -- --locked --offline
        if ($LASTEXITCODE -ne 0) { throw 'Falha na compilação do instalador assinado para o atualizador.' }
    } finally {
        Pop-Location
    }

    $packageDir = Join-Path $staging.FullName 'package'
    & ./tools/package-windows-beta.ps1 -Supplements 'tools/notice-supplements' -Output $packageDir

    $assets = New-Item -ItemType Directory -Path (Join-Path $staging.FullName 'assets')
    $zipName = "CM0304-Save-Editor-$version-windows-x64.zip"
    foreach ($name in $zipName, "$zipName.sha256") {
        Copy-Item -LiteralPath (Join-Path $packageDir $name) -Destination $assets.FullName
    }

    $installerName = "CM0304-Save-Editor-$version-windows-x64-setup.exe"
    $bundle = "target/release/bundle/nsis/${product}_${version}_x64-setup.exe"
    $signature = (Get-Content -LiteralPath "$bundle.sig" -Raw).Trim()
    if (!$signature) { throw 'O Tauri gerou uma assinatura vazia.' }
    Copy-Item -LiteralPath $bundle -Destination (Join-Path $assets.FullName $installerName)
    Copy-Item -LiteralPath "$bundle.sig" -Destination (Join-Path $assets.FullName "$installerName.sig")
    @{
        version = $version
        platforms = @{ 'windows-x86_64' = @{
            signature = $signature
            url = "$releaseBase/download/$tag/$installerName"
        } }
    } | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $assets.FullName 'latest.json') -Encoding utf8
    [IO.Directory]::Move($assets.FullName, (Join-Path $root 'target/release-assets'))
} finally {
    Pop-Location
}
