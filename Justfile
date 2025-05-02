alias b := build
alias r := run

# RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" \

build:
    cargo -Z build-std=std \
    build --release
    # -Z build-std-features="optimize_for_size" \


    du -sh target/release/bpdl
    # echo "Warning! Whilst built binary is small, it's undebuggable! Use debug builds using cargo build"

run:
    target/release/bpdl
