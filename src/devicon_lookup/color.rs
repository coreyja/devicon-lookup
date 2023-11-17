use ansi_term::Style;
use ansi_term::Colour::*;
pub use crate::file::File;
pub use crate::file_ext::FileExtensions;

pub fn iconify_style(style: Style) -> Style {
   style.background.or(style.foreground)
         .map(Style::from)
         .unwrap_or_default()
}

pub fn default_color() -> Style {
   iconify_style(Fixed(244).normal())
}

pub trait FileColours: std::marker::Sync {
    fn color_file(&self, file: &File) -> Option<Style>;
    fn color_dir(&self, file: &File) -> Option<Style>;
}

impl FileColours for FileExtensions {
   fn color_file(&self, file: &File) -> Option<Style> {
      Some(match file {
         f if self.is_temp(f)           => Fixed(244).normal(),
         f if self.is_immediate(f)      => Fixed(1).bold().underline(),
         f if self.is_image(f)          => Fixed(37).normal(),
         f if self.is_video(f)          => Fixed(135).normal(),
         f if self.is_music(f)          => Fixed(92).normal(),
         f if self.is_lossless(f)       => Fixed(93).normal(),
         f if self.is_crypto(f)         => Fixed(109).normal(),
         f if self.is_document(f)       => Fixed(187).normal(),
         f if self.is_compressed(f)     => Red.normal(),
         f if self.is_compiled(f)       => Fixed(137).normal(),
         f if self.is_pretty_data(f)    => Fixed(178).normal(),
         f if self.is_script(f)         => Fixed(173).normal(),
         f if self.is_config(f)         => Fixed(65).normal(),
         f if self.is_vim(f)            => Fixed(71).normal(),
         f if self.is_language(f)       => Fixed(75).normal(),
         _                         => Fixed(244).bold(),
      })
   }

   fn color_dir(&self, file: &File) -> Option<Style> {
      Some(match file {
         f if self.is_config_folder(f)                 => Fixed(65).bold(),
         f if self.is_folder_with_language_files(f)    => Fixed(75).bold(),
         f if self.is_folder_with_exe_files(f)         => Fixed(173).bold(),
         f if self.is_folder_with_document_files(f)    => Fixed(187).bold(),
         _                                             => Fixed(244).bold(),
      })
   }

}