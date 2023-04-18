use binrw::BinRead;

#[derive(BinRead, Debug, Clone)]
#[br(magic = b"spring tilefile\0", little)]
pub struct SMT {
    pub header: SMTHeader,

    #[br(count = header.num_tiles)]
    pub tiles: Vec<Tile>,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct SMTHeader {
    pub version: i32,          // Should just be one
    pub num_tiles: u32,        //
    pub tile_size: u32,        // Should just be 32
    pub compression_type: i32, // Should always just be one
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct Tile {
    #[br(count = 512)]
    pub mip_map32: Vec<u8>,
    #[br(count = 128)]
    pub mip_map16: Vec<u8>,
    #[br(count = 32)]
    pub mip_map8: Vec<u8>,
    #[br(count = 8)]
    pub mip_map4: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test() {
    //     let smt_raw = include_bytes!("../../assets/.temp/great_divide_v1/maps/Great_Divide.smt");

    //     let mut c = std::io::Cursor::new(smt_raw);

    //     let smt = SMT::read(&mut c).unwrap();

    //     // dbg!(String::from_utf8_lossy(&smt.header.magic_number));
    //     dbg!(smt.header);
    //     dbg!(smt.tiles.len());
    // }
}
