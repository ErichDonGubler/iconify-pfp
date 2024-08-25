use std::fs::create_dir_all;

use camino::Utf8PathBuf;
use clap::Parser as _;
use image::{imageops::FilterType, ColorType, DynamicImage, GenericImageView};

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
        let grayscale_pfp = pfp.grayscale();
        pfp = match pfp.color() {
            ColorType::L8 => grayscale_pfp.into_luma8().into(),
            ColorType::La8 => grayscale_pfp.into_luma_alpha8().into(),
            ColorType::Rgb8 => grayscale_pfp.into_rgb8().into(),
            ColorType::Rgba8 => grayscale_pfp.into_rgba8().into(),
            ColorType::L16 => grayscale_pfp.into_luma16().into(),
            ColorType::La16 => grayscale_pfp.into_luma_alpha16().into(),
            ColorType::Rgb16 => grayscale_pfp.into_rgb16().into(),
            ColorType::Rgba16 => grayscale_pfp.into_rgba16().into(),
            ColorType::Rgb32F => grayscale_pfp.into_rgb32f().into(),
            ColorType::Rgba32F => grayscale_pfp.into_rgba32f().into(),
            _ => unimplemented!("looks like it's time to add a new format!"),
        };
        pfp
    };

    for icon_path in icon_paths {
        let icon = image::open(&icon_path).unwrap();
        let iconified_profile_picture = {
            let mut pfp = profile_picture.clone();
            overlay_icon_over_pfp(&mut pfp, &icon);
            pfp
        };

        let iconified_pfp_file_path =
            out_dir.join(format!("{}.png", icon_path.file_stem().unwrap()));
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
