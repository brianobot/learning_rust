#![allow(unused)]

fn main() {
    // to add condition compilation to your project
    // annotate the source code with cfg attributes,
    // cfg works with the target parameter provided to rustc during compilation

    #[cfg(target_os = "windows")]
    const USAGE: &str = "
        akv_mem.exe FILE get KEY
        akv_mem.exe FILE delete KEY
        akv_mem.exe FILE insert KEY VALUE
        akv_mem.exe FILE update KEY VALUE
    ";

    #[cfg(not(target_os = "windows"))]
    const USAGE: &str = "
        akv_mem FILE get KEY
        akv_mem FILE delete KEY
        akv_mem FILE insert KEY VALUE
        akv_mem FILE update KEY VALUE
    ";
}
