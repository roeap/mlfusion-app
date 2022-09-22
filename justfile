set shell := ["powershell.exe", "-c"]

generate:
    buf generate

run:
    yarn tauri dev
