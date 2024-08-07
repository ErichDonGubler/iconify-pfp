use std::fs::create_dir_all;

use camino::Utf8PathBuf;
use clap::Parser as _;
use image::{imageops::FilterType, DynamicImage, GenericImageView};

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

    let profile_picture = {
        let mut pfp = image::open(&profile_picture_path).unwrap();
        if pfp.width() > 500 || pfp.height() > 500 {
            pfp = pfp.resize(
                500.min(pfp.height()),
                500.min(pfp.height()),
                FilterType::Lanczos3,
            );
        }
        pfp
    };

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
    use image::imageops::{overlay, overlay_bounds};

    // Divide the profile picture into an evenly spaced 3x3 grid. We want the icon to occupy the
    // bottom-right nonant of the profile picture. However, we also want a bit of padding, so take
    // 10% of _that_ space on all sides and make it blank.
    //
    // Do all this by working in (1/3 nonant dimension divisions * 1/10 margin within a nonant) of
    // the profile picture.
    let icon_top_left_corner_coords = (
        profile_picture.width() * 21 / 30,
        profile_picture.height() * 21 / 30,
    );
    let icon_bottom_right_corner_coords = (
        profile_picture.width() * 29 / 30,
        profile_picture.height() * 29 / 30,
    );

    let rescaled_icon = {
        let (x, y) = overlay_bounds(
            icon_bottom_right_corner_coords,
            icon.dimensions(),
            icon_top_left_corner_coords.0,
            icon_top_left_corner_coords.1,
        );
        icon.resize(x, y, FilterType::Lanczos3)
    };

    overlay(
        profile_picture,
        &rescaled_icon,
        icon_top_left_corner_coords.0.into(),
        icon_top_left_corner_coords.1.into(),
    )
}
