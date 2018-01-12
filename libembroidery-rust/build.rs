extern crate binder;
extern crate cheddar;
use std::env;
use std::path;
use std::io::{Read, Write};

use binder::compiler::Compiler;
use std::fs::File;

fn c89_header() -> cheddar::Header {
    cheddar::Header::with_standard(cheddar::Standard::C89)
        .insert_code("#include <stdlib.h>\n")
        .insert_code("#include <stdio.h>\n")
        // Allow us to use `uint8_t` and the like.
        .insert_code("#include <inttypes.h>\n")
        .insert_code("#include \"emb-file.h\"\n")
}

fn map_our_modules(ty: &str) -> String {
    if ty.starts_with("Emb") {
        return format!("{}", ty);
    } else {
        ty.to_owned()
    }
}

fn main() {
    let modules = vec![
        "helpers",
        "helpers::binary",
        "helpers::hash",
        "helpers::misc",
        "helpers::time",
        "pattern",
        "pattern::utils",
        "pattern::utils::color",
        "pattern::utils::hoop",
        "pattern::utils::flag",
        "pattern::utils::vector",
        "pattern::utils::thread",
        "pattern::line",
        "pattern::point",
        "pattern::settings",
        "pattern::stitch",
        "pattern::arc",
        "pattern::circle",
        "pattern::ellipse",
        "pattern::path",
        "pattern::polygon",
        "pattern::polyline",
        "pattern::rect",
        "pattern::spline",
        "pattern::pattern",
        "formats",
        // "formats::reader_writer",
        // "pattern::",
        // "pattern::",
        // "pattern::",
        // "pattern::",
    ];
    // fs::create_dir_all(dir.clone());

    let dir = path::PathBuf::from("./target")
        .join(env::var("PROFILE").unwrap())
        .join("includes");
    let compiled_file = dir.join(c89_header().language());
    let mut h_paths: Vec<path::PathBuf> = Vec::new();

    for (idx, module) in modules.iter().enumerate() {
        let h_name = format!("_{}.h", module.replace("::", "_"));
        h_paths.push(compiled_file.join(&h_name));
        let mut c89 = c89_header().name(&h_name);
        for search_module in modules.iter().take(idx) {
            c89 = c89.register_custom_module(search_module, map_our_modules);
        }
        binder::Binder::new()
            .expect("Failed to create binder")
            .output_directory(dir.to_str().unwrap())
            .module(module)
            .unwrap()
            .register(c89)
            .run_build();
    }
    let final_h_path = compiled_file.join("pattern.h");
    let mut tmp = Vec::with_capacity(4048);
    let mut final_h_file = File::create(&final_h_path).unwrap();
    for h_path in h_paths {
        tmp.truncate(0);
        File::open(&h_path).unwrap().read_to_end(&mut tmp).unwrap();
        final_h_file.write_all(&tmp).unwrap();
    }
    final_h_file.flush().unwrap();
}
