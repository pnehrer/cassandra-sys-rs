fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }

    #[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "musl"))]
    {
        println!("cargo:rustc-link-lib=static=cassandra");
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");
        println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static=uv");
        println!("cargo:rustc-link-search={}", "/usr/local/musl/lib");
    }

    // println!("cargo:rustc-flags=-l dylib=cassandra");
    // println!("cargo:rustc-flags=-l dylib=crypto");
    // println!("cargo:rustc-flags=-l dylib=ssl");
    // #[cfg(target_os = "macos")]
    // println!("cargo:rustc-flags=-l dylib=c++");
    // #[cfg(not(target_os = "macos"))]
    // println!("cargo:rustc-flags=-l dylib=stdc++");
    // println!("cargo:rustc-flags=-l dylib=uv");
    // println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-search={}", "/usr/local/lib/x86_64-linux-gnu");
    // println!("cargo:rustc-link-search={}", "/usr/local/lib64");
    // println!("cargo:rustc-link-search={}", "/usr/local/lib");
    // println!("cargo:rustc-link-search={}", "/usr/lib64/");
    // println!("cargo:rustc-link-search={}", "/usr/lib/");
    // println!("cargo:rustc-link-search={}", "/usr/local/opt/openssl/lib");
}
