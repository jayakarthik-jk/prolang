use std::{collections::VecDeque, io::Read, sync::mpsc::Sender};

pub(crate) struct FileReader {
    file: String,
    file_chunk_transmitter: Sender<VecDeque<u8>>,
}

impl FileReader {
    pub(crate) fn new(
        file_name: impl Into<String>,
        file_chunk_transmitter: Sender<VecDeque<u8>>,
    ) -> Self {
        Self {
            file: file_name.into(),
            file_chunk_transmitter,
        }
    }

    pub(crate) fn read(&self) {
        const BUFFER_SIZE: usize = 1024;
        let file = std::fs::File::open(&self.file).expect("Unable to open file");
        let mut reader = std::io::BufReader::new(file);
        let mut buf = [0; BUFFER_SIZE];
        while let Ok(count) = reader.read(&mut buf) {
            if count == 0 {
                break;
            }
            self.file_chunk_transmitter.send(buf.into()).unwrap();
            buf = [0; BUFFER_SIZE];
        }
    }
}
