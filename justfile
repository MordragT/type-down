#!/usr/bin/env -S nix shell nixpkgs#nushell nixpkgs#just --command 'just --justfile'

install-zed-ext:
    #!/usr/bin/env nu
    const ext_dir = [ $nu.home-path .local share zed extensions installed tyd-zed-extension] | path join

    cargo build -p tyd-zed-extension --target=wasm32-wasip1 --release

    let temp_dir = mktemp -d

    print $temp_dir

    let adaptor = $temp_dir | path join wasi_snapshot_preview1.wasm
    let ext = $temp_dir | path join extension.wasm

    http get "https://github.com/bytecodealliance/wasmtime/releases/download/v31.0.0/wasi_snapshot_preview1.reactor.wasm" | save $adaptor

    (wasm-tools component new target/wasm32-wasip1/release/tyd_zed_extension.wasm
        --adapt $"wasi_snapshot_preview1=($adaptor)"
        --output $ext
    )

    wasm-tools validate $ext

    print "Copying built extension"

    if ($ext_dir | path exists) { rm -r $ext_dir }

    mkdir $ext_dir

    cp -r crates/tyd-zed-extension/languages ($ext_dir | path join languages)
    cp crates/tyd-zed-extension/extension.toml ($ext_dir | path join extension.toml)
    cp $ext ($ext_dir | path join extension.wasm)

    rm -r $temp_dir
