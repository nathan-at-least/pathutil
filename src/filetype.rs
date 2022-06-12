use std::fs::FileType;

#[derive(Debug, PartialEq, Eq)]
pub enum FileTypeEnum {
    Dir,
    File,
    Symlink,
}

impl From<FileType> for FileTypeEnum {
    fn from(ftype: FileType) -> Self {
        use FileTypeEnum::*;

        if ftype.is_dir() {
            Dir
        } else if ftype.is_file() {
            File
        } else if ftype.is_symlink() {
            Symlink
        } else {
            unreachable!("Incoherent {:?}", ftype);
        }
    }
}
