TARGET_LINUX64 := "x86_64-unknown-linux-gnu"
TARGET_WIN64 := "x86_64-pc-windows-gnu"

release:
    cargo build --release --target {{TARGET_LINUX64}}

    cargo build --release --target {{TARGET_WIN64}}

    rm -rf build
    mkdir build

    zip build/{{TARGET_LINUX64}}.zip  target/{{TARGET_LINUX64}}/release/bpdl    LICENSE README.md
    zip build/{{TARGET_WIN64}}.zip    target/{{TARGET_WIN64}}/release/bpdl.exe  LICENSE README.md

    du -h target/{{TARGET_LINUX64}}/release/bpdl target/{{TARGET_WIN64}}/release/bpdl.exe
