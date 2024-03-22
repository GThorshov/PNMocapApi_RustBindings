use bindgen;

fn main() {

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .generate()
        .expect("Unable to generate C bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("mocap_api_rs.rs"))
        .expect("Couldn't write bindings!");

}