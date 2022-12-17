set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

generate:
    buf generate

run:
    yarn tauri dev
