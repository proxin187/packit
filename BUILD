

dependencies() {
    echo ""
}

config() {
    mkdir ~/.packit ~/.packit/repo
    touch ~/.packit/conf.toml
}

build() {
    cargo build --release
    mv target/release/packit /usr/bin/packit
}

