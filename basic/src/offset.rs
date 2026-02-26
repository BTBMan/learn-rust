pub fn main() {
    {
        #[derive(Debug)]
        pub struct Address {
            value: u64, // 对齐要求 = 8
            extra: [u8; 24],
        }

        #[repr(C)]
        struct Config {
            state: u8,     // offset 0 1 byte
            seed: [u8; 8], // offset 1 8 bytes
            // <-- 这里自动填充了 7 字节的 (9,10,11,12,13,14,15) padding (0 + 8 + 7 = 15)
            authority: Address, // offset 16 Address 要求对齐是 8 bytes
        }

        let user = Config {
            state: 0,
            seed: [0; 8],
            authority: Address {
                value: 8,
                extra: [0; 24],
            },
        };

        let authority = &user.authority;

        println!("{authority:?}");
        println!(
            "align_of::<Address>()   = {}",
            std::mem::align_of::<Address>()
        );
        println!(
            "offset of authority     = {}",
            std::mem::offset_of!(Config, authority)
        );
        println!(
            "size_of::<Config>()     = {}",
            std::mem::size_of::<Config>()
        );
    }
}
