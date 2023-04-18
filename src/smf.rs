use binrw::{BinRead, NullString};
use std::io::{Read, Seek, SeekFrom};

/// Parse a stream into the SMF structure
/// 
/// # Example
/// 
/// ```
/// let file = std::fs::File::open("assets/Great_Divide.smf").unwrap();
/// let mut buf_reader = std::io::BufReader::new(file); 
/// 
/// let smf = spring_cartographer_rs::smf::parse_smf(&mut buf_reader).unwrap();
/// ```
pub fn parse_smf<R: Read + Seek>(reader: &mut R) -> Result<SMF, binrw::Error> {
    SMF::read(reader)
}

#[derive(BinRead, Debug, Clone)]
#[br(magic = b"spring map file\0", little)]
pub struct SMF {
    pub header: SMFHeader,

    #[br(count = header.extra_header_count)]
    pub extra_headers: Vec<ExtraHeaders>,

    #[br(
        args { width: header.width, length: header.length },
        seek_before = SeekFrom::Start(header.height_map_ptr as u64)
    )]
    pub height_map: HeightMap,

    #[br(
        args { width: header.width, length: header.length },
        seek_before = SeekFrom::Start(header.type_map_ptr as u64)
    )]
    pub type_map: TypeMap,

    #[br(seek_before = SeekFrom::Start(header.minimap_ptr as u64))]
    pub mini_map: MiniMap,

    #[br(
        args { width: header.width, length: header.length },
        seek_before = SeekFrom::Start(header.tiles_ptr as u64)
    )]
    pub smt_file_info: TileInfo,

    #[br(
        args { width: header.width, length: header.length },
        seek_before = SeekFrom::Start(header.metal_map_ptr as u64)
    )]
    pub metal_map: MetalMap,

    #[br(seek_before = SeekFrom::Start(header.feature_ptr as u64))]
    pub features: Features,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct SMFHeader {
    pub version: u32,
    pub id: u32,
    pub width: u32,
    pub length: u32,
    pub square_size: u32,
    pub texel_per_square: u32,
    pub tile_size: u32,
    pub min_height: f32,
    pub max_height: f32,
    pub height_map_ptr: u32,
    pub type_map_ptr: u32,
    pub tiles_ptr: u32,
    pub minimap_ptr: u32,
    pub metal_map_ptr: u32,
    pub feature_ptr: u32,
    pub extra_header_count: u32,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct ExtraHeaders {
    pub size_of_header: u32,
    pub type_of_header: u32,

    #[br(count = size_of_header)]
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug, Clone)]
#[br(import { length: u32, width: u32 })]
pub struct HeightMap(#[br(count = (width + 1) * (length + 1))] pub Vec<u16>);

#[derive(BinRead, Debug, Clone)]
#[br(import { length: u32, width: u32 })]
pub struct TypeMap(#[br(count = (width / 2)*(length / 2))] pub Vec<u8>);

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct MiniMap(#[br(count = 699048)] pub Vec<u8>); // raw DXT1 compressed 1024x1024 image.

#[derive(BinRead, Debug, Clone)]
#[br(import { length: u32, width: u32 })]
pub struct TileInfo {
    pub header: TileIndexHeader,

    #[br(count = header.number_of_tile_files)]
    pub smt_file_info: Vec<SMTFileInfos>,

    #[br(args { width: width, length: length })]
    pub tile_index: TileIndex,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct TileIndexHeader {
    pub number_of_tile_files: u32,
    pub total_number_of_tiles_in_all_files: u32,
}

#[derive(BinRead, Debug, Clone)]
pub struct SMTFileInfos {
    pub number_of_tiles_in_this_file: u32,
    pub smt_filename: NullString,
}

#[derive(BinRead, Debug, Clone)]
#[br(import { length: u32, width: u32 })]
pub struct TileIndex(#[br(count = (width / 4) * (length / 4))] pub Vec<u32>);

#[derive(BinRead, Debug, Clone)]
#[br(import { length: u32, width: u32 })]
pub struct MetalMap(#[br(count = (width / 2)*(length / 2))] pub Vec<u8>);

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct Features {
    pub header: FeaturesHeader,

    #[br(count = header.num_feature_types)]
    pub features: Vec<Feature>,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct FeaturesHeader {
    pub num_features: i32,
    pub num_feature_types: i32,

    #[br(count = num_features)]
    pub feature_names: Vec<NullString>,
}

#[derive(BinRead, Debug, Clone)]
#[br()]
pub struct Feature {
    pub feature_type: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub relative_size: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let file = std::fs::File::open("assets/Great_Divide.smf").unwrap();
        let mut buf_reader = std::io::BufReader::new(file);

        let smf = parse_smf(&mut buf_reader).unwrap();

        dbg!(smf.header);

        dbg!(smf.smt_file_info.header);
        dbg!(smf.smt_file_info.smt_file_info);
        dbg!(smf.features);
    }
}
