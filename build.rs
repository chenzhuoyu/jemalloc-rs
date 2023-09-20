#![feature(exit_status_error)]

use std::{
    io::Result as IoResult,
    path::{Path, PathBuf},
    process::Command,
};

use bindgen::CargoCallbacks;

fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> IoResult<()> {
    std::fs::create_dir_all(&dest)?;
    for item in std::fs::read_dir(src)? {
        let de = item?;
        let ty = de.file_type()?;
        if ty.is_dir() {
            copy_dir(de.path(), dest.as_ref().join(de.file_name()))?;
        } else {
            std::fs::copy(de.path(), dest.as_ref().join(de.file_name()))?;
        }
    }
    Ok(())
}

fn debug_opts() -> &'static str {
    if std::env::var("DEBUG").unwrap() == "true" {
        "--enable-debug"
    } else {
        "--disable-debug"
    }
}

pub fn main() {
    let cur_dir = std::env::current_dir().unwrap();
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let num_jobs = std::env::var("NUM_JOBS").unwrap();
    let source_dir = out_dir.join("jemalloc");
    let prefix_dir = out_dir.join("jemalloc_build");

    /* clear existing builds */
    std::fs::remove_dir_all(&source_dir).ok();
    std::fs::remove_dir_all(&prefix_dir).ok();

    /* copy jemalloc source to avoid poisoning the source treee */
    copy_dir(cur_dir.join("jemalloc"), &source_dir).expect("cannot copy jemalloc source");
    println!("cargo:rerun-if-changed=jemalloc");

    /* run `configure` */
    Command::new("sh")
        .current_dir(&source_dir)
        .arg(source_dir.join("autogen.sh"))
        .arg("--prefix")
        .arg(&prefix_dir)
        .arg("--disable-doc")
        .arg("--disable-cxx")
        .arg("--enable-static")
        .arg("--disable-shared")
        .arg("--with-jemalloc-prefix=rog_")
        .arg("--with-private-namespace=_rog_priv_")
        .arg(debug_opts())
        .spawn()
        .expect("cannot start `configure`")
        .wait()
        .expect("cannot execute `configure`")
        .exit_ok()
        .expect("error when executing `configure`");

    /* build */
    Command::new("make")
        .current_dir(&source_dir)
        .arg("-j")
        .arg(&num_jobs)
        .spawn()
        .expect("cannot start `make`")
        .wait()
        .expect("cannot execute `make`")
        .exit_ok()
        .expect("error when executing `make`");

    /* install to prefix */
    Command::new("make")
        .current_dir(&source_dir)
        .arg("install_lib_static")
        .arg("install_include")
        .arg("-j")
        .arg(num_jobs)
        .spawn()
        .expect("cannot start `make`")
        .wait()
        .expect("cannot execute `make`")
        .exit_ok()
        .expect("error when executing `make`");

    /* generate bindings */
    bindgen::builder()
        .header(
            prefix_dir
                .join("include")
                .join("jemalloc")
                .join("jemalloc.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_item("MALLCTL_.*")
        .allowlist_item("MALLOCX_.*")
        .allowlist_item("JEMALLOC_.*")
        .allowlist_type("extent_hooks_t")
        .allowlist_function("rog_.*")
        .ctypes_prefix("libc")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("cannot generate bindings for jemalloc")
        .write_to_file(cur_dir.join("src").join("bindings.rs"))
        .expect("cannot save generated bindings");

    /* link against jemalloc, with the PIC version */
    let lib_dir = prefix_dir.join("lib");
    println!("cargo:rustc-link-lib=static=jemalloc_pic");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}
