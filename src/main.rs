use fft2d::slice::fft_2d;
use image::GrayImage;
use rustfft::num_complex::Complex;

fn main() -> color_eyre::Result<()> {
    let img = image::open("output.png")?.into_luma8();
    let (width, height) = img.dimensions();

    // Convert the image buffer to complex numbers to be able to compute the FFT.
    let mut img_buffer: Vec<Complex<f64>> = img
        .as_raw()
        .iter()
        .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
        .collect();
    fft_2d(width as usize, height as usize, &mut img_buffer);

    // Convert the complex img_buffer back into a gray image.
    let img_raw: Vec<u8> = img_buffer
        .iter()
        .map(|c| (c.norm().min(1.0) * 255.0) as u8)
        .collect();
    let out_img = GrayImage::from_raw(width, height, img_raw).unwrap();

    out_img.save("tranformed.png")?;
    Ok(())
}
