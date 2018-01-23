use libembroidery_sys::{embPattern_create, embPattern_free};

use error::{Result, ErrorKind};
use types::Pattern;

impl Pattern {
    pub fn create() -> Result<Self> {
        Ok(unsafe_pointer_call!{ embPattern_create() }.into())
    }
    pub fn free(self) -> Result<()> {
        unsafe { embPattern_free(self.into()) };
        Ok(())
    }


    // embPattern_addCircleObjectAbs
    // embPattern_addEllipseObjectAbs
    // embPattern_addLineObjectAbs
    // embPattern_addPathObjectAbs
    // embPattern_addPointObjectAbs
    // embPattern_addPolygonObjectAbs
    // embPattern_addPolylineObjectAbs
    // embPattern_addRectObjectAbs
    // embPattern_addStitchAbs
    // embPattern_addStitchRel
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


