use std::cmp::{max, min};

use libembroidery_sys::{
    embPattern_addStitchRel,
    embPattern_addStitchAbs,
    embPattern_center,
};

use error::{Result, ErrorKind};
use flags::StitchFlags;


struct Thread {
    color: (u8, u8, u8),
    name: String,
}
struct Stitch {
    x: f64,
    y: f64,
    flags: u8,
}


struct Pattern {
    stitches: Vec<(Thread, Vec<Stitch>)>,
}

impl Pattern {
    pub fn create() -> Self {
        Pattern {
            stitches: vec![]
        }
    }
    pub fn last_stitch(&self) -> Option<&Stitch> {
        for &(_, stitches) in self.stitches.iter().rev() {
            if let Some(stitch) = stitches.last() {
                return Some(stitch);
            }
        }
        None
    }
    pub fn first_stitch(&self) -> Option<&Stitch> {
        for &(_, stitches) in self.stitches.iter() {
            if let Some(stitch) = stitches.first() {
                return Some(stitch);
            }
        }
        None
    }
    pub fn bounding_box(&self) -> Option<Bounds> {
        let stitch = match self.first_stitch() {
            Some(stitch) => stitch,
            None => { return None },
        };
        let mut min_x = stitch.x;
        let mut max_x = stitch.x;
        let mut min_y = stitch.y;
        let mut max_y = stitch.y;
        for (_, stitch) in self.iter() {
            min_x = max(stitch.x, min_x);
            max_x = max(stitch.x, max_x);
            min_y = max(stitch.y, min_y);
            max_y = max(stitch.y, max_y);
        }
        None
    }

    pub fn iter(&self) -> Iter {

    }


    // pub fn add_stitch_rel(&mut self, dx: f64, dy:f64, flags: StitchFlags, auto_color: bool) -> Result<()> {
    //     unsafe { embPattern_addStitchRel(self.into(), dx, dy, flags.into(), auto_color as i32) };
    //     Ok(())
    // }
    // pub fn add_stitch_abs(&mut self, x: f64, y:f64, flags: StitchFlags, auto_color: bool) -> Result<()> {
    //     unsafe { embPattern_addStitchAbs(self.into(), x, y, flags.into(), auto_color as i32) };
    //     Ok(())
    // }
    // pub fn center_pattern(&mut self) -> Result<()>{
    //     let result = unsafe { embPattern_center(self.into()) };
    //     Ok(result)
    // }

    // pub fn bounding_box(&self) -> Result<

    // embPattern_addThread

    // embPattern_calcBoundingBox
    // embPattern_center
    // embPattern_changeColor
    // embPattern_combineJumpStitches
    // embPattern_copyPolylinesToStitchList
    // embPattern_copyStitchListToPolylines
    // embPattern_correctForMaxStitchLength
    // embPattern_fixColorCount
    // embPattern_flip
    // embPattern_flipHorizontal
    // embPattern_flipVertical
    // embPattern_free
    // embPattern_hideStitchesOverLength
    // embPattern_loadExternalColorFile
    // embPattern_movePolylinesToStitchList
    // embPattern_moveStitchListToPolylines
    // embPattern_read
    // embPattern_scale
    // embPattern_write
}
