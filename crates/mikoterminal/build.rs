fn main() {
    // Link Windows registry library when building as DLL
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
