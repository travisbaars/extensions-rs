//! Images
use enum_iterator::{all, Sequence};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord, Sequence)]
pub enum Image {
  Ext3DM,

  Ext3DS,

  ExtAI,

  ExtBMP,

  ExtDDS,

  ExtDWG,

  ExtDXF,

  ExtEPS,

  ExtGIF,
  ExtGPX,
  ExtJPG,
  ExtJPEG,
  ExtKML,
  ExtKMZ,
  ExtMAX,
  ExtPNG,
  ExtPS,
  ExtPSD,
  ExtSVG,
  ExtTGA,
  ExtTHM,
  ExtTIF,
  ExtTIFF,
  ExtWEBP,
  ExtXCF,
  ExtYUV,
  None,
}

impl Image {
  pub async fn contains(item: &str) -> bool {
    all::<Image>().any(|_| Image::from(item) != Image::None)
  }
}

impl From<&str> for Image {
  fn from(s: &str) -> Self {
    match s {
      "3dm" => Self::Ext3DM,
      "3ds" => Self::Ext3DS,
      "ai" => Self::ExtAI,
      "bmp" => Self::ExtBMP,
      "dds" => Self::ExtDDS,
      "dwg" => Self::ExtDWG,
      "dxf" => Self::ExtDXF,
      "eps" => Self::ExtEPS,
      "gif" => Self::ExtGIF,
      "gpx" => Self::ExtGPX,
      "jpg" => Self::ExtJPG,
      "jpeg" => Self::ExtJPEG,
      "kml" => Self::ExtKML,
      "kmz" => Self::ExtKMZ,
      "max" => Self::ExtMAX,
      "png" => Self::ExtPNG,
      "ps" => Self::ExtPS,
      "psd" => Self::ExtPSD,
      "svg" => Self::ExtSVG,
      "tga" => Self::ExtTGA,
      "thm" => Self::ExtTHM,
      "tif" => Self::ExtTIF,
      "tiff" => Self::ExtTIFF,
      "webp" => Self::ExtWEBP,
      "xcf" => Self::ExtXCF,
      "yuv" => Self::ExtYUV,
      _ => Self::None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_from_str() {
    assert_eq!(Image::Ext3DM, Image::from("3dm"));
    assert_eq!(Image::Ext3DS, Image::from("3ds"));
    assert_eq!(Image::ExtAI, Image::from("ai"));
    assert_eq!(Image::ExtBMP, Image::from("bmp"));
    assert_eq!(Image::ExtDDS, Image::from("dds"));
    assert_eq!(Image::ExtDWG, Image::from("dwg"));
    assert_eq!(Image::ExtDXF, Image::from("dxf"));
    assert_eq!(Image::ExtEPS, Image::from("eps"));
    assert_eq!(Image::ExtGIF, Image::from("gif"));
    assert_eq!(Image::ExtGPX, Image::from("gpx"));
    assert_eq!(Image::ExtJPG, Image::from("jpg"));
    assert_eq!(Image::ExtJPEG, Image::from("jpeg"));
    assert_eq!(Image::ExtKML, Image::from("kml"));
    assert_eq!(Image::ExtKMZ, Image::from("kmz"));
    assert_eq!(Image::ExtMAX, Image::from("max"));
    assert_eq!(Image::ExtPNG, Image::from("png"));
    assert_eq!(Image::ExtPS, Image::from("ps"));
    assert_eq!(Image::ExtPSD, Image::from("psd"));
    assert_eq!(Image::ExtSVG, Image::from("svg"));
    assert_eq!(Image::ExtTGA, Image::from("tga"));
    assert_eq!(Image::ExtTHM, Image::from("thm"));
    assert_eq!(Image::ExtTIF, Image::from("tif"));
    assert_eq!(Image::ExtTIFF, Image::from("tiff"));
    assert_eq!(Image::ExtWEBP, Image::from("webp"));
    assert_eq!(Image::ExtXCF, Image::from("xcf"));
    assert_eq!(Image::ExtYUV, Image::from("yuv"));
    assert_eq!(Image::None, Image::from(""));
  }
}
