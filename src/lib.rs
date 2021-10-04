use fltk_egui::{
    egui::{Image, TextureId, Vec2},
    Painter,
};

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
use image::{GenericImageView, ImageError};

#[cfg(any(
    feature = "svg",
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
pub struct ImageWidget {
    texture_id: TextureId,
    _size: Vec2,
}

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
impl Clone for ImageWidget {
    fn clone(&self) -> Self {
        Self {
            texture_id: self.texture_id.clone(),
            _size: self._size.clone(),
        }
    }
}

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
impl Drop for ImageWidget {
    fn drop(&mut self) {}
}

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
impl ImageWidget {
    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }

    pub fn size(&self) -> Vec2 {
        self._size
    }

    pub fn widget(&self) -> Image {
        Image::new(self.texture_id, self.size())
    }

    pub fn resize(&mut self, x: f32, y: f32) {
        self._size.x = x;
        self._size.y = y
    }

    pub fn set_size_x(&mut self, x: f32) {
        self._size.x = x;
    }

    pub fn set_size_y(&mut self, y: f32) {
        self._size.y = y
    }

    /// Create new image from bytes (image-format support will adapt according to the enabled features).
    pub fn new(painter: &mut Painter, bytes: &[u8]) -> Result<Self, ImageError> {
        let pixels;
        let size;
        {
            let dyn_image = image::load_from_memory(bytes)?;
            size = (dyn_image.width() as usize, dyn_image.height() as usize);
            pixels = dyn_image.into_rgba8().to_vec();
        }
        let tex = painter.new_user_texture_rgba8(size, pixels, true);
        let feimage = Self {
            texture_id: tex,
            _size: Vec2::from((size.0 as f32, size.1 as f32)),
        };
        Ok(feimage)
    }

    /// Update with new image from bytes.
    pub fn update_image(&mut self, painter: &mut Painter, bytes: &[u8]) -> Result<(), ImageError> {
        let pixels;
        let size;
        {
            let dyn_image = image::load_from_memory(bytes)?;
            size = (dyn_image.width() as usize, dyn_image.height() as usize);
            pixels = dyn_image.into_rgba8().to_vec();
        }
        painter.update_user_texture_rgba8_data(self.texture_id, pixels);
        self._size = Vec2::from((size.0 as f32, size.1 as f32));
        Ok(())
    }
}

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
impl Into<SVGWidget> for ImageWidget {
    fn into(self) -> SVGWidget {
        SVGWidget {
            texture_id: self.texture_id,
            _size: self._size,
        }
    }
}

#[cfg(any(
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
impl From<SVGWidget> for ImageWidget {
    fn from(t: SVGWidget) -> ImageWidget {
        ImageWidget {
            texture_id: t.texture_id,
            _size: t._size,
        }
    }
}

#[cfg(feature = "svg")]
use resvg::render;
#[cfg(feature = "svg")]
use std::io::{Error, ErrorKind};
#[cfg(feature = "svg")]
use tiny_skia::Pixmap;
#[cfg(feature = "svg")]
pub use usvg::Options;
#[cfg(feature = "svg")]
use usvg::{OptionsRef, Tree};

#[cfg(any(
    feature = "svg",
    feature = "png",
    feature = "jpeg",
    feature = "jpeg_rayon",
    feature = "bmp",
    feature = "ico",
    feature = "gif",
    feature = "webp",
    feature = "tga",
    feature = "pnm",
    feature = "hdr",
    feature = "dxt",
    feature = "dds",
    feature = "farbfeld"
))]
pub struct SVGWidget {
    texture_id: TextureId,
    _size: Vec2,
}

#[cfg(feature = "svg")]
impl Clone for SVGWidget {
    fn clone(&self) -> Self {
        Self {
            texture_id: self.texture_id.clone(),
            _size: self._size.clone(),
        }
    }
}

#[cfg(feature = "svg")]
impl Drop for SVGWidget {
    fn drop(&mut self) {}
}

#[cfg(feature = "svg")]
impl SVGWidget {
    pub fn texture_id(&self) -> TextureId {
        self.texture_id
    }

    pub fn size(&self) -> Vec2 {
        self._size
    }

    pub fn widget(&self) -> Image {
        Image::new(self.texture_id, self.size())
    }

    pub fn resize(&mut self, x: f32, y: f32) {
        self._size.x = x;
        self._size.y = y
    }

    pub fn set_size_x(&mut self, x: f32) {
        self._size.x = x;
    }

    pub fn set_size_y(&mut self, y: f32) {
        self._size.y = y
    }

    /// Create new svg image from bytes.
    pub fn new(painter: &mut Painter, bytes: &[u8], opt_ref: OptionsRef) -> Result<Self, Error> {
        let rtree = match Tree::from_data(&bytes, &opt_ref) {
            Ok(rtree) => rtree,
            Err(e) => {
                let err = Error::new(ErrorKind::Other, e.to_string());
                return Err(err);
            }
        };
        let pixels;
        let size;
        {
            let pixmap_size = rtree.svg_node().size.to_screen_size();
            let mut pixmap = match Pixmap::new(pixmap_size.width(), pixmap_size.height()) {
                Some(pixmap) => pixmap,
                _ => {
                    let err = Error::new(ErrorKind::Other, "while mapping SVG pixels!");
                    return Err(err);
                }
            };

            {
                if render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).is_none() {
                    let err = Error::new(ErrorKind::Other, "while rendering SVG data!");
                    return Err(err);
                }
            }
            size = (pixmap_size.width() as usize, pixmap_size.height() as usize);
            pixels = pixmap.data().to_vec();
        }
        let tex = painter.new_user_texture_rgba8(size, pixels, true);
        let feimage = Self {
            texture_id: tex,
            _size: Vec2::from((size.0 as f32, size.1 as f32)),
        };
        Ok(feimage)
    }

    /// Update with new svg image from bytes.
    pub fn update_svg(
        &mut self,
        painter: &mut Painter,
        bytes: &[u8],
        opt_ref: OptionsRef,
    ) -> Result<(), Error> {
        let rtree = match Tree::from_data(&bytes, &opt_ref) {
            Ok(rtree) => rtree,
            Err(e) => {
                let err = Error::new(ErrorKind::Other, e.to_string());
                return Err(err);
            }
        };
        let pixels;
        let size;
        {
            let pixmap_size = rtree.svg_node().size.to_screen_size();
            let mut pixmap = match Pixmap::new(pixmap_size.width(), pixmap_size.height()) {
                Some(pixmap) => pixmap,
                _ => {
                    let err = Error::new(ErrorKind::Other, "while mapping SVG pixels!");
                    return Err(err);
                }
            };

            {
                if render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).is_none() {
                    let err = Error::new(ErrorKind::Other, "while rendering SVG data!");
                    return Err(err);
                }
            }

            size = (pixmap_size.width() as usize, pixmap_size.height() as usize);
            pixels = pixmap.data().to_vec();
        }
        painter.update_user_texture_rgba8_data(self.texture_id, pixels);
        self._size = Vec2::from((size.0 as f32, size.1 as f32));
        Ok(())
    }
}

#[cfg(feature = "svg")]
impl Into<ImageWidget> for SVGWidget {
    fn into(self) -> ImageWidget {
        ImageWidget {
            texture_id: self.texture_id,
            _size: self._size,
        }
    }
}

#[cfg(feature = "svg")]
impl From<ImageWidget> for SVGWidget {
    fn from(t: ImageWidget) -> SVGWidget {
        SVGWidget {
            texture_id: t.texture_id,
            _size: t._size,
        }
    }
}
