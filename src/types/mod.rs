pub mod image;

use self::image::Image;
// use crate::ext::Image;
use crate::Extension::*;
use enum_iterator::Sequence;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord, Sequence)]
pub enum Extension {
    Image(Image),
    None,
}

#[derive(Debug, PartialEq, Clone, Copy, Sequence)]
pub enum ExtType {
    Image,
    None,
}

impl Extension {
    pub fn to_str(self) -> &'static str {
        match self {
            self::Image(i) => match i {
                Image::Ext3DM => "3dm",
                Image::Ext3DS => "3ds",
                Image::ExtAI => "ai",
                Image::ExtBMP => "bmp",
                Image::ExtDDS => "dds",
                Image::ExtDWG => "dwg",
                Image::ExtDXF => "dxf",
                Image::ExtEPS => "eps",
                Image::ExtGIF => "gif",
                Image::ExtGPX => "gpx",
                Image::ExtJPG => "jpg",
                Image::ExtJPEG => "jpeg",
                Image::ExtKML => "kml",
                Image::ExtKMZ => "kmz",
                Image::ExtMAX => "max",
                Image::ExtPNG => "png",
                Image::ExtPS => "ps",
                Image::ExtPSD => "psd",
                Image::ExtSVG => "svg",
                Image::ExtTGA => "tga",
                Image::ExtTHM => "thm",
                Image::ExtTIF => "tif",
                Image::ExtTIFF => "tiff",
                Image::ExtWEBP => "webp",
                Image::ExtXCF => "xcf",
                Image::ExtYUV => "yuv",
                Image::None => "",
            },
            self::None => "",
        }
    }
}

impl From<Image> for Extension {
    fn from(image: Image) -> Self {
        match image {
            Image::Ext3DM => Self::Image(Image::Ext3DM),
            _ => Self::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_from_image() {
        assert_eq!(
            Extension::Image(Image::Ext3DM),
            Extension::from(Image::Ext3DM)
        );
    }

    #[test]
    fn test_ext_image_to_str() {
        assert_eq!("3dm", Extension::Image(Image::Ext3DM).to_str());
        assert_eq!("3ds", Extension::Image(Image::Ext3DS).to_str());
        assert_eq!("ai", Extension::Image(Image::ExtAI).to_str());
        assert_eq!("bmp", Extension::Image(Image::ExtBMP).to_str());
        assert_eq!("dds", Extension::Image(Image::ExtDDS).to_str());
        assert_eq!("dwg", Extension::Image(Image::ExtDWG).to_str());
        assert_eq!("dxf", Extension::Image(Image::ExtDXF).to_str());
        assert_eq!("eps", Extension::Image(Image::ExtEPS).to_str());
        assert_eq!("gif", Extension::Image(Image::ExtGIF).to_str());
        assert_eq!("gpx", Extension::Image(Image::ExtGPX).to_str());
        assert_eq!("jpg", Extension::Image(Image::ExtJPG).to_str());
        assert_eq!("jpeg", Extension::Image(Image::ExtJPEG).to_str());
        assert_eq!("kml", Extension::Image(Image::ExtKML).to_str());
        assert_eq!("kmz", Extension::Image(Image::ExtKMZ).to_str());
        assert_eq!("max", Extension::Image(Image::ExtMAX).to_str());
        assert_eq!("png", Extension::Image(Image::ExtPNG).to_str());
        assert_eq!("ps", Extension::Image(Image::ExtPS).to_str());
        assert_eq!("psd", Extension::Image(Image::ExtPSD).to_str());
        assert_eq!("svg", Extension::Image(Image::ExtSVG).to_str());
        assert_eq!("tga", Extension::Image(Image::ExtTGA).to_str());
        assert_eq!("thm", Extension::Image(Image::ExtTHM).to_str());
        assert_eq!("tif", Extension::Image(Image::ExtTIF).to_str());
        assert_eq!("tiff", Extension::Image(Image::ExtTIFF).to_str());
        assert_eq!("webp", Extension::Image(Image::ExtWEBP).to_str());
        assert_eq!("xcf", Extension::Image(Image::ExtXCF).to_str());
        assert_eq!("yuv", Extension::Image(Image::ExtYUV).to_str());
        assert_eq!("", Extension::Image(Image::None).to_str());
    }
}
