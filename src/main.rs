use std::fs::create_dir_all;

use camino::Utf8PathBuf;
use clap::Parser as _;
use image::{DynamicImage, GenericImageView};

#[derive(Debug, clap::Parser)]
struct Cli {
    profile_picture_path: Utf8PathBuf,
    #[clap(long = "out", short)]
    out_dir: Option<Utf8PathBuf>,
    icon_paths: Vec<Utf8PathBuf>,
}

fn main() {
    env_logger::init();

    let Cli {
        profile_picture_path,
        out_dir,
        icon_paths,
    } = Cli::parse();

    let out_dir = out_dir.unwrap_or_else(|| profile_picture_path.with_extension(""));
    create_dir_all(&out_dir).unwrap();

    let profile_picture = image::open(&profile_picture_path).unwrap();

    for icon_path in icon_paths {
        let icon = image::open(&icon_path).unwrap();
        let iconified_profile_picture = {
            let mut pfp = profile_picture.clone();
            overlay_icon_over_pfp(&mut pfp, &icon);
            pfp
        };

        let iconified_pfp_file_path = out_dir.join(format!(
            "{}_{}.png",
            profile_picture_path.file_stem().unwrap(),
            icon_path.file_stem().unwrap(),
        ));
        iconified_profile_picture
            .save(iconified_pfp_file_path)
            .unwrap();
    }
}

fn overlay_icon_over_pfp(profile_picture: &mut DynamicImage, icon: &DynamicImage) {
    use image::imageops::{overlay, overlay_bounds, FilterType};

    // Divide the profile picture into an evenly spaced 3x3 grid. We want the icon to occupy the
    // bottom-right nonant of the profile picture.
    let x = 2 * profile_picture.width() / 3;
    let y = 2 * profile_picture.height() / 3;
    let rescaled_icon = {
        let (x, y) = overlay_bounds(profile_picture.dimensions(), icon.dimensions(), x, y);
        icon.resize(x, y, FilterType::Lanczos3)
    };

    overlay(profile_picture, &rescaled_icon, x.into(), y.into())
}
