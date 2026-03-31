use std::{
    env, fs,
    io::Cursor,
    path::{Path, PathBuf},
};

const STUDIO_UI_URL: &str = "https://txtx-public.s3.amazonaws.com/surfpool-studio-ui/latest.zip";
const STUDIO_UI_DIR_ENV: &str = "SURFPOOL_STUDIO_UI_DIR";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={STUDIO_UI_DIR_ENV}");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let asset_dir = out_dir.join("surfpool-studio-ui");

    println!("cargo:warning=------------ Studio Build Script ------------");

    if assets_present(&asset_dir) {
        println!(
            "cargo:warning=Studio assets already found at {}",
            asset_dir.display()
        );
        return Ok(());
    }

    if let Ok(local_dir) = env::var(STUDIO_UI_DIR_ENV) {
        println!(
            "cargo:warning=Copying Surfpool Studio UI assets from {}",
            local_dir
        );
        copy_dir_all(Path::new(&local_dir), &asset_dir)?;
        return Ok(());
    }

    println!(
        "cargo:warning=Extracting Surfpool Studio UI assets to {}",
        asset_dir.display()
    );
    download_and_extract(&asset_dir)?;
    Ok(())
}

fn assets_present(asset_dir: &Path) -> bool {
    asset_dir.join("_next").exists()
}

fn download_and_extract(asset_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(STUDIO_UI_URL)?.error_for_status()?;
    extract_zip(resp.bytes()?.to_vec(), asset_dir)?;
    Ok(())
}

fn extract_zip(bytes: Vec<u8>, asset_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(asset_dir)?;
    let reader = Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(reader)?;
    zip.extract(asset_dir)?;
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(from, to)?;
        }
    }
    Ok(())
}
