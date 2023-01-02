pub struct RenderManager {
    pub width: usize,
    pub height: usize,
    pub framebuffer: Vec<char>,
}
impl RenderManager {
    pub fn new(width: usize, height: usize) -> Self {
        let framebuffer = (0..(width*height)).map(|_| ' ' as char).collect();
        Self {
            width,
            height,
            framebuffer,
        }
    }
    pub fn get_point(&self, x: usize, y: usize) -> Result<char, &'static str> {
        if x >= self.width || y >= self.height {
            return Err("x and y must be smaller than framebuffer width and height");
        }
        let index = y * self.width + x;
        Ok(self.framebuffer[index])
    }
    pub fn set_point(&mut self, x: usize, y: usize, value: char) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("x and y must be smaller than framebuffer width and height");
        }
        let index = y * self.width + x;
        self.framebuffer[index] = value;
        Ok(())
    }
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, value: char) {
        for x_ in x..(x+w) {
            for y_ in y..(y+h) {
                let _ = self.set_point(x_, y_, value);
            }
        }
    }
    pub fn fill_background(&mut self, value: char) {
        self.fill_rect(0, 0, self.width, self.height, value);
    }
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, value: char) {
        const DETAIL: usize = 30;
        for progress in 0..DETAIL {
            let percent = (progress as f64) / (DETAIL as f64);
            let offx = ((x2 as isize - x1 as isize) as f64 * percent) as isize;
            let offy = ((y2 as isize - y1 as isize) as f64 * percent) as isize;
            let _ = self.set_point((x1 as isize + offx) as usize,
                                   (y1 as isize + offy) as usize, value);
        }
    }
    pub fn present(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = self.get_point(x, y).expect("If this fails, something real bad
                    has happened here");
                // we print every point 2 chars wide so the screen size isn't as wacky
                print!("{}{}", point, point);
            }
            // newline
            println!();
        }
    }
    pub fn draw_tri(&mut self, x1: usize, y1: usize, x2: usize, y2: usize,
                    x3: usize, y3: usize, value: char) {
        self.draw_line(x1, y1, x2, y2, value);
        self.draw_line(x2, y2, x3, y3, value);
        self.draw_line(x3, y3, x1, y1, value);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn render_test() {
        let mut rm = RenderManager::new(5, 4);
        rm.fill_background('3');
        let _ = rm.set_point(0, 1, '5');
        rm.present();
    }
}
