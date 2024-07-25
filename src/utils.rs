use std::{char, default, vec::Splice};

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok((r, g, b))
}

pub fn pos_to_voyager(x: u16, y: u16) -> usize {
    // 0,  5   is left 1st row
    // 6,  11  is left 2st row
    // 12, 17  is left 3st row
    // 18, 23  is left 4st row

    // 24, 25 is left thumb keys
    // 50, 51 is right thumb keys

    // 26, 31  is right 1st row
    // 32, 37  is right 2st row
    // 38, 43  is right 3st row
    // 44, 49  is right 4st row

    #[rustfmt::skip]
    let voyager_layout: [[usize; 12]; 5] = [
        [00, 01, 02, 03, 04, 05,        26, 27, 28, 29, 30, 31],
        [06, 07, 08, 09, 10, 11,        32, 33, 34, 35, 36, 37],
        [12, 13, 14, 15, 16, 17,        38, 39, 40, 41, 42, 43],
        [18, 19, 20, 21, 22, 23,        44, 45, 46, 47, 48, 49],

        [60, 60, 60, 60, 24, 26,        50, 51, 60, 60, 60, 60]
    ];
    voyager_layout[y as usize][x as usize]
}

pub struct PixelBuf<T, const X: usize, const Y: usize, const X_Limit: usize> {
    data: Vec<[[T; X]; Y]>,
    prev: [[[T; X]; Y]; X_Limit]
}

#[rustfmt::skip]
const SPACE_CHAR : [[u16; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],  
    [0, 0, 0, 0], 
    [0, 0, 0, 0]
];

#[rustfmt::skip]
const UNKNOWN_CHAR : [[u16; 4]; 4] = [
    [1, 1, 1, 0], 
    [1, 1, 1, 0], 
    [1, 1, 1, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const UNDERSCORE : [[u16; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],  
    [0, 0, 0, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_0 : [[u16; 4]; 4] = [
    [1, 1, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_1 : [[u16; 4]; 4] = [
    [0, 1, 0, 0], 
    [1, 1, 0, 0], 
    [0, 1, 0, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_2 : [[u16; 4]; 4] = [
    [1, 1, 0, 0], 
    [0, 0, 1, 0], 
    [0, 1, 0, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_3 : [[u16; 4]; 4] = [
    [1, 1, 1, 0], 
    [0, 0, 1, 0], 
    [0, 1, 1, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_4 : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [1, 1, 1, 0], 
    [0, 0, 1, 0]
];

#[rustfmt::skip]
const L_A : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_B : [[u16; 4]; 4] = [
    [1, 0, 0, 0], 
    [1, 1, 0, 0], 
    [1, 0, 1, 0], 
    [1, 1, 1, 0]
];

#[rustfmt::skip]
const L_C : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 0, 0, 0], 
    [1, 0, 0, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_D : [[u16; 4]; 4] = [
    [0, 0, 1, 0], 
    [0, 1, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_E : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 0, 1, 0], 
    [1, 1, 0, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_F : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [0, 1, 0, 0], 
    [1, 1, 1, 0], 
    [0, 1, 0, 0]
];

#[rustfmt::skip]
const L_G : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 1, 0], 
    [1, 1, 0, 0]
];

#[rustfmt::skip]
const L_H : [[u16; 4]; 4] = [
    [1, 0, 0, 0], 
    [1, 1, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_I : [[u16; 4]; 4] = [
    [1, 0, 0, 0], 
    [0, 0, 0, 0], 
    [1, 0, 0, 0], 
    [1, 0, 0, 0]
];

#[rustfmt::skip]
const L_J : [[u16; 4]; 4] = [
    [0, 1, 0, 0], 
    [0, 0, 0, 0], 
    [0, 1, 0, 0], 
    [1, 0, 0, 0]
];

#[rustfmt::skip]
const L_K : [[u16; 4]; 4] = [
    [1, 0, 0, 0], 
    [1, 0, 1, 0], 
    [1, 1, 0, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_L : [[u16; 4]; 4] = [
    [1, 0, 0, 0], 
    [1, 0, 0, 0], 
    [1, 0, 0, 0], 
    [1, 1, 0, 0]
];

#[rustfmt::skip]
const L_M : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 1, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_N : [[u16; 4]; 4] = [
    [0, 0, 0, 0], 
    [1, 1, 0, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_O : [[u16; 4]; 4] = [
    [0, 1, 0, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 0, 0]
];

#[rustfmt::skip]
const L_P : [[u16; 4]; 4] = [
    [1, 1, 0, 0], 
    [1, 0, 1, 0], 
    [1, 1, 0, 0], 
    [1, 0, 0, 0]
];

#[rustfmt::skip]
const L_Q : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 1, 0], 
    [0, 0, 1, 0]
];

#[rustfmt::skip]
const L_R : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 1, 0, 0], 
    [1, 0, 0, 0], 
    [1, 0, 0, 0]
];

#[rustfmt::skip]
const L_S : [[u16; 4]; 4] = [
    [0, 1, 1, 0], 
    [1, 1, 0, 0], 
    [0, 0, 1, 0], 
    [1, 1, 0, 0]
];

#[rustfmt::skip]
const L_T : [[u16; 4]; 4] = [
    [0, 1, 0, 0], 
    [1, 1, 1, 0], 
    [0, 1, 0, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_U : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 1, 0]
];

#[rustfmt::skip]
const L_V : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [0, 1, 0, 0]
];

#[rustfmt::skip]
const L_W : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [1, 0, 1, 0], 
    [1, 1, 1, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_X : [[u16; 4]; 4] = [
    [1, 0, 1, 0], 
    [0, 1, 0, 0], 
    [1, 0, 1, 0], 
    [1, 0, 1, 0]
];

#[rustfmt::skip]
const L_Z : [[u16; 4]; 4] = [
    [1, 1, 1, 0], 
    [0, 0, 1, 0], 
    [0, 1, 0, 0], 
    [1, 1, 1, 0]
];

pub fn text_to_px(a: &str) -> PixelBuf<u16, 4, 4, 3> {
    let mut v = vec![];
    for s in a.chars() {
        match s {
            ' ' => v.push(SPACE_CHAR),
            '0' => v.push(L_0),
            '1' => v.push(L_1),
            '2' => v.push(L_2),
            '3' => v.push(L_3),
            '4' => v.push(L_4),
            'a' => v.push(L_A),
            'b' => v.push(L_B),
            'c' => v.push(L_C),
            'd' => v.push(L_D),
            'e' => v.push(L_E),
            'f' => v.push(L_F),
            'g' => v.push(L_G),
            'h' => v.push(L_H),
            'i' => v.push(L_I),
            'j' => v.push(L_J),
            'k' => v.push(L_K),
            'l' => v.push(L_L),
            'm' => v.push(L_M),
            'n' => v.push(L_N),
            'o' => v.push(L_O),
            'p' => v.push(L_P),
            'q' => v.push(L_Q),
            's' => v.push(L_R),
            's' => v.push(L_S),
            't' => v.push(L_T),
            'u' => v.push(L_U),
            'v' => v.push(L_V),
            'w' => v.push(L_W),
            'x' => v.push(L_X),
            'z' => v.push(L_Z),
            '_' => v.push(UNDERSCORE),
            _ => v.push(UNKNOWN_CHAR),
        }
    }
    PixelBuf { data: v, prev: [SPACE_CHAR, SPACE_CHAR, SPACE_CHAR] }
}

impl<T, const X: usize, const Y: usize, const X_Limit: usize> PixelBuf<T, X, Y, X_Limit>
where
    T: Copy + PartialEq
{
    pub fn new() -> PixelBuf<T, {X}, {Y}, {X_Limit}>{
        PixelBuf { data: vec![], prev: [[[T; X]; Y]; X_Limit] }
    }

    pub fn foreach_px(&mut self, f: impl Fn(usize, usize, T)) {
        for (i, px_char) in self.data.iter().enumerate() {
            for y in 0..Y {
                for x in 0..X {
                    if i <= X_Limit {
                        let v = px_char[y][x];
                        if self.prev[i][y][x].eq(&v){
                            continue;
                        } else {
                            f(x + X * i, y, v);
                            self.prev[i][y][x] = v;
                        }
                    } else{
                        return;
                    }
                }
            }
        }
    }
}
