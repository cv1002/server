#!/usr/bin/env sh

clean() {
    rm -rf .VSCodeCounter
    rm -rf build
    rm -rf target*
    rm -rf log
}

build() {
    rm -rf    build
    mkdir     build
    cargo     build                     --release
    cp        target/release/service build
    cp -r     config                 build
    chmod     777                    build/service

    # Printout the size of output binary file.
    fileSize=$(du -kh build/service | cut -f1)
    printf "total size: %s\n" $fileSize
}

debugbuild() {
    rm -rf    build
    mkdir     build
    cargo     build
    cp        target/debug/service build
    cp -r     config               build
    chmod     777                  build/service

    # Printout the size of output binary file.
    fileSize=$(du -kh build/service | cut -f1)
    printf "total size: %s\n" $fileSize
}

staticbuild() {
    nightly_image='registry.gitlab.com/rust_musl_docker/image:nightly-2023-10-28'

    rm -rf    build
    mkdir     build
    docker run -it --rm  \
        -v $PWD:/workdir \
        -v ~/.cargo/git:/root/.cargo/git           \
        -v ~/.cargo/registry:/root/.cargo/registry \
        $nightly_image \
        cargo build --release -vv --target=x86_64-unknown-linux-musl
    cp        target/x86_64-unknown-linux-musl/release/service build
    cp -r     config                                           build
    chmod     777 build/service

    # Printout the size of output binary file.
    fileSize=$(du -kh build/service | cut -f1)
    printf "total size: %s\n" $fileSize
}

debugreleasebuild() {
    RUSTFLAGS=-g build
}

test() {
    cargo test
    cargo test --doc
}

help() {
    echo "Usage: ./commands.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "    clean              清理编译产物"
    echo "    build              发布模式编译"
    echo "    debugbuild         调试模式编译"
    echo "    staticbuild        发布模式，但静态链接"
    echo "    debugreleasebuild  发布模式，但带调试信息"
    echo "    test               执行单元测试和文档测试"
    echo "    cargodoc           构建CargoDocuments文档"
    echo "    help               显示帮助信息"
}

set -e

case $1 in
    "clean")
        clean
        ;;
    "build")
        build
        ;;
    "debugbuild")
        debugbuild
        ;;
    "staticbuild")
        staticbuild
        ;;
    "debugreleasebuild")
        debugreleasebuild
        ;;
    "packdocker")
        packdocker $2
        ;;
    "test")
        test
        ;;
    "cargodoc")
        cargo doc $2
        ;;
    "help" | *)
        help
        ;;
esac
