use std::path::Path;

use fluidlite::FileApi;

pub struct EmbeddedFile{
    pos: usize,
    bytes: &'static [u8],
}

pub struct EmbeddedApi;

impl FileApi for EmbeddedApi {
    type File = EmbeddedFile;
    fn open(&mut self, _filename: &Path) -> Option<Self::File> {
        let bytes = include_bytes!("../../assets/sblive.sf2");
        Some(Self::File {
            pos: 0,
            bytes,
        })
    }

    fn read(file: &mut Self::File, buf: &mut [u8]) -> bool {
        let size = buf.len().min(file.bytes.len() - file.pos);
        for i in 0..size {
            buf[i] = file.bytes[file.pos + i];
        }
        //println!("EmbeddedApi::read {}/{} - {}", file.pos, file.bytes.len(), size);
        file.pos += size;
        size > 0
    }

    fn seek(file: &mut Self::File, pos: std::io::SeekFrom) -> bool {
        match pos {
            std::io::SeekFrom::Start(offset) => {
                if offset < file.bytes.len() as u64 {
                    file.pos = offset as usize;
                    //println!("EmbeddedApi::seek {:?} -> {}/{}", pos, file.pos, file.bytes.len());
                    return true;
                }
            },
            std::io::SeekFrom::End(offset) => {
                if offset <= 0 && offset > -(file.bytes.len() as i64) {
                    file.pos = ((file.bytes.len()) as i64 + offset) as usize;
                    //println!("EmbeddedApi::seek {:?} -> {}/{}", pos, file.pos, file.bytes.len());
                    return true;
                }
            },
            std::io::SeekFrom::Current(offset) => {
                if offset >= 0 {
                    if (file.pos + offset as usize) <= file.bytes.len() {
                        file.pos = file.pos + offset as usize;
                        //println!("EmbeddedApi::seek {:?} -> {}/{}", pos, file.pos, file.bytes.len());
                        return true;
                    }
                } else if (file.pos as i64 + offset) >= 0 {
                    file.pos = (file.pos as i64 + offset) as usize;
                    //println!("EmbeddedApi::seek {:?} -> {}/{}", pos, file.pos, file.bytes.len());
                    return true;
                }
            },
        }
        println!("EmbeddedApi::seek {:?} failed -> {}/{}", pos, file.pos, file.bytes.len());
        false
    }

    fn tell(file: &mut Self::File) -> Option<u64> {
        //println!("EmbeddedApi::tell -> {}/{}", file.pos, file.bytes.len());
        Some(file.pos as u64)
    }
}