use std::io::Cursor;

use image::ImageDecoder;
use image::ColorType;
use image::bmp::BMPDecoder;

use crate::board::Board;

static FONT: &[u8] = include_bytes!("../res/font.bmp");

static GLYPH_W: i32 = 5;
static GLYPH_H: i32 = 7;

static FONT_W: i32 = 26;
static FONT_H: i32 = 3;

pub struct Font {
    font_buf: Vec<u8>,
}

impl Font {
    pub fn new() -> Font {
        // We don't catch errors here because this is a static resource that shouldn't fail to read
        // in production
        let mut font_cursor = Cursor::new(FONT);

        let decoder = BMPDecoder::new(&mut font_cursor).unwrap();

        assert_eq!(decoder.dimensions(), ((GLYPH_W*FONT_W) as u64, (GLYPH_H*FONT_H) as u64));
        assert_eq!(decoder.colortype(), ColorType::RGB(8));

        let font_buf = decoder.read_image().unwrap();

        Font { font_buf }
    }

    pub fn glyph_width(&self, glyph: char) -> i32 {
        match glyph {
            'i' => 1,
            't' => 3,
            'j' => 3,
            'l' => 2,
            _ => GLYPH_W
        }
    }

    pub fn render(&self,
                  target: &mut Board,
                  x: i32,
                  y: i32,
                  color: (u8, u8, u8),
                  glyph: char) {

        let (selx, sely) =
            match glyph {
                'A'...'Z' => {
                    let i = (glyph as u32 - 'A' as u32) as i32;
                    (i, 0)
                },
                'a'...'z' => {
                    let i = (glyph as u32 - 'a' as u32) as i32;
                    (i, 1)
                },
                '0'...'9' => {
                    let i = (glyph as u32 - '0' as u32) as i32;
                    (i, 2)
                },
                ':' => (10, 2),
                '@' => (11, 2),
                '=' => (12, 2),
                _ => {
                    // Unsupported.
                    target.clear(x, y, GLYPH_W, GLYPH_H);
                    return;
                }
            };

        let ox = selx * GLYPH_W;
        let oy = sely * GLYPH_H;
        let ow = FONT_W * GLYPH_W;

        let max_w = self.glyph_width(glyph);

        for cy in 0..GLYPH_H {
            for cx in 0..max_w {
                let byte = self.font_buf[(((oy+cy)*ow + (ox+cx))*3) as usize];

                if byte > 0 {
                    target.put(x + cx, y + cy, color);
                } else {
                    target.put(x + cx, y + cy, (0,0,0));
                }
            }
        }
    }

    pub fn render_str(&self,
                      target: &mut Board,
                      mut x: i32,
                      y: i32,
                      color: (u8, u8, u8),
                      string: &str) {
        for glyph in string.chars() {
            self.render(target, x, y, color, glyph);

            x += self.glyph_width(glyph) + 1;

            if x > target.width() {
                return;
            }
        }
    }
}
