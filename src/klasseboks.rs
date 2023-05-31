use image::{Rgb, RgbImage};

pub struct KlasseBoks {
    navn: String,
    funksjoner: Vec<String>,
}
impl KlasseBoks {
    pub fn new(navn: String, funksjoner: Vec<String>) -> Self {
        KlasseBoks { navn, funksjoner }
    }
    pub fn hoyde(&self) -> u32 {
        return 20 + 15*self.funksjoner.len() as u32;
    }
    pub fn draw_box(&self, bilde: &mut RgbImage, posisjon: [u32;2]) {
        let storrelse: [u32; 2] = [400, 600];
        for [x, y] in Self::outline(posisjon, storrelse) {
            let pixel = bilde.get_pixel_mut(x, y);
            *pixel = Rgb([255, 255, 255])
        }
    }
    fn outline(posisjon: [u32;2], storrelse: [u32;2]) -> Vec<[u32;2]> {
        let linje_bredde = 2;
        let mut out: Vec<[u32; 2]> = Vec::new();

        for x in posisjon[0]..=(posisjon[0] + storrelse[0]) {
            for y in (posisjon[1]..=(posisjon[1]+ linje_bredde))
                .chain((posisjon[1]+storrelse[1]-linje_bredde)..=(posisjon[1]+storrelse[1])) {
                    out.push([x, y])
                }
        }
        for x in (posisjon[0]..=(posisjon[0] + linje_bredde))
            .chain((posisjon[0]+storrelse[0]-linje_bredde)..=(posisjon[0]+storrelse[0])) {
                for y in posisjon[1]..=(posisjon[1] + storrelse[1]) {
                    out.push([x, y]);
                }
        }

        return out;
    }
}
