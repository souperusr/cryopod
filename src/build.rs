fn main() {
    // Export image
    let cryopod_image = std::env::var("CRYOPOD_IMAGE")
        .expect("CRYOPOD_IMAGE must be set in environment during build");

    let cryopod_image_digest = std::env::var("CRYOPOD_IMAGE_DIGEST")
    .expect("CRYOPOD_IMAGE_DIGEST must be set in environment during build");

    println!("cargo:rustc-env=CRYOPOD_IMAGE={}", cryopod_image);
    println!("cargo:rustc-env=CRYOPOD_IMAGE_DIGEST={}", cryopod_image_digest);
}
