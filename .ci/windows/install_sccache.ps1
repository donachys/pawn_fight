# https://github.com/lukesampson/scoop
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
Invoke-Expression (New-Object System.Net.WebClient).DownloadString('https://get.scoop.sh')

Join-Path (Resolve-Path ~).Path "scoop\shims" >> $Env:GITHUB_PATH

scoop update
scoop install sccache
