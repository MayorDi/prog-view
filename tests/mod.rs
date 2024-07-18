#![cfg(test)]

use image::Rgb;
use prog_view::Canvas;

#[test]
fn create_image() {
    let (w, h) = (100, 100);
    let mut imgbuf  = image::RgbImage::new(w, h);

    let bytes = Canvas::<3, _>::new(w as usize, h as usize)
        .logic_func(|x, y| {
            let f = x.powf(2.0) + y.powf(2.0);

            if f <= 1.0 {
                return [255; 3];
            }

            [0; 3]
        })
        .run();

    for x in 0..w {
        for y in 0..h {
            let idx = (y as usize * w as usize + x as usize) * 3usize;
            let pixel = &bytes[idx..(idx+3)];

            imgbuf.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]))
        }
    }

    imgbuf.save("math.png").unwrap();
}
