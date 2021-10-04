use fltk_egui::{
    egui::{Color32, Image, TextureId, Vec2},
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
#[derive(Clone)]
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
            // pixels = dyn_image.into_rgba8().to_vec()
            pixels = dyn_image
                .to_rgba8()
                .chunks(4)
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect::<Vec<_>>();
        }
        // let tex = painter.new_user_texture_rgba8(size, pixels, true);
        let tex = painter.new_user_texture(size, &pixels, true);
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
            // pixels = dyn_image.into_rgba8().to_vec()
            pixels = dyn_image
                .to_rgba8()
                .chunks(4)
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect::<Vec<_>>();
        }
        // painter.update_user_texture_rgba8_data(self.texture_id, pixels);
        painter.update_user_texture_data(self.texture_id, &pixels);
        self._size = Vec2::from((size.0 as f32, size.1 as f32));
        Ok(())
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

#[cfg(feature = "svg")]
#[derive(Clone)]
pub struct SVGWidget {
    texture_id: TextureId,
    _size: Vec2,
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
            // pixels = pixmap.data().to_vec();
            pixels = pixmap
                .data()
                .chunks(4)
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect::<Vec<_>>();
        }
        // let tex = painter.new_user_texture_rgba8(size, pixels, true);
        let tex = painter.new_user_texture(size, &pixels, true);
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
            // pixels = pixmap.data().to_vec();
            pixels = pixmap
                .data()
                .chunks(4)
                .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
                .collect::<Vec<_>>();
        }
        // egui-fltk next release will be enabled.
        // painter.update_user_texture_rgba8_data(self.texture_id, pixels);
        painter.update_user_texture_data(self.texture_id, &pixels);
        self._size = Vec2::from((size.0 as f32, size.1 as f32));
        Ok(())
    }
}
