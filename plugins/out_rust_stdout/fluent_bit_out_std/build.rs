// https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib
// https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program
// https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
// How the linker works:
// https://eli.thegreenplace.net/2013/07/09/library-order-in-static-linking/

fn main() {
    // specify static libraries to link
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    // TODO: If we get to decide the distribution channel, might want to investigate
    // whether we want to use the shared library version of fluent-bit library
    // (i.e. https://github.com/fluent/fluent-bit/blob/master/CMakeLists.txt#L58)
    // instead of the static library to reduce Rust plugin memory footprint (only
    // in the case of multi-process I guess)
    println!("cargo:rustc-link-lib=static=fluent-bit"); // libfluent-bit.a
    println!("cargo:rustc-link-lib=static=mk_core"); // libmk_core.a
    println!("cargo:rustc-link-lib=static=co"); // libco.a
    println!("cargo:rustc-link-lib=static=mpack-static"); // libmpack-static.a
    println!("cargo:rustc-link-lib=static=msgpackc"); // libmsgpackc.a
    println!("cargo:rustc-link-lib=static=jsmn"); // libjsmn.a
    println!("cargo:rustc-link-lib=static=chunkio-static"); // libchunkio-static.a
    println!("cargo:rustc-link-lib=static=onigmo"); // libonigmo.a
    println!("cargo:rustc-link-lib=static=cio-crc32"); //libcio-crc32.a

    // not sure whether these are needed. Might as wll write a for loop
    // that looks for all the .a files under /usr/local/lib and link
    // all of them?
    println!("cargo:rustc-link-lib=static=tutf8e"); // libtutf8e.a
    println!("cargo:rust-link-lib=static=miniz"); // libminiz.a
    println!("cargo:rust-link-lib=static=rbtree"); // librbtree.a
}
