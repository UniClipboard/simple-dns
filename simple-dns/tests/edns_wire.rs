use simple_dns::{rdata::OPT, Packet, RCODE};

fn response(ttl: u32, header_rcode: u8) -> Vec<u8> {
    let mut bytes = vec![
        0x12,
        0x34,
        0x80,
        header_rcode,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        41,
        4,
        208,
    ];
    bytes.extend_from_slice(&ttl.to_be_bytes());
    bytes.extend_from_slice(&[0, 0]);
    bytes
}

#[test]
fn reserved_flags_and_dnssec_do_do_not_change_rcode_or_version() {
    for flags in [1, 0x80, 0x100, 0x8000, 0xffff] {
        let bytes = response(flags, 0);
        let packet = Packet::parse(&bytes).unwrap();
        assert_eq!(packet.rcode(), RCODE::NoError, "flags={flags:#x}");
        assert_eq!(packet.opt().unwrap().version, 0, "flags={flags:#x}");
    }
}

#[test]
fn extended_rcode_uses_high_ttl_byte_and_preserves_header_bits() {
    for code in [0, 1, 10, 16, 17, 23, 0x120, 0xff0] {
        let bytes = response(((code >> 4) as u32) << 24, (code & 15) as u8);
        assert_eq!(Packet::parse(&bytes).unwrap().rcode(), RCODE::from(code));
    }
}

#[test]
fn version_uses_second_ttl_byte() {
    for version in [1, 2, 0xff] {
        let bytes = response((version as u32) << 16, 0);
        let packet = Packet::parse(&bytes).unwrap();
        assert_eq!(packet.opt().unwrap().version, version);
        assert_eq!(packet.rcode(), RCODE::NoError);
    }
}

#[test]
fn encoded_extended_rcode_and_version_match_wire_format() {
    for code in [0, 1, 10, 16, 17] {
        for version in [0, 1, 0xff] {
            let mut packet = Packet::new_reply(0x1234);
            *packet.rcode_mut() = RCODE::from(code);
            *packet.opt_mut() = Some(OPT {
                opt_codes: Vec::new(),
                udp_packet_size: 1232,
                version,
            });
            let bytes = packet.build_bytes_vec().unwrap();
            assert_eq!(bytes[3] & 15, (code & 15) as u8);
            assert_eq!(&bytes[17..21], &[((code >> 4) & 0xff) as u8, version, 0, 0]);
            let parsed = Packet::parse(&bytes).unwrap();
            assert_eq!(parsed.rcode(), RCODE::from(code));
            assert_eq!(parsed.opt().unwrap().version, version);
        }
    }
}

#[test]
fn no_opt_preserves_ordinary_response_codes() {
    for code in 0..=15 {
        let bytes = [0x12, 0x34, 0x80, code, 0, 0, 0, 0, 0, 0, 0, 0];
        let packet = Packet::parse(&bytes).unwrap();
        assert_eq!(packet.rcode(), RCODE::from(code as u16));
        assert!(packet.opt().is_none());
    }
}

#[test]
fn captured_windows_response_is_successful() {
    let hex = "12-34-85-80-00-01-00-01-00-00-00-01-06-75-73-65-31-2D-31-05-72-65-6C-61-79-02-6E-30-04-69-72-6F-68-04-6C-69-6E-6B-00-00-01-00-01-C0-0C-00-01-00-01-00-00-00-01-00-04-C6-12-00-19-00-00-29-04-D0-00-00-00-01-00-00";
    let bytes: Vec<u8> = hex
        .split('-')
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .collect();
    let packet = Packet::parse(&bytes).unwrap();
    assert_eq!(packet.rcode(), RCODE::NoError);
    assert_eq!(packet.answers.len(), 1);
    assert_eq!(packet.opt().unwrap().version, 0);
}
