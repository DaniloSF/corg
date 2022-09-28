use std::{
    fs::File,
    io::{BufReader, Read},
};

#[macro_export]
macro_rules! r_lock {
    ($state: expr) => {
        $state.store.lock().unwrap()
    };
}

trait RoaBufferRead {
    fn buffer_read(buf: &mut BufReader<&File>) -> Self;
}

impl RoaBufferRead for u8 {
    fn buffer_read(buf: &mut BufReader<&File>) -> u8 {
        let buffer: &mut [u8; 1] = &mut [0; 1];
        if !buf.read_exact(buffer).is_ok() {
            return 0;
        }
        let result: u8 = u8::from_le_bytes(*buffer);
        result
    }
}
impl RoaBufferRead for u16 {
    fn buffer_read(buf: &mut BufReader<&File>) -> u16 {
        let buffer = &mut [0; 2];
        if !buf.read_exact(buffer).is_ok() {
            return 0;
        }
        // println!("{:?}", buffer);
        let result: u16 = u16::from_le_bytes(*buffer);
        result
    }
}
impl RoaBufferRead for String {
    fn buffer_read(buf: &mut BufReader<&File>) -> String {
        let mut result = String::new();
        let buffer = &mut [0; 1];
        loop {
            if !buf.read_exact(buffer).is_ok() || buffer[0] == 0 {
                break;
            }
            result.push(buffer[0] as char);
        }

        result
    }
}


pub fn parse_order_section(buffer: &mut BufReader<&File>) -> Vec<String> {
    let o_magic: String = RoaBufferRead::buffer_read(buffer.by_ref());

    if o_magic != "order.roa" {
        panic!("Invalid file format {}", o_magic);
    }

    let junk_bool: u8 = RoaBufferRead::buffer_read(buffer.by_ref());
    if junk_bool != 1 {
        panic!("Unexpected value at parsing order.roa");
    }

    // read first size order.roa with the characters
    let char_size: u16 = RoaBufferRead::buffer_read(buffer.by_ref());
    // println!("char_size: {}", char_size);

    // Junk null
    let junk_bool: u16 = RoaBufferRead::buffer_read(buffer.by_ref());
    if junk_bool != 0 {
        panic!("Unexpected value at parsing order.roa");
    }

    let mut paths: Vec<String> = Vec::new();
    if char_size != 0 {
        for _i in 0..char_size {
            let path: String = RoaBufferRead::buffer_read(buffer.by_ref());
            paths.push(path);
        }
    }
    paths
}

pub fn parse_categories_section(f: &File) -> Vec<(String, u16)> {
    let mut buffer = BufReader::new(f);
    let mut cat_offset: u16;
    let cats_amount: u16 = RoaBufferRead::buffer_read(buffer.by_ref());
    let mut cats: Vec<(String, u16)> = Vec::new();
    for _ in 0..cats_amount {
        cat_offset = RoaBufferRead::buffer_read(buffer.by_ref());
        let cat_name: String = RoaBufferRead::buffer_read(buffer.by_ref());
        cats.push((cat_name, cat_offset));
    }
    cats
}
