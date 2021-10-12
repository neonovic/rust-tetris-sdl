fn main() {
    let u = Universe {
        width: 3,
        height: 3,
        area: [0, 0, 0, 1, 1, 0, 0, 1, 0].into(),
    };
    dbg!(4 % 3);
    //dbg!(u.live_neighbor_count(1, 1));
}

#[derive(Debug)]
struct Universe {
    width: u32,
    height: u32,
    area: Vec<u8>,
}

impl Universe {
    fn new(width: u32, height: u32) -> Self {
        let area = (0..width * height)
            .map(|i| if i % 2 == 0 || i % 7 == 0 { 1 } else { 0 })
            .collect();
        Self {
            width,
            height,
            area,
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.area[idx];
            }
        }
        count
    }
}
