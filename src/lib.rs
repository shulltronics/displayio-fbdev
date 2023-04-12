use embedded_graphics::{
    prelude::*,
    pixelcolor::Bgr888,
    geometry::{OriginDimensions, Size},
    draw_target::DrawTarget,
};

use framebuffer::Framebuffer;

use pyo3::prelude::*;

#[pyfunction]
fn rust2py_test(a: String) -> PyResult<String> {
    Ok(a)
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[pyclass]
pub enum Orientation {
    PORTRAIT,
    LANDSCAPE,
}

#[pyclass]
pub struct FramebufferDisplay {
    // underlying Framebuffer struct
    fb: Framebuffer,
    // pixel width and height of screen
    width: u32,
    height: u32,
    // number of pixels per line
    _ppl: u32,
    // orientation of device
    orientation: Orientation,
}

#[pymethods]
impl FramebufferDisplay {
    #[new]
    pub fn new() -> FramebufferDisplay {
        let fb_raw = Framebuffer::new("/dev/fb0").unwrap();
        let xres: u32 = fb_raw.var_screen_info.xres;
        println!("fb width: {:?}", xres);
        let yres: u32 = fb_raw.var_screen_info.yres;
        println!("fb height: {:?}", yres);
        // get the BYTES per pixel
        let bpp: u32 = fb_raw.var_screen_info.bits_per_pixel / 8;
        assert!(bpp == 4, "This crate only supports 4 bytes-per-pixel framebuffers!");
        // println!("fb bpp: {:?}", bpp);
        let ll: u32 = fb_raw.fix_screen_info.line_length;
        println!("line length: {:?}", ll);
        let ppl: u32 = ll / bpp;
        println!("pixels per line: {:?}", ppl);

        Self {
            fb: fb_raw,
            width: xres,
            height: yres,
            _ppl: ppl,
            // start in PORTRAIT
            orientation: Orientation::PORTRAIT,
        }
    }


    pub fn get_orientation(&self) -> Orientation {
        return self.orientation;
    }

    pub fn set_orientation(&mut self, o: Orientation) {
        if o != self.orientation {
            let (w, h) = (self.width, self.height);
            self.width = h;
            self.height = w;
            self.orientation = o;
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    pub fn clear(&mut self) {
        let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
        for i in 0..pixels.len() {
            pixels[i] = 0u32;
        }
    }

    pub fn fill(&mut self, color: u32) {
        let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
        for idx in 0..pixels.len() {
            pixels[idx] = color;
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        // Get both of the pixel buffers as 4 byte slices
        let (_fb_prefix, fb_pixels, _fb_suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
        let (_circpy_prefix, circpy_pixels, _circpy_suffic) = unsafe {
            bytes.align_to::<u32>()
        };
        // For now we only support drawing to the whole screen
        assert!(circpy_pixels.len() == (self.width*self.height) as usize, "TODO: support sub rectangles");
        // Loop through the screen coordinates, and draw the pixels
        for x in 0..self.width-1 {
            for y in 0..self.height-1 {
                let circpy_idx = (x + y*self.width) as usize;
                // Get the index into the framebuffer (accounting for hidden indexes)
                let fb_idx: usize = match self.orientation {
                    Orientation::PORTRAIT => {
                        (x + y*self._ppl) as usize
                    },
                    Orientation::LANDSCAPE => {
                        ((self.height - 1 - y) + x*(self._ppl)) as usize
                    },
                };
                fb_pixels[fb_idx] = circpy_pixels[circpy_idx];
            }            
        }
    }
    
    pub fn set_pixel(&mut self, idx: usize, color: u32) {
        let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
        pixels[idx] = color;
    }

}


impl FramebufferDisplay {
    
    pub fn set_idx(&mut self, idx: usize, color: Bgr888) {
        let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
        pixels[idx] = (color.into_storage()) as u32;
    }

}


#[pymodule]
fn framebuffer_display(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust2py_test, m)?)?;
    m.add_class::<FramebufferDisplay>()?;
    m.add_class::<Orientation>()?;
    Ok(())
}

impl DrawTarget for FramebufferDisplay {
    type Color = Bgr888;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let (x, y) = (coord.x as u32, coord.y as u32);
            // constrain pixels to screen area
            if x > self.width-1 || y > self.height-1 {
                continue;
            }
            let idx: usize;
            if self.orientation == Orientation::PORTRAIT {
                idx = (x + y*self._ppl) as usize;
            } else {
                // make sure to account for the invisible "pixels" outside the visible region
                idx = ((self.height - 1 - y) + x*(self._ppl)) as usize;
            }
            // we don't multiply the second term by bits-per-pixels because we cast the buffer here
            let (_prefix, pixels, _suffix) = unsafe { self.fb.frame.align_to_mut::<u32>() };
            pixels[idx] = (color.into_storage()) as u32;
        }

        return Ok(());
    }

}

impl OriginDimensions for FramebufferDisplay {
    fn size(&self) -> Size {
        return Size::new(self.width, self.height);
    }
}
