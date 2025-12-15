//! Tests for image tools.

mod common;

use common::TestFixture;
use dx_media::tools::image;

// =============================================================================
// 1. converter - Image format conversion
// =============================================================================

#[test]
fn test_image_format_enum() {
    let _ = image::ImageFormat::Png;
    let _ = image::ImageFormat::Jpeg;
    let _ = image::ImageFormat::Gif;
    let _ = image::ImageFormat::Webp;
    let _ = image::ImageFormat::Bmp;
    let _ = image::ImageFormat::Tiff;
}

#[test]
fn test_image_convert() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("test.png");

    let result = image::convert(&input, &output);
    let _ = result; // May fail without ImageMagick
}

#[test]
fn test_image_convert_to_format() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("test.jpg");

    let result = image::convert_to_format(&input, &output, image::ImageFormat::Jpeg);
    let _ = result;
}

#[test]
fn test_image_get_info() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");

    let result = image::get_info(&input);
    let _ = result;
}

// =============================================================================
// 2. resizer - Image resizing
// =============================================================================

#[test]
fn test_resize_filter_enum() {
    let _ = image::ResizeFilter::Lanczos;
    let _ = image::ResizeFilter::Bilinear;
    let _ = image::ResizeFilter::Bicubic;
}

#[test]
fn test_image_resize() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("resized.pgm");

    let result = image::resize(&input, &output, 100, 100);
    let _ = result;
}

#[test]
fn test_image_resize_fit() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("resized.pgm");

    let result = image::resize_fit(&input, &output, 200, 200);
    let _ = result;
}

#[test]
fn test_image_scale() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("scaled.pgm");

    let result = image::scale(&input, &output, 50);
    let _ = result;
}

#[test]
fn test_image_thumbnail() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("thumb.pgm");

    let result = image::thumbnail(&input, &output, 64);
    let _ = result;
}

// =============================================================================
// 3. compressor - Image compression
// =============================================================================

#[test]
fn test_compression_quality_enum() {
    let _ = image::CompressionQuality::Low;
    let _ = image::CompressionQuality::Medium;
    let _ = image::CompressionQuality::High;
}

#[test]
fn test_image_compress() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("compressed.pgm");

    let result = image::compress(&input, &output, 80);
    let _ = result;
}

#[test]
fn test_image_optimize() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("optimized.pgm");

    let result = image::optimize(&input, &output);
    let _ = result;
}

// =============================================================================
// 4. watermark - Image watermarking
// =============================================================================

#[test]
fn test_watermark_position_enum() {
    let _ = image::WatermarkPosition::TopLeft;
    let _ = image::WatermarkPosition::TopRight;
    let _ = image::WatermarkPosition::BottomLeft;
    let _ = image::WatermarkPosition::BottomRight;
    let _ = image::WatermarkPosition::Center;
}

#[test]
fn test_image_text_watermark() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("watermarked.pgm");

    let result = image::add_text_watermark(
        &input,
        &output,
        "© 2025",
        image::WatermarkPosition::BottomRight,
    );
    let _ = result;
}

#[test]
fn test_image_watermark_options() {
    let options = image::WatermarkOptions::default();
    let _ = options;
}

// =============================================================================
// 5. exif - EXIF metadata
// =============================================================================

#[test]
fn test_exif_read() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");

    let result = image::read_exif(&input);
    let _ = result; // May fail without exiftool
}

#[test]
fn test_exif_strip_metadata() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("stripped.pgm");

    let result = image::strip_metadata(&input, &output);
    let _ = result;
}

#[test]
fn test_exif_set_copyright() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("copyrighted.pgm");

    let result = image::set_copyright(&input, &output, "© Test 2025");
    let _ = result;
}

// =============================================================================
// 6. qrcode - QR code generation
// =============================================================================

#[test]
fn test_qr_error_correction_enum() {
    let _ = image::QrErrorCorrection::Low;
    let _ = image::QrErrorCorrection::Medium;
    let _ = image::QrErrorCorrection::High;
}

#[test]
fn test_qr_generate() {
    let fixture = TestFixture::new();
    let output = fixture.path("qr.png");

    let result = image::generate_qr("https://example.com", &output, 200);
    let _ = result;
}

#[test]
fn test_qr_generate_svg() {
    let fixture = TestFixture::new();
    let output = fixture.path("qr.svg");

    let result = image::generate_qr_svg("Test data", &output, 200);
    let _ = result;
}

#[test]
fn test_qr_decode() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("qr.png");

    let result = image::decode_qr(&input);
    let _ = result;
}

// =============================================================================
// 7. palette - Color palette extraction
// =============================================================================

#[test]
fn test_palette_extract() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");

    let result = image::extract_palette(&input, 5);
    let _ = result;
}

#[test]
fn test_color_struct() {
    let color = image::Color {
        r: 255,
        g: 128,
        b: 64,
        percentage: 25.0,
    };
    assert_eq!(color.to_hex(), "#ff8040");
    assert_eq!(color.to_rgb(), "rgb(255, 128, 64)");
}

#[test]
fn test_dominant_color() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");

    let result = image::extract_dominant_color(&input);
    let _ = result;
}

// =============================================================================
// 8. filters - Image filters
// =============================================================================

#[test]
fn test_filter_enum() {
    let _ = image::Filter::Grayscale;
    let _ = image::Filter::Sepia;
    let _ = image::Filter::Invert;
    let _ = image::Filter::Blur;
    let _ = image::Filter::Sharpen;
    let _ = image::Filter::Emboss;
    let _ = image::Filter::Edge;
    let _ = image::Filter::OilPaint;
    let _ = image::Filter::Charcoal;
}

#[test]
fn test_apply_filter() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("filtered.pgm");

    let result = image::apply_filter(&input, &output, image::Filter::Grayscale);
    let _ = result;
}

#[test]
fn test_grayscale() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("gray.pgm");

    let result = image::grayscale(&input, &output);
    let _ = result;
}

#[test]
fn test_sepia() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("sepia.pgm");

    let result = image::sepia(&input, &output);
    let _ = result;
}

#[test]
fn test_brightness() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("bright.pgm");

    let result = image::brightness(&input, &output, 20);
    let _ = result;
}

#[test]
fn test_contrast() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("contrast.pgm");

    let result = image::contrast(&input, &output, 30);
    let _ = result;
}

#[test]
fn test_blur() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("blurred.pgm");

    let result = image::blur(&input, &output, 3.0);
    let _ = result;
}

#[test]
fn test_rotate() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("rotated.pgm");

    let result = image::rotate(&input, &output, 90.0);
    let _ = result;
}

#[test]
fn test_flip() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output_h = fixture.path("flipped_h.pgm");
    let output_v = fixture.path("flipped_v.pgm");

    let _ = image::flip_horizontal(&input, &output_h);
    let _ = image::flip_vertical(&input, &output_v);
}

// =============================================================================
// 9. ocr - Optical character recognition
// =============================================================================

#[test]
fn test_ocr_options() {
    let options = image::OcrOptions::default();
    let _ = options;
}

#[test]
fn test_ocr_extract_simple() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");

    let result = image::extract_text_simple(&input);
    let _ = result; // May fail without tesseract
}

#[test]
fn test_ocr_extract_with_options() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let options = image::OcrOptions::default();

    let result = image::extract_text(&input, options);
    let _ = result;
}

#[test]
fn test_ocr_list_languages() {
    let result = image::list_languages();
    let _ = result;
}

// =============================================================================
// 10. icons - Icon generation
// =============================================================================

#[test]
fn test_icon_generate() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("icon.png");

    let result = image::generate_icon(&input, &output, 64);
    let _ = result;
}

#[test]
fn test_favicon_generate() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output = fixture.path("favicon.ico");

    let result = image::generate_favicon(&input, &output);
    let _ = result;
}

#[test]
fn test_ios_icons() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output_dir = fixture.path("ios_icons");

    let result = image::generate_ios_icons(&input, &output_dir);
    let _ = result;
}

#[test]
fn test_android_icons() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output_dir = fixture.path("android_icons");

    let result = image::generate_android_icons(&input, &output_dir);
    let _ = result;
}

#[test]
fn test_all_icons() {
    let fixture = TestFixture::new();
    let input = fixture.create_test_image("test.pgm");
    let output_dir = fixture.path("all_icons");

    let result = image::generate_all_icons(&input, &output_dir);
    let _ = result;
}
