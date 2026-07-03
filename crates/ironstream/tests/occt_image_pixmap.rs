extern crate ironstream;

use ironstream::image_pixmap::{AlienPixMap, ImageFormat, PixMap, SupportedFormats};

#[test]
fn pixmap_rgba_round_trip() {
    let mut pm = PixMap::new(4, 4, ImageFormat::Rgba);
    assert!(pm.set_pixel_rgba(1, 2, 255, 128, 0, 255));
    let px = pm.get_pixel_rgba(1, 2).unwrap();
    assert_eq!(px, [255, 128, 0, 255]);
}

#[test]
fn pixmap_size_bytes_rgb() {
    let pm = PixMap::new(8, 8, ImageFormat::Rgb);
    assert_eq!(pm.size_bytes(), 8 * 8 * 3);
    assert_eq!(pm.row_size(), 8 * 3);
}

#[test]
fn pixmap_fill_and_empty() {
    let mut pm = PixMap::new(2, 2, ImageFormat::Gray);
    pm.fill(255);
    assert!(pm.data.iter().all(|&b| b == 255));
    let empty = PixMap::new(0, 0, ImageFormat::Gray);
    assert!(empty.is_empty());
}

#[test]
fn image_format_bytes_per_pixel() {
    assert_eq!(ImageFormat::GrayF.bytes_per_pixel(), 4);
    assert_eq!(ImageFormat::Rgba.bytes_per_pixel(), 4);
    assert_eq!(ImageFormat::Bgr.bytes_per_pixel(), 3);
}

#[test]
fn alien_pixmap_and_supported_formats() {
    let mut ap = AlienPixMap::new(10, 10, ImageFormat::Rgb);
    assert!(!ap.load("image.png"));
    assert_eq!(ap.width(), 10);
    assert_eq!(ap.height(), 10);
    let sf = SupportedFormats::new();
    assert!(sf.has("jpeg"));
    assert!(!sf.has("SVG"));
}
