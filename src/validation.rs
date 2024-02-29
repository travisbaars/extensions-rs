//! Validation
use lazy_static::lazy_static;
use std::collections::HashSet;

// use crate::Extension;

pub struct Validate {}

impl Validate {
  pub async fn check_str(input: &str) -> bool {
    if VALID_EXT.contains(input) {
      return true;
    }

    false
  }
}

lazy_static! {
  pub static ref VALID_EXT: HashSet<&'static str> = HashSet::from_iter([
    // Archive Extensions
    "7z", "a", "apk", "ar", "bz2", "cab", "cpio", "deb", "dmg", "egg", "gz", "iso", "jar",
    "lha", "mar", "pea", "rar", "rpm", "s7z", "shar", "tar", "tbz2", "tgz", "tlz", "war",
    "whl", "xpi", "zip", "zipx", "xz", "pak",
    // Audio Extensions
    "aac", "aiff", "ape", "au", "flac", "gsm", "it", "m3u", "m4a", "mid", "mod", "mp3", "mpa",
    "pls", "ra", "s3m", "sid", "wav", "wma", "xm",
    // Book Extensions
    "mobi", "epub", "azw1", "azw3", "azw4", "azw6", "azw", "cbr", "cbz",
    // Executable Extensions
    "exe", "msi", "bin", "command", "sh", "bat", "crx", "bash", "csh", "fish", "ksh", "zsh",
    // Image Extensions
    "3dm", "3ds", "max", "bmp", "dds", "gif", "jpg", "jpeg", "png", "psd", "xcf", "tga",
    "thm", "tif", "tiff", "yuv", "ai", "eps", "ps","svg", "dwg", "dxf","gpx", "kml",
    "kmz", "webp",
    // Programming Extensions
    "1.ada", "2.ada", "ada", "adb", "ads", "asm", "bas", "bash", "bat", "c", "c++", "cbl", "cc",
    "class", "clj", "cob", "cpp", "cs", "csh", "cxx", "d", "diff", "e", "el", "f", "f77", "f90",
    "fish", "for", "fth", "ftn", "go", "groovy", "h", "hh", "hpp", "hs", "html", "htm", "hxx",
    "java", "js", "jsx", "jsp", "ksh", "kt", "lhs", "lisp", "lua", "m", "m4", "nim", "patch", "php",
    "pl", "po", "pp", "py", "r", "rb", "rs", "s", "scala", "sh", "swg", "swift", "v", "vb", "vcxproj",
    "xcodeproj","xml", "zsh",
    // Text Extensions
    "doc", "docx", "ebook", "log", "md", "msg", "odt", "org", "pages", "pdf", "rtf", "rst", "tex", "txt",
    "wpd", "wps",
    // Video Extensions
    "3g2", "3gp", "aaf", "asf", "avchd", "avi", "drc", "flv", "m2v", "m4p", "m4v", "mkv", "mng", "mov", "mp2",
    "mp4", "mpe", "mpeg", "mpg", "mpv", "mxf", "nsv", "ogg", "ogv", "ogm", "qt", "rm", "rmvb", "roq", "srt",
    "svi", "vob", "webm", "wmv", "yuv",
    // Web Extensions
    "html", "htm", "css", "js", "jsx", "less", "scss", "wasm", "php"
  ]);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_validate_check() {
    assert_eq!(true, Validate::check_str("jpg").await);
    assert_eq!(true, Validate::check_str("m2v").await);
    assert_eq!(true, Validate::check_str("c++").await);
    assert_eq!(false, Validate::check_str("").await);
  }
}
