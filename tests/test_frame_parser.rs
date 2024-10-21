#![allow(unused)]

#[cfg(test)]
mod tests {
    use pmu::frame_parser::{calculate_crc, parse_complete_frame};
    use pmu::frames::{DataFrame2011, HeaderFrame2011, HeaderFrame2024, TailFrame2011};
    use pmu::mocks::{
        mock_complete_frame2011, mock_dataframe2011, mock_header2011, mock_header2024,
        mock_invalid_frame, mock_tailframe2011,
    };

    #[test]
    fn test_header_parse() {
        let mock_header = mock_header2011();
        let header_decoded: HeaderFrame2011 = bincode::deserialize(&mock_header).unwrap();

        assert_eq!(header_decoded.fracsec, 456);
    }
    #[test]
    fn test_header2024_parse() {
        let mock_header = mock_header2024();
        let header_decoded: HeaderFrame2024 = bincode::deserialize(&mock_header).unwrap();
        assert_eq!(header_decoded.stream_id, 1234);
    }

    #[test]
    fn test_dataframe2011_parse() {
        let mock_data = mock_dataframe2011();
        let data_decoded: DataFrame2011 = bincode::deserialize(&mock_data).unwrap();
        assert!(data_decoded.stat > 0); // Just checking if it's not zero, as it's random
    }

    #[test]
    fn test_tailframe2011_parse() {
        let mock_tail = mock_tailframe2011();
        let tail_decoded: TailFrame2011 = bincode::deserialize(&mock_tail).unwrap();
        assert_eq!(tail_decoded.chk, 0xABCD);
    }

    #[test]
    fn test_complete_frame_parse() {
        let complete_frame = mock_complete_frame2011();
        // Here you would implement your frame parsing logic and test it
        // For example:
        let parsed_frame = parse_complete_frame(&complete_frame);
        assert!(parsed_frame.is_ok());
    }

    #[test]
    fn test_invalid_frame() {
        let invalid_frame = mock_invalid_frame();
        // Here you would test your error handling for invalid frames
        // For example:
        // let result = parse_complete_frame(&invalid_frame);
        // assert!(result.is_err());
    }
    #[test]
    fn test_parse_complete_frame() {
        let complete_frame = mock_complete_frame2011();
        let result = parse_complete_frame(&complete_frame);
        assert!(result.is_ok(), "Failed to parse complete frame");

        if let Ok((header, config, tail)) = result {
            // Add assertions to check the parsed data
            assert_eq!(header.framesize, complete_frame.len() as u16);
            // Add more assertions for config and tail...
        }
    }

    #[test]
    fn test_crc_calculation() {
        let complete_frame = mock_complete_frame2011();
        let framesize = u16::from_be_bytes([complete_frame[2], complete_frame[3]]) as usize;
        let calculated_crc = calculate_crc(&complete_frame[..framesize - 2]);
        let frame_crc =
            u16::from_be_bytes([complete_frame[framesize - 2], complete_frame[framesize - 1]]);
        assert_eq!(calculated_crc, frame_crc, "CRC mismatch");
    }
}