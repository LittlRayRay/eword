pub enum FileType {
    Txt,
    Epub,
    Pdf,
}

pub struct File {
    pub filename: String,
    pub file_type: FileType,
}
