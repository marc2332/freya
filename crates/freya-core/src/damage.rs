use smallvec::SmallVec;
use torin::prelude::Area;

const MAX_DAMAGE_RECTS: usize = 8;

/// Screen rects that must repaint in the next frame, merged into growing
/// invalid areas that collapse into a single bounding rect at the cap.
#[derive(Default, Clone, Debug)]
pub struct Damage {
    rects: SmallVec<[Area; MAX_DAMAGE_RECTS]>,
    full: bool,
}

impl Damage {
    /// Add a rect to the damage, merging it with any rect it overlaps.
    pub fn push(&mut self, area: Area) {
        if self.full || area.width() <= 0.0 || area.height() <= 0.0 {
            return;
        }

        let mut merged = area;
        loop {
            let mut changed = false;
            self.rects.retain(|rect| {
                if merged.intersects(rect) {
                    merged = merged.union(rect);
                    changed = true;
                    false
                } else {
                    true
                }
            });
            if !changed {
                break;
            }
        }
        self.rects.push(merged);

        if self.rects.len() > MAX_DAMAGE_RECTS {
            let mut union = self.rects[0];
            for rect in self.rects.iter().skip(1) {
                union = union.union(rect);
            }
            self.rects.clear();
            self.rects.push(union);
        }
    }

    /// Add the bounds of a recording, where `None` means unbounded.
    pub fn push_bounds(&mut self, bounds: Option<Area>) {
        match bounds {
            Some(bounds) => self.push(bounds),
            None => self.mark_full(),
        }
    }

    /// Mark everything as damaged.
    pub fn mark_full(&mut self) {
        self.full = true;
        self.rects.clear();
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    pub fn is_empty(&self) -> bool {
        !self.full && self.rects.is_empty()
    }

    pub fn intersects(&self, area: &Area) -> bool {
        if self.full {
            return true;
        }
        self.rects.iter().any(|rect| rect.intersects(area))
    }

    pub fn contains(&self, area: &Area) -> bool {
        if self.full {
            return true;
        }
        self.rects.iter().any(|rect| rect.contains_rect(area))
    }

    pub fn rects(&self) -> &[Area] {
        &self.rects
    }

    /// Take the accumulated damage, leaving this one empty.
    pub fn take(&mut self) -> Damage {
        std::mem::take(self)
    }
}

#[cfg(test)]
mod test {
    use torin::prelude::{
        Area,
        Point2D,
        Size2D,
    };

    use crate::damage::Damage;

    fn area(x: f32, y: f32, width: f32, height: f32) -> Area {
        Area::new(Point2D::new(x, y), Size2D::new(width, height))
    }

    #[test]
    fn overlapping_rects_merge() {
        let mut damage = Damage::default();
        damage.push(area(0., 0., 50., 50.));
        damage.push(area(25., 25., 50., 50.));
        assert_eq!(damage.rects(), &[area(0., 0., 75., 75.)]);
    }

    #[test]
    fn distant_rects_stay_apart() {
        let mut damage = Damage::default();
        damage.push(area(0., 0., 10., 10.));
        damage.push(area(400., 400., 10., 10.));
        assert_eq!(damage.rects().len(), 2);
        assert!(damage.intersects(&area(5., 5., 1., 1.)));
        assert!(damage.intersects(&area(405., 405., 1., 1.)));
        assert!(!damage.intersects(&area(200., 200., 1., 1.)));
    }

    #[test]
    fn chained_merges_collapse() {
        let mut damage = Damage::default();
        damage.push(area(0., 0., 10., 10.));
        damage.push(area(20., 0., 10., 10.));
        damage.push(area(5., 0., 20., 10.));
        assert_eq!(damage.rects(), &[area(0., 0., 30., 10.)]);
    }

    #[test]
    fn cap_unions_everything() {
        let mut damage = Damage::default();
        for i in 0..12 {
            damage.push(area(i as f32 * 100., 0., 10., 10.));
        }
        assert!(damage.rects().len() <= 8);
        assert!(damage.contains(&area(500., 0., 10., 10.)));
    }

    #[test]
    fn empty_rects_are_ignored() {
        let mut damage = Damage::default();
        damage.push(area(0., 0., 0., 10.));
        assert!(damage.is_empty());
    }

    #[test]
    fn full_damage() {
        let mut damage = Damage::default();
        damage.mark_full();
        assert!(damage.is_full());
        assert!(damage.intersects(&area(0., 0., 1., 1.)));
        let taken = damage.take();
        assert!(taken.is_full());
        assert!(!damage.is_full());
    }
}
