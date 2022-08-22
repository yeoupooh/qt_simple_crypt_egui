# Introduction

- An example of qt_simple_crypt with egui.

# Prepare

```
git clone https://github.com/yeoupooh/qt_simple_crypt.git
git clone https://github.com/yeoupooh/qt_simple_crypt_egui.git
```

# Native app

## Run

```
cd qt_simple_crypt_egui
cargo run
```

# WASM app (Windows)

## Prerequisites
- Chocolatey


## Prepare

```
choco install jq
cargo install wasm-opt
setup_web.bat
build_web.bat
```

## Run
```
start_server.bat
```
- Check on http://127.0.0.1:8080
