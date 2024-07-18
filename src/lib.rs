use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Debug, Getters, CopyGetters, MutGetters, Setters)]
pub struct Canvas<const CHANNELS: usize, LF> {
    #[getset(get_copy = "pub", set = "pub")]
    width: usize,
    #[getset(get_copy = "pub", set = "pub")]
    height: usize,
    #[getset(get_copy = "pub", set = "pub")]
    step: f32,
    #[getset(get_copy = "pub", set = "pub")]
    min_x: f32,
    #[getset(get_copy = "pub", set = "pub")]
    min_y: f32,
    #[getset(get_copy = "pub", set = "pub")]
    max_x: f32,
    #[getset(get_copy = "pub", set = "pub")]
    max_y: f32,
    #[getset(get_copy = "pub", set = "pub")]
    x: f32,
    y: f32,
    #[getset(get_mut = "pub", get = "pub", set = "pub")]
    map: Vec<u8>,
    logic_function: Option<LF>,
}

impl<const CHANNELS: usize, LF> Canvas<CHANNELS, LF> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            map: vec![0; width * height * CHANNELS],
            step: 0.001,
            min_x: -1.5,
            min_y: -1.5,
            max_x: 1.5,
            max_y: 1.5,
            x: -1.5,
            y: -1.5,
            logic_function: None,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; CHANNELS] {
        let mut pixel: [u8; CHANNELS] = [0; CHANNELS];

        let idx = (y * self.width + x) * CHANNELS;

        let mut i = 0;
        for c in self.map()[idx..(idx + CHANNELS)].iter() {
            pixel[i] = *c;
            i += 1;
        }

        pixel
    }

    pub fn set_pixel(&mut self, new_pixel: [u8; CHANNELS], x: usize, y: usize) {
        let idx = (y * self.width + x) * CHANNELS;

        // if idx >= self.map().len() {
        //     return;
        // }

        let mut i = 0;
        for c in self.map_mut()[idx..(idx + CHANNELS)].iter_mut() {
            *c = new_pixel[i];
            i += 1;
        }
    }
}

impl<const CHANNELS: usize, LF: Fn(f32, f32) -> [u8; CHANNELS]> Canvas<CHANNELS, LF> {
    pub fn logic_func(mut self, lf: LF) -> Self {
        self.logic_function = Some(lf);

        self
    }

    pub fn run(mut self) -> Vec<u8> {
        let size = (
            (self.min_x().abs() + self.max_x().abs()),
            (self.min_y().abs() + self.max_y().abs()),
        );

        let (step_map_w, step_map_h) =
            (self.width() as f32 / size.0, self.height() as f32 / size.1);

        let mut y = self.y;
        while y < self.max_y() {
            let mut x = self.x;
            while x < self.max_x() {
                let pixel = {
                    let Some(f) = &self.logic_function else {
                        unreachable!()
                    };
                    f(x, y)
                };

                self.set_pixel(
                    pixel,
                    ((self.max_x() + x) * step_map_w).floor() as usize,
                    ((self.max_y() + y) * step_map_h).floor() as usize,
                );

                x += self.step();
            }

            y += self.step();
        }

        self.map
    }
}
