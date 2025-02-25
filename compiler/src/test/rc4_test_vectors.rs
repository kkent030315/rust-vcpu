const RFC6229_1_KEY: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
const RFC6229_2_KEY: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
const RFC6229_3_KEY: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
const RFC6229_4_KEY: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
const RFC6229_5_KEY: &[u8] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
];
const RFC6229_6_KEY: &[u8] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
];
const RFC6229_7_KEY: &[u8] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
];
const RFC6229_8_KEY: &[u8] = &[0x83, 0x32, 0x22, 0x77, 0x2a];
const RFC6229_9_KEY: &[u8] = &[0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a];
const RFC6229_10_KEY: &[u8] = &[0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a];
const RFC6229_11_KEY: &[u8] = &[0x8b, 0x37, 0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a];
const RFC6229_12_KEY: &[u8] = &[
    0xeb, 0xb4, 0x62, 0x27, 0xc6, 0xcc, 0x8b, 0x37, 0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a,
];
const RFC6229_13_KEY: &[u8] = &[
    0xc1, 0x09, 0x16, 0x39, 0x08, 0xeb, 0xe5, 0x1d, 0xeb, 0xb4, 0x62, 0x27, 0xc6, 0xcc, 0x8b, 0x37,
    0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a,
];
const RFC6229_14_KEY: &[u8] = &[
    0x1a, 0xda, 0x31, 0xd5, 0xcf, 0x68, 0x82, 0x21, 0xc1, 0x09, 0x16, 0x39, 0x08, 0xeb, 0xe5, 0x1d,
    0xeb, 0xb4, 0x62, 0x27, 0xc6, 0xcc, 0x8b, 0x37, 0x64, 0x19, 0x10, 0x83, 0x32, 0x22, 0x77, 0x2a,
];

const RFC6229_1_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xb2, 0x39, 0x63, 0x05, 0xf0, 0x3d, 0xc0, 0x27, 0xcc, 0xc3, 0x52, 0x4a, 0x0a, 0x11, 0x18,
        0xa8, 0x69, 0x82, 0x94, 0x4f, 0x18, 0xfc, 0x82, 0xd5, 0x89, 0xc4, 0x03, 0xa4, 0x7a, 0x0d,
        0x09, 0x19,
    ],
    &[
        0x28, 0xcb, 0x11, 0x32, 0xc9, 0x6c, 0xe2, 0x86, 0x42, 0x1d, 0xca, 0xad, 0xb8, 0xb6, 0x9e,
        0xae, 0x1c, 0xfc, 0xf6, 0x2b, 0x03, 0xed, 0xdb, 0x64, 0x1d, 0x77, 0xdf, 0xcf, 0x7f, 0x8d,
        0x8c, 0x93,
    ],
    &[
        0x42, 0xb7, 0xd0, 0xcd, 0xd9, 0x18, 0xa8, 0xa3, 0x3d, 0xd5, 0x17, 0x81, 0xc8, 0x1f, 0x40,
        0x41, 0x64, 0x59, 0x84, 0x44, 0x32, 0xa7, 0xda, 0x92, 0x3c, 0xfb, 0x3e, 0xb4, 0x98, 0x06,
        0x61, 0xf6,
    ],
    &[
        0xec, 0x10, 0x32, 0x7b, 0xde, 0x2b, 0xee, 0xfd, 0x18, 0xf9, 0x27, 0x76, 0x80, 0x45, 0x7e,
        0x22, 0xeb, 0x62, 0x63, 0x8d, 0x4f, 0x0b, 0xa1, 0xfe, 0x9f, 0xca, 0x20, 0xe0, 0x5b, 0xf8,
        0xff, 0x2b,
    ],
    &[
        0x45, 0x12, 0x90, 0x48, 0xe6, 0xa0, 0xed, 0x0b, 0x56, 0xb4, 0x90, 0x33, 0x8f, 0x07, 0x8d,
        0xa5, 0x30, 0xab, 0xbc, 0xc7, 0xc2, 0x0b, 0x01, 0x60, 0x9f, 0x23, 0xee, 0x2d, 0x5f, 0x6b,
        0xb7, 0xdf,
    ],
    &[
        0x32, 0x94, 0xf7, 0x44, 0xd8, 0xf9, 0x79, 0x05, 0x07, 0xe7, 0x0f, 0x62, 0xe5, 0xbb, 0xce,
        0xea, 0xd8, 0x72, 0x9d, 0xb4, 0x18, 0x82, 0x25, 0x9b, 0xee, 0x4f, 0x82, 0x53, 0x25, 0xf5,
        0xa1, 0x30,
    ],
    &[
        0x1e, 0xb1, 0x4a, 0x0c, 0x13, 0xb3, 0xbf, 0x47, 0xfa, 0x2a, 0x0b, 0xa9, 0x3a, 0xd4, 0x5b,
        0x8b, 0xcc, 0x58, 0x2f, 0x8b, 0xa9, 0xf2, 0x65, 0xe2, 0xb1, 0xbe, 0x91, 0x12, 0xe9, 0x75,
        0xd2, 0xd7,
    ],
    &[
        0xf2, 0xe3, 0x0f, 0x9b, 0xd1, 0x02, 0xec, 0xbf, 0x75, 0xaa, 0xad, 0xe9, 0xbc, 0x35, 0xc4,
        0x3c, 0xec, 0x0e, 0x11, 0xc4, 0x79, 0xdc, 0x32, 0x9d, 0xc8, 0xda, 0x79, 0x68, 0xfe, 0x96,
        0x56, 0x81,
    ],
    &[
        0x06, 0x83, 0x26, 0xa2, 0x11, 0x84, 0x16, 0xd2, 0x1f, 0x9d, 0x04, 0xb2, 0xcd, 0x1c, 0xa0,
        0x50, 0xff, 0x25, 0xb5, 0x89, 0x95, 0x99, 0x67, 0x07, 0xe5, 0x1f, 0xbd, 0xf0, 0x8b, 0x34,
        0xd8, 0x75,
    ],
];
const RFC6229_2_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x29, 0x3f, 0x02, 0xd4, 0x7f, 0x37, 0xc9, 0xb6, 0x33, 0xf2, 0xaf, 0x52, 0x85, 0xfe, 0xb4,
        0x6b, 0xe6, 0x20, 0xf1, 0x39, 0x0d, 0x19, 0xbd, 0x84, 0xe2, 0xe0, 0xfd, 0x75, 0x20, 0x31,
        0xaf, 0xc1,
    ],
    &[
        0x91, 0x4f, 0x02, 0x53, 0x1c, 0x92, 0x18, 0x81, 0x0d, 0xf6, 0x0f, 0x67, 0xe3, 0x38, 0x15,
        0x4c, 0xd0, 0xfd, 0xb5, 0x83, 0x07, 0x3c, 0xe8, 0x5a, 0xb8, 0x39, 0x17, 0x74, 0x0e, 0xc0,
        0x11, 0xd5,
    ],
    &[
        0x75, 0xf8, 0x14, 0x11, 0xe8, 0x71, 0xcf, 0xfa, 0x70, 0xb9, 0x0c, 0x74, 0xc5, 0x92, 0xe4,
        0x54, 0x0b, 0xb8, 0x72, 0x02, 0x93, 0x8d, 0xad, 0x60, 0x9e, 0x87, 0xa5, 0xa1, 0xb0, 0x79,
        0xe5, 0xe4,
    ],
    &[
        0xc2, 0x91, 0x12, 0x46, 0xb6, 0x12, 0xe7, 0xe7, 0xb9, 0x03, 0xdf, 0xed, 0xa1, 0xda, 0xd8,
        0x66, 0x32, 0x82, 0x8f, 0x91, 0x50, 0x2b, 0x62, 0x91, 0x36, 0x8d, 0xe8, 0x08, 0x1d, 0xe3,
        0x6f, 0xc2,
    ],
    &[
        0xf3, 0xb9, 0xa7, 0xe3, 0xb2, 0x97, 0xbf, 0x9a, 0xd8, 0x04, 0x51, 0x2f, 0x90, 0x63, 0xef,
        0xf1, 0x8e, 0xcb, 0x67, 0xa9, 0xba, 0x1f, 0x55, 0xa5, 0xa0, 0x67, 0xe2, 0xb0, 0x26, 0xa3,
        0x67, 0x6f,
    ],
    &[
        0xd2, 0xaa, 0x90, 0x2b, 0xd4, 0x2d, 0x0d, 0x7c, 0xfd, 0x34, 0x0c, 0xd4, 0x58, 0x10, 0x52,
        0x9f, 0x78, 0xb2, 0x72, 0xc9, 0x6e, 0x42, 0xea, 0xb4, 0xc6, 0x0b, 0xd9, 0x14, 0xe3, 0x9d,
        0x06, 0xe3,
    ],
    &[
        0xf4, 0x33, 0x2f, 0xd3, 0x1a, 0x07, 0x93, 0x96, 0xee, 0x3c, 0xee, 0x3f, 0x2a, 0x4f, 0xf0,
        0x49, 0x05, 0x45, 0x97, 0x81, 0xd4, 0x1f, 0xda, 0x7f, 0x30, 0xc1, 0xbe, 0x7e, 0x12, 0x46,
        0xc6, 0x23,
    ],
    &[
        0xad, 0xfd, 0x38, 0x68, 0xb8, 0xe5, 0x14, 0x85, 0xd5, 0xe6, 0x10, 0x01, 0x7e, 0x3d, 0xd6,
        0x09, 0xad, 0x26, 0x58, 0x1c, 0x0c, 0x5b, 0xe4, 0x5f, 0x4c, 0xea, 0x01, 0xdb, 0x2f, 0x38,
        0x05, 0xd5,
    ],
    &[
        0xf3, 0x17, 0x2c, 0xef, 0xfc, 0x3b, 0x3d, 0x99, 0x7c, 0x85, 0xcc, 0xd5, 0xaf, 0x1a, 0x95,
        0x0c, 0xe7, 0x4b, 0x0b, 0x97, 0x31, 0x22, 0x7f, 0xd3, 0x7c, 0x0e, 0xc0, 0x8a, 0x47, 0xdd,
        0xd8, 0xb8,
    ],
];
const RFC6229_3_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x97, 0xab, 0x8a, 0x1b, 0xf0, 0xaf, 0xb9, 0x61, 0x32, 0xf2, 0xf6, 0x72, 0x58, 0xda, 0x15,
        0xa8, 0x82, 0x63, 0xef, 0xdb, 0x45, 0xc4, 0xa1, 0x86, 0x84, 0xef, 0x87, 0xe6, 0xb1, 0x9e,
        0x5b, 0x09,
    ],
    &[
        0x96, 0x36, 0xeb, 0xc9, 0x84, 0x19, 0x26, 0xf4, 0xf7, 0xd1, 0xf3, 0x62, 0xbd, 0xdf, 0x6e,
        0x18, 0xd0, 0xa9, 0x90, 0xff, 0x2c, 0x05, 0xfe, 0xf5, 0xb9, 0x03, 0x73, 0xc9, 0xff, 0x4b,
        0x87, 0x0a,
    ],
    &[
        0x73, 0x23, 0x9f, 0x1d, 0xb7, 0xf4, 0x1d, 0x80, 0xb6, 0x43, 0xc0, 0xc5, 0x25, 0x18, 0xec,
        0x63, 0x16, 0x3b, 0x31, 0x99, 0x23, 0xa6, 0xbd, 0xb4, 0x52, 0x7c, 0x62, 0x61, 0x26, 0x70,
        0x3c, 0x0f,
    ],
    &[
        0x49, 0xd6, 0xc8, 0xaf, 0x0f, 0x97, 0x14, 0x4a, 0x87, 0xdf, 0x21, 0xd9, 0x14, 0x72, 0xf9,
        0x66, 0x44, 0x17, 0x3a, 0x10, 0x3b, 0x66, 0x16, 0xc5, 0xd5, 0xad, 0x1c, 0xee, 0x40, 0xc8,
        0x63, 0xd0,
    ],
    &[
        0x27, 0x3c, 0x9c, 0x4b, 0x27, 0xf3, 0x22, 0xe4, 0xe7, 0x16, 0xef, 0x53, 0xa4, 0x7d, 0xe7,
        0xa4, 0xc6, 0xd0, 0xe7, 0xb2, 0x26, 0x25, 0x9f, 0xa9, 0x02, 0x34, 0x90, 0xb2, 0x61, 0x67,
        0xad, 0x1d,
    ],
    &[
        0x1f, 0xe8, 0x98, 0x67, 0x13, 0xf0, 0x7c, 0x3d, 0x9a, 0xe1, 0xc1, 0x63, 0xff, 0x8c, 0xf9,
        0xd3, 0x83, 0x69, 0xe1, 0xa9, 0x65, 0x61, 0x0b, 0xe8, 0x87, 0xfb, 0xd0, 0xc7, 0x91, 0x62,
        0xaa, 0xfb,
    ],
    &[
        0x0a, 0x01, 0x27, 0xab, 0xb4, 0x44, 0x84, 0xb9, 0xfb, 0xef, 0x5a, 0xbc, 0xae, 0x1b, 0x57,
        0x9f, 0xc2, 0xcd, 0xad, 0xc6, 0x40, 0x2e, 0x8e, 0xe8, 0x66, 0xe1, 0xf3, 0x7b, 0xdb, 0x47,
        0xe4, 0x2c,
    ],
    &[
        0x26, 0xb5, 0x1e, 0xa3, 0x7d, 0xf8, 0xe1, 0xd6, 0xf7, 0x6f, 0xc3, 0xb6, 0x6a, 0x74, 0x29,
        0xb3, 0xbc, 0x76, 0x83, 0x20, 0x5d, 0x4f, 0x44, 0x3d, 0xc1, 0xf2, 0x9d, 0xda, 0x33, 0x15,
        0xc8, 0x7b,
    ],
    &[
        0xd5, 0xfa, 0x5a, 0x34, 0x69, 0xd2, 0x9a, 0xaa, 0xf8, 0x3d, 0x23, 0x58, 0x9d, 0xb8, 0xc8,
        0x5b, 0x3f, 0xb4, 0x6e, 0x2c, 0x8f, 0x0f, 0x06, 0x8e, 0xdc, 0xe8, 0xcd, 0xcd, 0x7d, 0xfc,
        0x58, 0x62,
    ],
];
const RFC6229_4_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xed, 0xe3, 0xb0, 0x46, 0x43, 0xe5, 0x86, 0xcc, 0x90, 0x7d, 0xc2, 0x18, 0x51, 0x70, 0x99,
        0x02, 0x03, 0x51, 0x6b, 0xa7, 0x8f, 0x41, 0x3b, 0xeb, 0x22, 0x3a, 0xa5, 0xd4, 0xd2, 0xdf,
        0x67, 0x11,
    ],
    &[
        0x3c, 0xfd, 0x6c, 0xb5, 0x8e, 0xe0, 0xfd, 0xde, 0x64, 0x01, 0x76, 0xad, 0x00, 0x00, 0x04,
        0x4d, 0x48, 0x53, 0x2b, 0x21, 0xfb, 0x60, 0x79, 0xc9, 0x11, 0x4c, 0x0f, 0xfd, 0x9c, 0x04,
        0xa1, 0xad,
    ],
    &[
        0x3e, 0x8c, 0xea, 0x98, 0x01, 0x71, 0x09, 0x97, 0x90, 0x84, 0xb1, 0xef, 0x92, 0xf9, 0x9d,
        0x86, 0xe2, 0x0f, 0xb4, 0x9b, 0xdb, 0x33, 0x7e, 0xe4, 0x8b, 0x8d, 0x8d, 0xc0, 0xf4, 0xaf,
        0xef, 0xfe,
    ],
    &[
        0x5c, 0x25, 0x21, 0xea, 0xcd, 0x79, 0x66, 0xf1, 0x5e, 0x05, 0x65, 0x44, 0xbe, 0xa0, 0xd3,
        0x15, 0xe0, 0x67, 0xa7, 0x03, 0x19, 0x31, 0xa2, 0x46, 0xa6, 0xc3, 0x87, 0x5d, 0x2f, 0x67,
        0x8a, 0xcb,
    ],
    &[
        0xa6, 0x4f, 0x70, 0xaf, 0x88, 0xae, 0x56, 0xb6, 0xf8, 0x75, 0x81, 0xc0, 0xe2, 0x3e, 0x6b,
        0x08, 0xf4, 0x49, 0x03, 0x1d, 0xe3, 0x12, 0x81, 0x4e, 0xc6, 0xf3, 0x19, 0x29, 0x1f, 0x4a,
        0x05, 0x16,
    ],
    &[
        0xbd, 0xae, 0x85, 0x92, 0x4b, 0x3c, 0xb1, 0xd0, 0xa2, 0xe3, 0x3a, 0x30, 0xc6, 0xd7, 0x95,
        0x99, 0x8a, 0x0f, 0xed, 0xdb, 0xac, 0x86, 0x5a, 0x09, 0xbc, 0xd1, 0x27, 0xfb, 0x56, 0x2e,
        0xd6, 0x0a,
    ],
    &[
        0xb5, 0x5a, 0x0a, 0x5b, 0x51, 0xa1, 0x2a, 0x8b, 0xe3, 0x48, 0x99, 0xc3, 0xe0, 0x47, 0x51,
        0x1a, 0xd9, 0xa0, 0x9c, 0xea, 0x3c, 0xe7, 0x5f, 0xe3, 0x96, 0x98, 0x07, 0x03, 0x17, 0xa7,
        0x13, 0x39,
    ],
    &[
        0x55, 0x22, 0x25, 0xed, 0x11, 0x77, 0xf4, 0x45, 0x84, 0xac, 0x8c, 0xfa, 0x6c, 0x4e, 0xb5,
        0xfc, 0x7e, 0x82, 0xcb, 0xab, 0xfc, 0x95, 0x38, 0x1b, 0x08, 0x09, 0x98, 0x44, 0x21, 0x29,
        0xc2, 0xf8,
    ],
    &[
        0x1f, 0x13, 0x5e, 0xd1, 0x4c, 0xe6, 0x0a, 0x91, 0x36, 0x9d, 0x23, 0x22, 0xbe, 0xf2, 0x5e,
        0x3c, 0x08, 0xb6, 0xbe, 0x45, 0x12, 0x4a, 0x43, 0xe2, 0xeb, 0x77, 0x95, 0x3f, 0x84, 0xdc,
        0x85, 0x53,
    ],
];
const RFC6229_5_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x9a, 0xc7, 0xcc, 0x9a, 0x60, 0x9d, 0x1e, 0xf7, 0xb2, 0x93, 0x28, 0x99, 0xcd, 0xe4, 0x1b,
        0x97, 0x52, 0x48, 0xc4, 0x95, 0x90, 0x14, 0x12, 0x6a, 0x6e, 0x8a, 0x84, 0xf1, 0x1d, 0x1a,
        0x9e, 0x1c,
    ],
    &[
        0x06, 0x59, 0x02, 0xe4, 0xb6, 0x20, 0xf6, 0xcc, 0x36, 0xc8, 0x58, 0x9f, 0x66, 0x43, 0x2f,
        0x2b, 0xd3, 0x9d, 0x56, 0x6b, 0xc6, 0xbc, 0xe3, 0x01, 0x07, 0x68, 0x15, 0x15, 0x49, 0xf3,
        0x87, 0x3f,
    ],
    &[
        0xb6, 0xd1, 0xe6, 0xc4, 0xa5, 0xe4, 0x77, 0x1c, 0xad, 0x79, 0x53, 0x8d, 0xf2, 0x95, 0xfb,
        0x11, 0xc6, 0x8c, 0x1d, 0x5c, 0x55, 0x9a, 0x97, 0x41, 0x23, 0xdf, 0x1d, 0xbc, 0x52, 0xa4,
        0x3b, 0x89,
    ],
    &[
        0xc5, 0xec, 0xf8, 0x8d, 0xe8, 0x97, 0xfd, 0x57, 0xfe, 0xd3, 0x01, 0x70, 0x1b, 0x82, 0xa2,
        0x59, 0xec, 0xcb, 0xe1, 0x3d, 0xe1, 0xfc, 0xc9, 0x1c, 0x11, 0xa0, 0xb2, 0x6c, 0x0b, 0xc8,
        0xfa, 0x4d,
    ],
    &[
        0xe7, 0xa7, 0x25, 0x74, 0xf8, 0x78, 0x2a, 0xe2, 0x6a, 0xab, 0xcf, 0x9e, 0xbc, 0xd6, 0x60,
        0x65, 0xbd, 0xf0, 0x32, 0x4e, 0x60, 0x83, 0xdc, 0xc6, 0xd3, 0xce, 0xdd, 0x3c, 0xa8, 0xc5,
        0x3c, 0x16,
    ],
    &[
        0xb4, 0x01, 0x10, 0xc4, 0x19, 0x0b, 0x56, 0x22, 0xa9, 0x61, 0x16, 0xb0, 0x01, 0x7e, 0xd2,
        0x97, 0xff, 0xa0, 0xb5, 0x14, 0x64, 0x7e, 0xc0, 0x4f, 0x63, 0x06, 0xb8, 0x92, 0xae, 0x66,
        0x11, 0x81,
    ],
    &[
        0xd0, 0x3d, 0x1b, 0xc0, 0x3c, 0xd3, 0x3d, 0x70, 0xdf, 0xf9, 0xfa, 0x5d, 0x71, 0x96, 0x3e,
        0xbd, 0x8a, 0x44, 0x12, 0x64, 0x11, 0xea, 0xa7, 0x8b, 0xd5, 0x1e, 0x8d, 0x87, 0xa8, 0x87,
        0x9b, 0xf5,
    ],
    &[
        0xfa, 0xbe, 0xb7, 0x60, 0x28, 0xad, 0xe2, 0xd0, 0xe4, 0x87, 0x22, 0xe4, 0x6c, 0x46, 0x15,
        0xa3, 0xc0, 0x5d, 0x88, 0xab, 0xd5, 0x03, 0x57, 0xf9, 0x35, 0xa6, 0x3c, 0x59, 0xee, 0x53,
        0x76, 0x23,
    ],
    &[
        0xff, 0x38, 0x26, 0x5c, 0x16, 0x42, 0xc1, 0xab, 0xe8, 0xd3, 0xc2, 0xfe, 0x5e, 0x57, 0x2b,
        0xf8, 0xa3, 0x6a, 0x4c, 0x30, 0x1a, 0xe8, 0xac, 0x13, 0x61, 0x0c, 0xcb, 0xc1, 0x22, 0x56,
        0xca, 0xcc,
    ],
];
const RFC6229_6_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x05, 0x95, 0xe5, 0x7f, 0xe5, 0xf0, 0xbb, 0x3c, 0x70, 0x6e, 0xda, 0xc8, 0xa4, 0xb2, 0xdb,
        0x11, 0xdf, 0xde, 0x31, 0x34, 0x4a, 0x1a, 0xf7, 0x69, 0xc7, 0x4f, 0x07, 0x0a, 0xee, 0x9e,
        0x23, 0x26,
    ],
    &[
        0xb0, 0x6b, 0x9b, 0x1e, 0x19, 0x5d, 0x13, 0xd8, 0xf4, 0xa7, 0x99, 0x5c, 0x45, 0x53, 0xac,
        0x05, 0x6b, 0xd2, 0x37, 0x8e, 0xc3, 0x41, 0xc9, 0xa4, 0x2f, 0x37, 0xba, 0x79, 0xf8, 0x8a,
        0x32, 0xff,
    ],
    &[
        0xe7, 0x0b, 0xce, 0x1d, 0xf7, 0x64, 0x5a, 0xdb, 0x5d, 0x2c, 0x41, 0x30, 0x21, 0x5c, 0x35,
        0x22, 0x9a, 0x57, 0x30, 0xc7, 0xfc, 0xb4, 0xc9, 0xaf, 0x51, 0xff, 0xda, 0x89, 0xc7, 0xf1,
        0xad, 0x22,
    ],
    &[
        0x04, 0x85, 0x05, 0x5f, 0xd4, 0xf6, 0xf0, 0xd9, 0x63, 0xef, 0x5a, 0xb9, 0xa5, 0x47, 0x69,
        0x82, 0x59, 0x1f, 0xc6, 0x6b, 0xcd, 0xa1, 0x0e, 0x45, 0x2b, 0x03, 0xd4, 0x55, 0x1f, 0x6b,
        0x62, 0xac,
    ],
    &[
        0x27, 0x53, 0xcc, 0x83, 0x98, 0x8a, 0xfa, 0x3e, 0x16, 0x88, 0xa1, 0xd3, 0xb4, 0x2c, 0x9a,
        0x02, 0x93, 0x61, 0x0d, 0x52, 0x3d, 0x1d, 0x3f, 0x00, 0x62, 0xb3, 0xc2, 0xa3, 0xbb, 0xc7,
        0xc7, 0xf0,
    ],
    &[
        0x96, 0xc2, 0x48, 0x61, 0x0a, 0xad, 0xed, 0xfe, 0xaf, 0x89, 0x78, 0xc0, 0x3d, 0xe8, 0x20,
        0x5a, 0x0e, 0x31, 0x7b, 0x3d, 0x1c, 0x73, 0xb9, 0xe9, 0xa4, 0x68, 0x8f, 0x29, 0x6d, 0x13,
        0x3a, 0x19,
    ],
    &[
        0xbd, 0xf0, 0xe6, 0xc3, 0xcc, 0xa5, 0xb5, 0xb9, 0xd5, 0x33, 0xb6, 0x9c, 0x56, 0xad, 0xa1,
        0x20, 0x88, 0xa2, 0x18, 0xb6, 0xe2, 0xec, 0xe1, 0xe6, 0x24, 0x6d, 0x44, 0xc7, 0x59, 0xd1,
        0x9b, 0x10,
    ],
    &[
        0x68, 0x66, 0x39, 0x7e, 0x95, 0xc1, 0x40, 0x53, 0x4f, 0x94, 0x26, 0x34, 0x21, 0x00, 0x6e,
        0x40, 0x32, 0xcb, 0x0a, 0x1e, 0x95, 0x42, 0xc6, 0xb3, 0xb8, 0xb3, 0x98, 0xab, 0xc3, 0xb0,
        0xf1, 0xd5,
    ],
    &[
        0x29, 0xa0, 0xb8, 0xae, 0xd5, 0x4a, 0x13, 0x23, 0x24, 0xc6, 0x2e, 0x42, 0x3f, 0x54, 0xb4,
        0xc8, 0x3c, 0xb0, 0xf3, 0xb5, 0x02, 0x0a, 0x98, 0xb8, 0x2a, 0xf9, 0xfe, 0x15, 0x44, 0x84,
        0xa1, 0x68,
    ],
];
const RFC6229_7_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xea, 0xa6, 0xbd, 0x25, 0x88, 0x0b, 0xf9, 0x3d, 0x3f, 0x5d, 0x1e, 0x4c, 0xa2, 0x61, 0x1d,
        0x91, 0xcf, 0xa4, 0x5c, 0x9f, 0x7e, 0x71, 0x4b, 0x54, 0xbd, 0xfa, 0x80, 0x02, 0x7c, 0xb1,
        0x43, 0x80,
    ],
    &[
        0x11, 0x4a, 0xe3, 0x44, 0xde, 0xd7, 0x1b, 0x35, 0xf2, 0xe6, 0x0f, 0xeb, 0xad, 0x72, 0x7f,
        0xd8, 0x02, 0xe1, 0xe7, 0x05, 0x6b, 0x0f, 0x62, 0x39, 0x00, 0x49, 0x64, 0x22, 0x94, 0x3e,
        0x97, 0xb6,
    ],
    &[
        0x91, 0xcb, 0x93, 0xc7, 0x87, 0x96, 0x4e, 0x10, 0xd9, 0x52, 0x7d, 0x99, 0x9c, 0x6f, 0x93,
        0x6b, 0x49, 0xb1, 0x8b, 0x42, 0xf8, 0xe8, 0x36, 0x7c, 0xbe, 0xb5, 0xef, 0x10, 0x4b, 0xa1,
        0xc7, 0xcd,
    ],
    &[
        0x87, 0x08, 0x4b, 0x3b, 0xa7, 0x00, 0xba, 0xde, 0x95, 0x56, 0x10, 0x67, 0x27, 0x45, 0xb3,
        0x74, 0xe7, 0xa7, 0xb9, 0xe9, 0xec, 0x54, 0x0d, 0x5f, 0xf4, 0x3b, 0xdb, 0x12, 0x79, 0x2d,
        0x1b, 0x35,
    ],
    &[
        0xc7, 0x99, 0xb5, 0x96, 0x73, 0x8f, 0x6b, 0x01, 0x8c, 0x76, 0xc7, 0x4b, 0x17, 0x59, 0xbd,
        0x90, 0x7f, 0xec, 0x5b, 0xfd, 0x9f, 0x9b, 0x89, 0xce, 0x65, 0x48, 0x30, 0x90, 0x92, 0xd7,
        0xe9, 0x58,
    ],
    &[
        0x40, 0xf2, 0x50, 0xb2, 0x6d, 0x1f, 0x09, 0x6a, 0x4a, 0xfd, 0x4c, 0x34, 0x0a, 0x58, 0x88,
        0x15, 0x3e, 0x34, 0x13, 0x5c, 0x79, 0xdb, 0x01, 0x02, 0x00, 0x76, 0x76, 0x51, 0xcf, 0x26,
        0x30, 0x73,
    ],
    &[
        0xf6, 0x56, 0xab, 0xcc, 0xf8, 0x8d, 0xd8, 0x27, 0x02, 0x7b, 0x2c, 0xe9, 0x17, 0xd4, 0x64,
        0xec, 0x18, 0xb6, 0x25, 0x03, 0xbf, 0xbc, 0x07, 0x7f, 0xba, 0xbb, 0x98, 0xf2, 0x0d, 0x98,
        0xab, 0x34,
    ],
    &[
        0x8a, 0xed, 0x95, 0xee, 0x5b, 0x0d, 0xcb, 0xfb, 0xef, 0x4e, 0xb2, 0x1d, 0x3a, 0x3f, 0x52,
        0xf9, 0x62, 0x5a, 0x1a, 0xb0, 0x0e, 0xe3, 0x9a, 0x53, 0x27, 0x34, 0x6b, 0xdd, 0xb0, 0x1a,
        0x9c, 0x18,
    ],
    &[
        0xa1, 0x3a, 0x7c, 0x79, 0xc7, 0xe1, 0x19, 0xb5, 0xab, 0x02, 0x96, 0xab, 0x28, 0xc3, 0x00,
        0xb9, 0xf3, 0xe4, 0xc0, 0xa2, 0xe0, 0x2d, 0x1d, 0x01, 0xf7, 0xf0, 0xa7, 0x46, 0x18, 0xaf,
        0x2b, 0x48,
    ],
];
const RFC6229_8_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x80, 0xad, 0x97, 0xbd, 0xc9, 0x73, 0xdf, 0x8a, 0x2e, 0x87, 0x9e, 0x92, 0xa4, 0x97, 0xef,
        0xda, 0x20, 0xf0, 0x60, 0xc2, 0xf2, 0xe5, 0x12, 0x65, 0x01, 0xd3, 0xd4, 0xfe, 0xa1, 0x0d,
        0x5f, 0xc0,
    ],
    &[
        0xfa, 0xa1, 0x48, 0xe9, 0x90, 0x46, 0x18, 0x1f, 0xec, 0x6b, 0x20, 0x85, 0xf3, 0xb2, 0x0e,
        0xd9, 0xf0, 0xda, 0xf5, 0xba, 0xb3, 0xd5, 0x96, 0x83, 0x98, 0x57, 0x84, 0x6f, 0x73, 0xfb,
        0xfe, 0x5a,
    ],
    &[
        0x1c, 0x7e, 0x2f, 0xc4, 0x63, 0x92, 0x32, 0xfe, 0x29, 0x75, 0x84, 0xb2, 0x96, 0x99, 0x6b,
        0xc8, 0x3d, 0xb9, 0xb2, 0x49, 0x40, 0x6c, 0xc8, 0xed, 0xff, 0xac, 0x55, 0xcc, 0xd3, 0x22,
        0xba, 0x12,
    ],
    &[
        0xe4, 0xf9, 0xf7, 0xe0, 0x06, 0x61, 0x54, 0xbb, 0xd1, 0x25, 0xb7, 0x45, 0x56, 0x9b, 0xc8,
        0x97, 0x75, 0xd5, 0xef, 0x26, 0x2b, 0x44, 0xc4, 0x1a, 0x9c, 0xf6, 0x3a, 0xe1, 0x45, 0x68,
        0xe1, 0xb9,
    ],
    &[
        0x6d, 0xa4, 0x53, 0xdb, 0xf8, 0x1e, 0x82, 0x33, 0x4a, 0x3d, 0x88, 0x66, 0xcb, 0x50, 0xa1,
        0xe3, 0x78, 0x28, 0xd0, 0x74, 0x11, 0x9c, 0xab, 0x5c, 0x22, 0xb2, 0x94, 0xd7, 0xa9, 0xbf,
        0xa0, 0xbb,
    ],
    &[
        0xad, 0xb8, 0x9c, 0xea, 0x9a, 0x15, 0xfb, 0xe6, 0x17, 0x29, 0x5b, 0xd0, 0x4b, 0x8c, 0xa0,
        0x5c, 0x62, 0x51, 0xd8, 0x7f, 0xd4, 0xaa, 0xae, 0x9a, 0x7e, 0x4a, 0xd5, 0xc2, 0x17, 0xd3,
        0xf3, 0x00,
    ],
    &[
        0xe7, 0x11, 0x9b, 0xd6, 0xdd, 0x9b, 0x22, 0xaf, 0xe8, 0xf8, 0x95, 0x85, 0x43, 0x28, 0x81,
        0xe2, 0x78, 0x5b, 0x60, 0xfd, 0x7e, 0xc4, 0xe9, 0xfc, 0xb6, 0x54, 0x5f, 0x35, 0x0d, 0x66,
        0x0f, 0xab,
    ],
    &[
        0xaf, 0xec, 0xc0, 0x37, 0xfd, 0xb7, 0xb0, 0x83, 0x8e, 0xb3, 0xd7, 0x0b, 0xcd, 0x26, 0x83,
        0x82, 0xdb, 0xc1, 0xa7, 0xb4, 0x9d, 0x57, 0x35, 0x8c, 0xc9, 0xfa, 0x6d, 0x61, 0xd7, 0x3b,
        0x7c, 0xf0,
    ],
    &[
        0x63, 0x49, 0xd1, 0x26, 0xa3, 0x7a, 0xfc, 0xba, 0x89, 0x79, 0x4f, 0x98, 0x04, 0x91, 0x4f,
        0xdc, 0xbf, 0x42, 0xc3, 0x01, 0x8c, 0x2f, 0x7c, 0x66, 0xbf, 0xde, 0x52, 0x49, 0x75, 0x76,
        0x81, 0x15,
    ],
];
const RFC6229_9_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xbc, 0x92, 0x22, 0xdb, 0xd3, 0x27, 0x4d, 0x8f, 0xc6, 0x6d, 0x14, 0xcc, 0xbd, 0xa6, 0x69,
        0x0b, 0x7a, 0xe6, 0x27, 0x41, 0x0c, 0x9a, 0x2b, 0xe6, 0x93, 0xdf, 0x5b, 0xb7, 0x48, 0x5a,
        0x63, 0xe3,
    ],
    &[
        0x3f, 0x09, 0x31, 0xaa, 0x03, 0xde, 0xfb, 0x30, 0x0f, 0x06, 0x01, 0x03, 0x82, 0x6f, 0x2a,
        0x64, 0xbe, 0xaa, 0x9e, 0xc8, 0xd5, 0x9b, 0xb6, 0x81, 0x29, 0xf3, 0x02, 0x7c, 0x96, 0x36,
        0x11, 0x81,
    ],
    &[
        0x74, 0xe0, 0x4d, 0xb4, 0x6d, 0x28, 0x64, 0x8d, 0x7d, 0xee, 0x8a, 0x00, 0x64, 0xb0, 0x6c,
        0xfe, 0x9b, 0x5e, 0x81, 0xc6, 0x2f, 0xe0, 0x23, 0xc5, 0x5b, 0xe4, 0x2f, 0x87, 0xbb, 0xf9,
        0x32, 0xb8,
    ],
    &[
        0xce, 0x17, 0x8f, 0xc1, 0x82, 0x6e, 0xfe, 0xcb, 0xc1, 0x82, 0xf5, 0x79, 0x99, 0xa4, 0x61,
        0x40, 0x8b, 0xdf, 0x55, 0xcd, 0x55, 0x06, 0x1c, 0x06, 0xdb, 0xa6, 0xbe, 0x11, 0xde, 0x4a,
        0x57, 0x8a,
    ],
    &[
        0x62, 0x6f, 0x5f, 0x4d, 0xce, 0x65, 0x25, 0x01, 0xf3, 0x08, 0x7d, 0x39, 0xc9, 0x2c, 0xc3,
        0x49, 0x42, 0xda, 0xac, 0x6a, 0x8f, 0x9a, 0xb9, 0xa7, 0xfd, 0x13, 0x7c, 0x60, 0x37, 0x82,
        0x56, 0x82,
    ],
    &[
        0xcc, 0x03, 0xfd, 0xb7, 0x91, 0x92, 0xa2, 0x07, 0x31, 0x2f, 0x53, 0xf5, 0xd4, 0xdc, 0x33,
        0xd9, 0xf7, 0x0f, 0x14, 0x12, 0x2a, 0x1c, 0x98, 0xa3, 0x15, 0x5d, 0x28, 0xb8, 0xa0, 0xa8,
        0xa4, 0x1d,
    ],
    &[
        0x2a, 0x3a, 0x30, 0x7a, 0xb2, 0x70, 0x8a, 0x9c, 0x00, 0xfe, 0x0b, 0x42, 0xf9, 0xc2, 0xd6,
        0xa1, 0x86, 0x26, 0x17, 0x62, 0x7d, 0x22, 0x61, 0xea, 0xb0, 0xb1, 0x24, 0x65, 0x97, 0xca,
        0x0a, 0xe9,
    ],
    &[
        0x55, 0xf8, 0x77, 0xce, 0x4f, 0x2e, 0x1d, 0xdb, 0xbf, 0x8e, 0x13, 0xe2, 0xcd, 0xe0, 0xfd,
        0xc8, 0x1b, 0x15, 0x56, 0xcb, 0x93, 0x5f, 0x17, 0x33, 0x37, 0x70, 0x5f, 0xbb, 0x5d, 0x50,
        0x1f, 0xc1,
    ],
    &[
        0xec, 0xd0, 0xe9, 0x66, 0x02, 0xbe, 0x7f, 0x8d, 0x50, 0x92, 0x81, 0x6c, 0xcc, 0xf2, 0xc2,
        0xe9, 0x02, 0x78, 0x81, 0xfa, 0xb4, 0x99, 0x3a, 0x1c, 0x26, 0x20, 0x24, 0xa9, 0x4f, 0xff,
        0x3f, 0x61,
    ],
];
const RFC6229_10_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xbb, 0xf6, 0x09, 0xde, 0x94, 0x13, 0x17, 0x2d, 0x07, 0x66, 0x0c, 0xb6, 0x80, 0x71, 0x69,
        0x26, 0x46, 0x10, 0x1a, 0x6d, 0xab, 0x43, 0x11, 0x5d, 0x6c, 0x52, 0x2b, 0x4f, 0xe9, 0x36,
        0x04, 0xa9,
    ],
    &[
        0xcb, 0xe1, 0xff, 0xf2, 0x1c, 0x96, 0xf3, 0xee, 0xf6, 0x1e, 0x8f, 0xe0, 0x54, 0x2c, 0xbd,
        0xf0, 0x34, 0x79, 0x38, 0xbf, 0xfa, 0x40, 0x09, 0xc5, 0x12, 0xcf, 0xb4, 0x03, 0x4b, 0x0d,
        0xd1, 0xa7,
    ],
    &[
        0x78, 0x67, 0xa7, 0x86, 0xd0, 0x0a, 0x71, 0x47, 0x90, 0x4d, 0x76, 0xdd, 0xf1, 0xe5, 0x20,
        0xe3, 0x8d, 0x3e, 0x9e, 0x1c, 0xae, 0xfc, 0xcc, 0xb3, 0xfb, 0xf8, 0xd1, 0x8f, 0x64, 0x12,
        0x0b, 0x32,
    ],
    &[
        0x94, 0x23, 0x37, 0xf8, 0xfd, 0x76, 0xf0, 0xfa, 0xe8, 0xc5, 0x2d, 0x79, 0x54, 0x81, 0x06,
        0x72, 0xb8, 0x54, 0x8c, 0x10, 0xf5, 0x16, 0x67, 0xf6, 0xe6, 0x0e, 0x18, 0x2f, 0xa1, 0x9b,
        0x30, 0xf7,
    ],
    &[
        0x02, 0x11, 0xc7, 0xc6, 0x19, 0x0c, 0x9e, 0xfd, 0x12, 0x37, 0xc3, 0x4c, 0x8f, 0x2e, 0x06,
        0xc4, 0xbd, 0xa6, 0x4f, 0x65, 0x27, 0x6d, 0x2a, 0xac, 0xb8, 0xf9, 0x02, 0x12, 0x20, 0x3a,
        0x80, 0x8e,
    ],
    &[
        0xbd, 0x38, 0x20, 0xf7, 0x32, 0xff, 0xb5, 0x3e, 0xc1, 0x93, 0xe7, 0x9d, 0x33, 0xe2, 0x7c,
        0x73, 0xd0, 0x16, 0x86, 0x16, 0x86, 0x19, 0x07, 0xd4, 0x82, 0xe3, 0x6c, 0xda, 0xc8, 0xcf,
        0x57, 0x49,
    ],
    &[
        0x97, 0xb0, 0xf0, 0xf2, 0x24, 0xb2, 0xd2, 0x31, 0x71, 0x14, 0x80, 0x8f, 0xb0, 0x3a, 0xf7,
        0xa0, 0xe5, 0x96, 0x16, 0xe4, 0x69, 0x78, 0x79, 0x39, 0xa0, 0x63, 0xce, 0xea, 0x9a, 0xf9,
        0x56, 0xd1,
    ],
    &[
        0xc4, 0x7e, 0x0d, 0xc1, 0x66, 0x09, 0x19, 0xc1, 0x11, 0x01, 0x20, 0x8f, 0x9e, 0x69, 0xaa,
        0x1f, 0x5a, 0xe4, 0xf1, 0x28, 0x96, 0xb8, 0x37, 0x9a, 0x2a, 0xad, 0x89, 0xb5, 0xb5, 0x53,
        0xd6, 0xb0,
    ],
    &[
        0x6b, 0x6b, 0x09, 0x8d, 0x0c, 0x29, 0x3b, 0xc2, 0x99, 0x3d, 0x80, 0xbf, 0x05, 0x18, 0xb6,
        0xd9, 0x81, 0x70, 0xcc, 0x3c, 0xcd, 0x92, 0xa6, 0x98, 0x62, 0x1b, 0x93, 0x9d, 0xd3, 0x8f,
        0xe7, 0xb9,
    ],
];
const RFC6229_11_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xab, 0x65, 0xc2, 0x6e, 0xdd, 0xb2, 0x87, 0x60, 0x0d, 0xb2, 0xfd, 0xa1, 0x0d, 0x1e, 0x60,
        0x5c, 0xbb, 0x75, 0x90, 0x10, 0xc2, 0x96, 0x58, 0xf2, 0xc7, 0x2d, 0x93, 0xa2, 0xd1, 0x6d,
        0x29, 0x30,
    ],
    &[
        0xb9, 0x01, 0xe8, 0x03, 0x6e, 0xd1, 0xc3, 0x83, 0xcd, 0x3c, 0x4c, 0x4d, 0xd0, 0xa6, 0xab,
        0x05, 0x3d, 0x25, 0xce, 0x49, 0x22, 0x92, 0x4c, 0x55, 0xf0, 0x64, 0x94, 0x33, 0x53, 0xd7,
        0x8a, 0x6c,
    ],
    &[
        0x12, 0xc1, 0xaa, 0x44, 0xbb, 0xf8, 0x7e, 0x75, 0xe6, 0x11, 0xf6, 0x9b, 0x2c, 0x38, 0xf4,
        0x9b, 0x28, 0xf2, 0xb3, 0x43, 0x4b, 0x65, 0xc0, 0x98, 0x77, 0x47, 0x00, 0x44, 0xc6, 0xea,
        0x17, 0x0d,
    ],
    &[
        0xbd, 0x9e, 0xf8, 0x22, 0xde, 0x52, 0x88, 0x19, 0x61, 0x34, 0xcf, 0x8a, 0xf7, 0x83, 0x93,
        0x04, 0x67, 0x55, 0x9c, 0x23, 0xf0, 0x52, 0x15, 0x84, 0x70, 0xa2, 0x96, 0xf7, 0x25, 0x73,
        0x5a, 0x32,
    ],
    &[
        0x8b, 0xab, 0x26, 0xfb, 0xc2, 0xc1, 0x2b, 0x0f, 0x13, 0xe2, 0xab, 0x18, 0x5e, 0xab, 0xf2,
        0x41, 0x31, 0x18, 0x5a, 0x6d, 0x69, 0x6f, 0x0c, 0xfa, 0x9b, 0x42, 0x80, 0x8b, 0x38, 0xe1,
        0x32, 0xa2,
    ],
    &[
        0x56, 0x4d, 0x3d, 0xae, 0x18, 0x3c, 0x52, 0x34, 0xc8, 0xaf, 0x1e, 0x51, 0x06, 0x1c, 0x44,
        0xb5, 0x3c, 0x07, 0x78, 0xa7, 0xb5, 0xf7, 0x2d, 0x3c, 0x23, 0xa3, 0x13, 0x5c, 0x7d, 0x67,
        0xb9, 0xf4,
    ],
    &[
        0xf3, 0x43, 0x69, 0x89, 0x0f, 0xcf, 0x16, 0xfb, 0x51, 0x7d, 0xca, 0xae, 0x44, 0x63, 0xb2,
        0xdd, 0x02, 0xf3, 0x1c, 0x81, 0xe8, 0x20, 0x07, 0x31, 0xb8, 0x99, 0xb0, 0x28, 0xe7, 0x91,
        0xbf, 0xa7,
    ],
    &[
        0x72, 0xda, 0x64, 0x62, 0x83, 0x22, 0x8c, 0x14, 0x30, 0x08, 0x53, 0x70, 0x17, 0x95, 0x61,
        0x6f, 0x4e, 0x0a, 0x8c, 0x6f, 0x79, 0x34, 0xa7, 0x88, 0xe2, 0x26, 0x5e, 0x81, 0xd6, 0xd0,
        0xc8, 0xf4,
    ],
    &[
        0x43, 0x8d, 0xd5, 0xea, 0xfe, 0xa0, 0x11, 0x1b, 0x6f, 0x36, 0xb4, 0xb9, 0x38, 0xda, 0x2a,
        0x68, 0x5f, 0x6b, 0xfc, 0x73, 0x81, 0x58, 0x74, 0xd9, 0x71, 0x00, 0xf0, 0x86, 0x97, 0x93,
        0x57, 0xd8,
    ],
];
const RFC6229_12_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x72, 0x0c, 0x94, 0xb6, 0x3e, 0xdf, 0x44, 0xe1, 0x31, 0xd9, 0x50, 0xca, 0x21, 0x1a, 0x5a,
        0x30, 0xc3, 0x66, 0xfd, 0xea, 0xcf, 0x9c, 0xa8, 0x04, 0x36, 0xbe, 0x7c, 0x35, 0x84, 0x24,
        0xd2, 0x0b,
    ],
    &[
        0xb3, 0x39, 0x4a, 0x40, 0xaa, 0xbf, 0x75, 0xcb, 0xa4, 0x22, 0x82, 0xef, 0x25, 0xa0, 0x05,
        0x9f, 0x48, 0x47, 0xd8, 0x1d, 0xa4, 0x94, 0x2d, 0xbc, 0x24, 0x9d, 0xef, 0xc4, 0x8c, 0x92,
        0x2b, 0x9f,
    ],
    &[
        0x08, 0x12, 0x8c, 0x46, 0x9f, 0x27, 0x53, 0x42, 0xad, 0xda, 0x20, 0x2b, 0x2b, 0x58, 0xda,
        0x95, 0x97, 0x0d, 0xac, 0xef, 0x40, 0xad, 0x98, 0x72, 0x3b, 0xac, 0x5d, 0x69, 0x55, 0xb8,
        0x17, 0x61,
    ],
    &[
        0x3c, 0xb8, 0x99, 0x93, 0xb0, 0x7b, 0x0c, 0xed, 0x93, 0xde, 0x13, 0xd2, 0xa1, 0x10, 0x13,
        0xac, 0xef, 0x2d, 0x67, 0x6f, 0x15, 0x45, 0xc2, 0xc1, 0x3d, 0xc6, 0x80, 0xa0, 0x2f, 0x4a,
        0xdb, 0xfe,
    ],
    &[
        0xb6, 0x05, 0x95, 0x51, 0x4f, 0x24, 0xbc, 0x9f, 0xe5, 0x22, 0xa6, 0xca, 0xd7, 0x39, 0x36,
        0x44, 0xb5, 0x15, 0xa8, 0xc5, 0x01, 0x17, 0x54, 0xf5, 0x90, 0x03, 0x05, 0x8b, 0xdb, 0x81,
        0x51, 0x4e,
    ],
    &[
        0x3c, 0x70, 0x04, 0x7e, 0x8c, 0xbc, 0x03, 0x8e, 0x3b, 0x98, 0x20, 0xdb, 0x60, 0x1d, 0xa4,
        0x95, 0x11, 0x75, 0xda, 0x6e, 0xe7, 0x56, 0xde, 0x46, 0xa5, 0x3e, 0x2b, 0x07, 0x56, 0x60,
        0xb7, 0x70,
    ],
    &[
        0x00, 0xa5, 0x42, 0xbb, 0xa0, 0x21, 0x11, 0xcc, 0x2c, 0x65, 0xb3, 0x8e, 0xbd, 0xba, 0x58,
        0x7e, 0x58, 0x65, 0xfd, 0xbb, 0x5b, 0x48, 0x06, 0x41, 0x04, 0xe8, 0x30, 0xb3, 0x80, 0xf2,
        0xae, 0xde,
    ],
    &[
        0x34, 0xb2, 0x1a, 0xd2, 0xad, 0x44, 0xe9, 0x99, 0xdb, 0x2d, 0x7f, 0x08, 0x63, 0xf0, 0xd9,
        0xb6, 0x84, 0xa9, 0x21, 0x8f, 0xc3, 0x6e, 0x8a, 0x5f, 0x2c, 0xcf, 0xbe, 0xae, 0x53, 0xa2,
        0x7d, 0x25,
    ],
    &[
        0xa2, 0x22, 0x1a, 0x11, 0xb8, 0x33, 0xcc, 0xb4, 0x98, 0xa5, 0x95, 0x40, 0xf0, 0x54, 0x5f,
        0x4a, 0x5b, 0xbe, 0xb4, 0x78, 0x7d, 0x59, 0xe5, 0x37, 0x3f, 0xdb, 0xea, 0x6c, 0x6f, 0x75,
        0xc2, 0x9b,
    ],
];
const RFC6229_13_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0x54, 0xb6, 0x4e, 0x6b, 0x5a, 0x20, 0xb5, 0xe2, 0xec, 0x84, 0x59, 0x3d, 0xc7, 0x98, 0x9d,
        0xa7, 0xc1, 0x35, 0xee, 0xe2, 0x37, 0xa8, 0x54, 0x65, 0xff, 0x97, 0xdc, 0x03, 0x92, 0x4f,
        0x45, 0xce,
    ],
    &[
        0xcf, 0xcc, 0x92, 0x2f, 0xb4, 0xa1, 0x4a, 0xb4, 0x5d, 0x61, 0x75, 0xaa, 0xbb, 0xf2, 0xd2,
        0x01, 0x83, 0x7b, 0x87, 0xe2, 0xa4, 0x46, 0xad, 0x0e, 0xf7, 0x98, 0xac, 0xd0, 0x2b, 0x94,
        0x12, 0x4f,
    ],
    &[
        0x17, 0xa6, 0xdb, 0xd6, 0x64, 0x92, 0x6a, 0x06, 0x36, 0xb3, 0xf4, 0xc3, 0x7a, 0x4f, 0x46,
        0x94, 0x4a, 0x5f, 0x9f, 0x26, 0xae, 0xee, 0xd4, 0xd4, 0xa2, 0x5f, 0x63, 0x2d, 0x30, 0x52,
        0x33, 0xd9,
    ],
    &[
        0x80, 0xa3, 0xd0, 0x1e, 0xf0, 0x0c, 0x8e, 0x9a, 0x42, 0x09, 0xc1, 0x7f, 0x4e, 0xeb, 0x35,
        0x8c, 0xd1, 0x5e, 0x7d, 0x5f, 0xfa, 0xaa, 0xbc, 0x02, 0x07, 0xbf, 0x20, 0x0a, 0x11, 0x77,
        0x93, 0xa2,
    ],
    &[
        0x34, 0x96, 0x82, 0xbf, 0x58, 0x8e, 0xaa, 0x52, 0xd0, 0xaa, 0x15, 0x60, 0x34, 0x6a, 0xea,
        0xfa, 0xf5, 0x85, 0x4c, 0xdb, 0x76, 0xc8, 0x89, 0xe3, 0xad, 0x63, 0x35, 0x4e, 0x5f, 0x72,
        0x75, 0xe3,
    ],
    &[
        0x53, 0x2c, 0x7c, 0xec, 0xcb, 0x39, 0xdf, 0x32, 0x36, 0x31, 0x84, 0x05, 0xa4, 0xb1, 0x27,
        0x9c, 0xba, 0xef, 0xe6, 0xd9, 0xce, 0xb6, 0x51, 0x84, 0x22, 0x60, 0xe0, 0xd1, 0xe0, 0x5e,
        0x3b, 0x90,
    ],
    &[
        0xe8, 0x2d, 0x8c, 0x6d, 0xb5, 0x4e, 0x3c, 0x63, 0x3f, 0x58, 0x1c, 0x95, 0x2b, 0xa0, 0x42,
        0x07, 0x4b, 0x16, 0xe5, 0x0a, 0xbd, 0x38, 0x1b, 0xd7, 0x09, 0x00, 0xa9, 0xcd, 0x9a, 0x62,
        0xcb, 0x23,
    ],
    &[
        0x36, 0x82, 0xee, 0x33, 0xbd, 0x14, 0x8b, 0xd9, 0xf5, 0x86, 0x56, 0xcd, 0x8f, 0x30, 0xd9,
        0xfb, 0x1e, 0x5a, 0x0b, 0x84, 0x75, 0x04, 0x5d, 0x9b, 0x20, 0xb2, 0x62, 0x86, 0x24, 0xed,
        0xfd, 0x9e,
    ],
    &[
        0x63, 0xed, 0xd6, 0x84, 0xfb, 0x82, 0x62, 0x82, 0xfe, 0x52, 0x8f, 0x9c, 0x0e, 0x92, 0x37,
        0xbc, 0xe4, 0xdd, 0x2e, 0x98, 0xd6, 0x96, 0x0f, 0xae, 0x0b, 0x43, 0x54, 0x54, 0x56, 0x74,
        0x33, 0x91,
    ],
];
const RFC6229_14_EXPECT: &[&[u8; 32]; 9] = &[
    &[
        0xdd, 0x5b, 0xcb, 0x00, 0x18, 0xe9, 0x22, 0xd4, 0x94, 0x75, 0x9d, 0x7c, 0x39, 0x5d, 0x02,
        0xd3, 0xc8, 0x44, 0x6f, 0x8f, 0x77, 0xab, 0xf7, 0x37, 0x68, 0x53, 0x53, 0xeb, 0x89, 0xa1,
        0xc9, 0xeb,
    ],
    &[
        0xaf, 0x3e, 0x30, 0xf9, 0xc0, 0x95, 0x04, 0x59, 0x38, 0x15, 0x15, 0x75, 0xc3, 0xfb, 0x90,
        0x98, 0xf8, 0xcb, 0x62, 0x74, 0xdb, 0x99, 0xb8, 0x0b, 0x1d, 0x20, 0x12, 0xa9, 0x8e, 0xd4,
        0x8f, 0x0e,
    ],
    &[
        0x25, 0xc3, 0x00, 0x5a, 0x1c, 0xb8, 0x5d, 0xe0, 0x76, 0x25, 0x98, 0x39, 0xab, 0x71, 0x98,
        0xab, 0x9d, 0xcb, 0xc1, 0x83, 0xe8, 0xcb, 0x99, 0x4b, 0x72, 0x7b, 0x75, 0xbe, 0x31, 0x80,
        0x76, 0x9c,
    ],
    &[
        0xa1, 0xd3, 0x07, 0x8d, 0xfa, 0x91, 0x69, 0x50, 0x3e, 0xd9, 0xd4, 0x49, 0x1d, 0xee, 0x4e,
        0xb2, 0x85, 0x14, 0xa5, 0x49, 0x58, 0x58, 0x09, 0x6f, 0x59, 0x6e, 0x4b, 0xcd, 0x66, 0xb1,
        0x06, 0x65,
    ],
    &[
        0x5f, 0x40, 0xd5, 0x9e, 0xc1, 0xb0, 0x3b, 0x33, 0x73, 0x8e, 0xfa, 0x60, 0xb2, 0x25, 0x5d,
        0x31, 0x34, 0x77, 0xc7, 0xf7, 0x64, 0xa4, 0x1b, 0xac, 0xef, 0xf9, 0x0b, 0xf1, 0x4f, 0x92,
        0xb7, 0xcc,
    ],
    &[
        0xac, 0x4e, 0x95, 0x36, 0x8d, 0x99, 0xb9, 0xeb, 0x78, 0xb8, 0xda, 0x8f, 0x81, 0xff, 0xa7,
        0x95, 0x8c, 0x3c, 0x13, 0xf8, 0xc2, 0x38, 0x8b, 0xb7, 0x3f, 0x38, 0x57, 0x6e, 0x65, 0xb7,
        0xc4, 0x46,
    ],
    &[
        0x13, 0xc4, 0xb9, 0xc1, 0xdf, 0xb6, 0x65, 0x79, 0xed, 0xdd, 0x8a, 0x28, 0x0b, 0x9f, 0x73,
        0x16, 0xdd, 0xd2, 0x78, 0x20, 0x55, 0x01, 0x26, 0x69, 0x8e, 0xfa, 0xad, 0xc6, 0x4b, 0x64,
        0xf6, 0x6e,
    ],
    &[
        0xf0, 0x8f, 0x2e, 0x66, 0xd2, 0x8e, 0xd1, 0x43, 0xf3, 0xa2, 0x37, 0xcf, 0x9d, 0xe7, 0x35,
        0x59, 0x9e, 0xa3, 0x6c, 0x52, 0x55, 0x31, 0xb8, 0x80, 0xba, 0x12, 0x43, 0x34, 0xf5, 0x7b,
        0x0b, 0x70,
    ],
    &[
        0xd5, 0xa3, 0x9e, 0x3d, 0xfc, 0xc5, 0x02, 0x80, 0xba, 0xc4, 0xa6, 0xb5, 0xaa, 0x0d, 0xca,
        0x7d, 0x37, 0x0b, 0x1c, 0x1f, 0xe6, 0x55, 0x91, 0x6d, 0x97, 0xfd, 0x0d, 0x47, 0xca, 0x1d,
        0x72, 0xb8,
    ],
];
