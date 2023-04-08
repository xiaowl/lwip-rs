use std::{path, env};

extern crate bindgen;
extern crate cc;

fn compile_lwip_library() {
    println!("cargo:rerun-if-changed=src/lwip");
    println!("cargo:rerun-if-changed=src/lwip-opts");
    let mut build = cc::Build::new();
    // files from lwip/src/Filelists.cmake
    build
        .file("src/lwip/src/core/init.c")
        .file("src/lwip/src/core/def.c")
        .file("src/lwip/src/core/dns.c")
        .file("src/lwip/src/core/inet_chksum.c")
        .file("src/lwip/src/core/ip.c")
        .file("src/lwip/src/core/mem.c")
        .file("src/lwip/src/core/memp.c")
        .file("src/lwip/src/core/netif.c")
        .file("src/lwip/src/core/pbuf.c")
        .file("src/lwip/src/core/raw.c")
        .file("src/lwip/src/core/stats.c")
        .file("src/lwip/src/core/sys.c")
        .file("src/lwip/src/core/altcp.c")
        .file("src/lwip/src/core/altcp_alloc.c")
        .file("src/lwip/src/core/altcp_tcp.c")
        .file("src/lwip/src/core/tcp.c")
        .file("src/lwip/src/core/tcp_in.c")
        .file("src/lwip/src/core/tcp_out.c")
        .file("src/lwip/src/core/timeouts.c")
        .file("src/lwip/src/core/ipv4/autoip.c")
        .file("src/lwip/src/core/ipv4/dhcp.c")
        .file("src/lwip/src/core/ipv4/etharp.c")
        .file("src/lwip/src/core/ipv4/icmp.c")
        .file("src/lwip/src/core/ipv4/igmp.c")
        .file("src/lwip/src/core/ipv4/ip4_frag.c")
        .file("src/lwip/src/core/ipv4/ip4.c")
        .file("src/lwip/src/core/ipv4/ip4_addr.c")
        .file("src/lwip/src/core/ipv6/dhcp6.c")
        .file("src/lwip/src/core/ipv6/ethip6.c")
        .file("src/lwip/src/core/ipv6/icmp6.c")
        .file("src/lwip/src/core/ipv6/inet6.c")
        .file("src/lwip/src/core/ipv6/ip6.c")
        .file("src/lwip/src/core/ipv6/ip6_addr.c")
        .file("src/lwip/src/core/ipv6/ip6_frag.c")
        .file("src/lwip/src/core/ipv6/mld6.c")
        .file("src/lwip/src/core/ipv6/nd6.c")
        .file("src/lwip-opts/platform.c");
    build.include("src/lwip/src/include");
    build.include("src/lwip-opts");
    build.flag(&env::var("LWIP_FLAGS").unwrap_or_default());
    build.debug(false);
    build.compile("liblwip.a");
}

fn generate_lwip_bindings() {
    println!("cargo:rustc-link-lib=lwip");
    println!("cargo:rerun-if-changed=src/lwip");
    println!("cargo:rerun-if-changed=src/lwip-opts");
    let builder = bindgen::Builder::default()
        .header("src/lwip/src/include/lwip/init.h")
        .header("src/lwip/src/include/lwip/tcpip.h")
        .header("src/lwip/src/include/lwip/tcp.h")
        .header("src/lwip/src/include/lwip/udp.h")
        .header("src/lwip/src/include/lwip/netif.h")
        .header("src/lwip/src/include/lwip/ip_addr.h")
        .clang_arg("-Isrc/lwip/src/include")
        .clang_arg("-Isrc/lwip-opts")
        .clang_arg(&env::var("LWIP_FLAGS").unwrap_or_default())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));
    let mut out_path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_path.push("src");
    out_path.push("bindings.rs");
    println!("cargo:rerun-if-changed={}", out_path.to_str().unwrap());
    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}

fn main() {
    generate_lwip_bindings();
    compile_lwip_library();
}
