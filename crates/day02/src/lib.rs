use shared::combine::Parser;

pub struct Prism {
    pub l: u32,
    pub w: u32,
    pub h: u32,
}

impl Prism {
    pub fn surface_area(&self) -> u64 {
        self.side_areas().into_iter().map(|side| 2 * side).sum()
    }

    pub fn smallest_side_surface_area(&self) -> u64 {
        self.side_areas().into_iter().min().unwrap()
    }

    pub fn volume(&self) -> u64 {
        self.l as u64 * self.w as u64 * self.h as u64
    }

    pub fn smallest_perimiter(&self) -> u64 {
        [
            2 * self.l as u64 + 2 * self.w as u64,
            2 * self.w as u64 + 2 * self.h as u64,
            2 * self.h as u64 + 2 * self.l as u64,
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    fn side_areas(&self) -> [u64; 3] {
        [
            self.l as u64 * self.w as u64,
            self.w as u64 * self.h as u64,
            self.h as u64 * self.l as u64,
        ]
    }
}

pub fn parser<'a>() -> impl Parser<&'a str, Output = Prism> {
    use shared::combine::parser::char::char;
    use shared::combine::parser::sequence::skip;
    (
        skip(shared::parse::u32(), char('x')),
        skip(shared::parse::u32(), char('x')),
        shared::parse::u32(),
    )
        .map(|(l, w, h)| Prism { l, w, h })
}
