#[derive(Debug)]
pub struct Board {
    width: i32,
    height: i32,
    board: Vec<u8>,
}

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width,
            height,
            board: vec![0; (width*height*3) as usize]
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get(&mut self, x: i32, y: i32) -> Option<(u8, u8, u8)> {
        if x < 0 || y < 0 {
            return None;
        }

        if x >= self.width || y >= self.height {
            return None;
        }

        let offset = ((y * self.width + x) * 3) as usize;

        Some((
            self.board[offset + 0],
            self.board[offset + 1],
            self.board[offset + 2]
        ))
    }

    pub fn put(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        if x < 0 || y < 0 {
            return;
        }

        if x >= self.width || y >= self.height {
            return;
        }

        let offset = ((y * self.width + x) * 3) as usize;

        self.board[offset + 0] = color.0;
        self.board[offset + 1] = color.1;
        self.board[offset + 2] = color.2;
    }

    pub fn clear(&mut self, x: i32, y: i32, w: i32, h: i32) {
        assert!(w >= 0);
        assert!(h >= 0);

        for cy in y..(y+h) {
            for cx in x..(x+w) {
                self.put(cx, cy, (0, 0, 0));
            }
        }
    }

    pub fn scroll_down(&mut self, lines: i32) {
        assert!(lines > 0);

        let mut row = vec![0; (self.width * 3) as usize];

        for y in 0..self.height {
            if y < lines {
                continue;
            }

            if y + lines >= self.height {
                for x in 0..self.width {
                    self.put(x, y, (0, 0, 0));
                }
            } else {
                let offset_s = ((y * self.width) * 3) as usize;
                let offset_d = (((y - lines) * self.width) * 3) as usize;
                let len      = (self.width * 3) as usize;

                row.copy_from_slice(&self.board[offset_s..(offset_s+len)]);
                self.board[offset_d..(offset_d+len)].copy_from_slice(&row);
            }
        }
    }
}
