use self::{smf::SMF, smt::SMT};

pub mod smf;
pub mod smt;

pub fn extract_texture_32(smf: &SMF, smt: &SMT) -> Vec<u8> {
    let mm = 32_usize;

    let tile_columns = (smf.header.width / 4) as usize;
    let tile_rows = (smf.header.length / 4) as usize;

    let mut data = vec![0; ((tile_columns * mm * 4) * (tile_rows * mm)) as usize];
    let mut tile_i = 0;
    let mut max = usize::MIN;
    for row in 0..(tile_rows as usize) {
        for col in 0..(tile_columns as usize) {
            let tile_index = smf.smt_file_info.tile_index.0[tile_i];
            let dxt1 = &smt.tiles[tile_index as usize].mip_map32;

            let mut tile_buf = vec![0; mm * mm * 4];
            let format = texpresso::Format::Bc1;

            format.decompress(dxt1, mm, mm, &mut tile_buf);

            for (n, color) in tile_buf.chunks(4).enumerate() {
                let width = tile_columns * mm;
                let i = (row * width * mm * 4)
                    + ((n / mm) * width * 4)
                    + (col * mm * 4)
                    + ((n % mm) * 4);

                data[i + 0] = color[0];
                data[i + 1] = color[1];
                data[i + 2] = color[2];
                data[i + 3] = color[3];

                max = max.max(i);
            }
            tile_i += 1;
        }
    }

    data
}






#[cfg(test)]
mod test {
    use binrw::BinRead;
    use image::ImageFormat;

    use super::*;

    #[test]
    fn smf() {
        let smf_raw = include_bytes!("../.temp/great_divide_v1/maps/Great_Divide.smf");

        let mut c = std::io::Cursor::new(smf_raw);

        let smf = SMF::read(&mut c).unwrap();

        dbg!(smf.header);

        dbg!(smf.smt_file_info.header);
        dbg!(smf.smt_file_info.smt_file_info);
        dbg!(smf.features);
    }

    #[test]
    fn smt() {
        let smt_raw = include_bytes!("../.temp/great_divide_v1/maps/Great_Divide.smt");

        let mut c = std::io::Cursor::new(smt_raw);

        let smt = SMT::read(&mut c).unwrap();

        dbg!(smt.header);
        dbg!(smt.tiles.len());
    }

    #[test]
    fn both() {
        let smf = {
            let smf_raw =
                include_bytes!("../.temp/throne_v8/maps/Throne_v8.smf");

            let mut c = std::io::Cursor::new(smf_raw);

            SMF::read(&mut c).unwrap()
        };

        let smt = {
            let smt_raw =
                include_bytes!("../.temp/throne_v8/maps/Throne_v8.smt");
            let mut c = std::io::Cursor::new(smt_raw);
            SMT::read(&mut c).unwrap()
        };

        let data = extract_texture_32(&smf, &smt);

        image::save_buffer_with_format(
            format!("assets/.temp/tiles/test.png"),
            &data,
            smf.header.width / 4 * 32,
            smf.header.length / 4 * 32,
            image::ColorType::Rgba8,
            ImageFormat::Png,
        )
        .unwrap();
    }
}
