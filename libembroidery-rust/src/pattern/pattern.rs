#![allow(warnings)]
use std::ffi;

use libc::c_char;
use libc;

use pattern::utils::color::*;
use pattern::utils::hoop::*;
use pattern::utils::thread::*;
use pattern::arc::*;
use pattern::circle::*;
use pattern::ellipse::*;
use pattern::line::*;
use pattern::path::*;
use pattern::polygon::*;
use pattern::polyline::*;
use pattern::point::*;
use pattern::rect::*;
use pattern::settings::*;
use pattern::spline::*;
use pattern::stitch::*;

use formats::reader_writer::EmbReaderWriter;

extern "C" {
    fn embSettings_home(settings: *mut EmbSettings) -> EmbPoint;
    fn embSettings_init() -> EmbSettings;
    fn strcat(__dest: *mut u8, __src: *const u8) -> *mut u8;
    fn strrchr(__s: *const u8, __c: i32) -> *mut u8;

    fn embReaderWriter_getByFileName(fileName: *const u8) -> *mut EmbReaderWriter;
}


#[derive(Copy)]
#[repr(C)]
pub struct EmbPattern {
    pub settings: EmbSettings,
    pub hoop: EmbHoop,
    pub stitchList: *mut EmbStitchList,
    pub threadList: *mut EmbThreadList,
    pub arcObjList: *mut EmbArcObjectList,
    pub circleObjList: *mut EmbCircleObjectList,
    pub ellipseObjList: *mut EmbEllipseObjectList,
    pub lineObjList: *mut EmbLineObjectList,
    pub pathObjList: *mut EmbPathObjectList,
    pub pointObjList: *mut EmbPointObjectList,
    pub polygonObjList: *mut EmbPolygonObjectList,
    pub polylineObjList: *mut EmbPolylineObjectList,
    pub rectObjList: *mut EmbRectObjectList,
    pub splineObjList: *mut EmbSplineObjectList,
    pub lastStitch: *mut EmbStitchList,
    pub lastThread: *mut EmbThreadList,
    pub lastArcObj: *mut EmbArcObjectList,
    pub lastCircleObj: *mut EmbCircleObjectList,
    pub lastEllipseObj: *mut EmbEllipseObjectList,
    pub lastLineObj: *mut EmbLineObjectList,
    pub lastPathObj: *mut EmbPathObjectList,
    pub lastPointObj: *mut EmbPointObjectList,
    pub lastPolygonObj: *mut EmbPolygonObjectList,
    pub lastPolylineObj: *mut EmbPolylineObjectList,
    pub lastRectObj: *mut EmbRectObjectList,
    pub lastSplineObj: *mut EmbSplineObjectList,
    pub currentColorIndex: i32,
    pub lastX: f64,
    pub lastY: f64,
}

impl Clone for EmbPattern {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_create() -> *mut EmbPattern {
    let mut p: *mut EmbPattern = 0i32 as (*mut EmbPattern);
    p = libc::malloc(::std::mem::size_of::<EmbPattern>()) as (*mut EmbPattern);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_create(), unable to allocate memory for p\n");
        0i32 as (*mut EmbPattern)
    } else {
        (*p).settings = embSettings_init();
        (*p).currentColorIndex = 0i32;
        (*p).stitchList = 0i32 as (*mut EmbStitchList);
        (*p).threadList = 0i32 as (*mut EmbThreadList);
        (*p).hoop.height = 0.0f64;
        (*p).hoop.width = 0.0f64;
        (*p).arcObjList = 0i32 as (*mut EmbArcObjectList);
        (*p).circleObjList = 0i32 as (*mut EmbCircleObjectList);
        (*p).ellipseObjList = 0i32 as (*mut EmbEllipseObjectList);
        (*p).lineObjList = 0i32 as (*mut EmbLineObjectList);
        (*p).pathObjList = 0i32 as (*mut EmbPathObjectList);
        (*p).pointObjList = 0i32 as (*mut EmbPointObjectList);
        (*p).polygonObjList = 0i32 as (*mut EmbPolygonObjectList);
        (*p).polylineObjList = 0i32 as (*mut EmbPolylineObjectList);
        (*p).rectObjList = 0i32 as (*mut EmbRectObjectList);
        (*p).splineObjList = 0i32 as (*mut EmbSplineObjectList);
        (*p).lastStitch = 0i32 as (*mut EmbStitchList);
        (*p).lastThread = 0i32 as (*mut EmbThreadList);
        (*p).lastArcObj = 0i32 as (*mut EmbArcObjectList);
        (*p).lastCircleObj = 0i32 as (*mut EmbCircleObjectList);
        (*p).lastLineObj = 0i32 as (*mut EmbLineObjectList);
        (*p).lastEllipseObj = 0i32 as (*mut EmbEllipseObjectList);
        (*p).lastPathObj = 0i32 as (*mut EmbPathObjectList);
        (*p).lastPointObj = 0i32 as (*mut EmbPointObjectList);
        (*p).lastPolygonObj = 0i32 as (*mut EmbPolygonObjectList);
        (*p).lastPolylineObj = 0i32 as (*mut EmbPolylineObjectList);
        (*p).lastRectObj = 0i32 as (*mut EmbRectObjectList);
        (*p).lastSplineObj = 0i32 as (*mut EmbSplineObjectList);
        (*p).lastX = 0.0f64;
        (*p).lastY = 0.0f64;
        p
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_hideStitchesOverLength(
    mut p: *mut EmbPattern,
    mut length: i32,
) {
    let mut prevX: f64 = 0i32 as (f64);
    let mut prevY: f64 = 0i32 as (f64);
    let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_hideStitchesOverLength(), p argument is null\n");
    } else {
        pointer = (*p).stitchList;
        'loop2: loop {
            if pointer.is_null() {
                break;
            }
            if ((*pointer).stitch.xx - prevX).abs() > length as (f64)
                || ((*pointer).stitch.yy - prevY).abs() > length as (f64)
            {
                (*pointer).stitch.flags = (*pointer).stitch.flags | 2i32;
                (*pointer).stitch.flags = (*pointer).stitch.flags & !0i32;
            }
            prevX = (*pointer).stitch.xx;
            prevY = (*pointer).stitch.yy;
            pointer = (*pointer).next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addThread(
    mut p: *mut EmbPattern,
    mut thread: EmbThread,
) -> i32 {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addThread(), p argument is null\n");
        0i32
    } else {
        if embThreadList_empty((*p).threadList) != 0 {
            (*p).threadList = {
                (*p).lastThread = embThreadList_create(thread);
                (*p).lastThread
            };
        } else {
            (*p).lastThread = embThreadList_add((*p).lastThread, thread);
        }
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_fixColorCount(mut p: *mut EmbPattern) {
    let mut maxColorIndex: i32 = 0i32;
    let mut list: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_fixColorCount(), p argument is null\n");
    } else {
        list = (*p).stitchList;
        'loop2: loop {
            if list.is_null() {
                break;
            }
            maxColorIndex = if maxColorIndex > (*list).stitch.color {
                maxColorIndex
            } else {
                (*list).stitch.color
            };
            list = (*list).next;
        }
        'loop3: loop {
            if !(embThreadList_count((*p).threadList) <= maxColorIndex) {
                break;
            }
            embPattern_addThread(p, embThread_getRandom());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_copyStitchListToPolylines(mut p: *mut EmbPattern) {
    let mut _currentBlock;
    let mut stList: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    let mut breakAtFlags: i32;
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_copyStitchListToPolylines(), p argument is null\n");
    } else {
        breakAtFlags = 4i32 | 1i32 | 2i32;
        stList = (*p).stitchList;
        'loop2: loop {
            if stList.is_null() {
                _currentBlock = 3;
                break;
            }
            let mut pointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
            let mut lastPoint: *mut EmbPointList = 0i32 as (*mut EmbPointList);
            let mut color = EmbColor::black();
            'loop5: loop {
                if stList.is_null() {
                    break;
                }
                if (*stList).stitch.flags & breakAtFlags != 0 {
                    break;
                }
                if (*stList).stitch.flags & 1i32 == 0 {
                    if pointList.is_null() {
                        pointList = {
                            lastPoint =
                                embPointList_create((*stList).stitch.xx, (*stList).stitch.yy);
                            lastPoint
                        };
                        color = embThreadList_getAt((*p).threadList, (*stList).stitch.color).color;
                    } else {
                        lastPoint = embPointList_add(
                            lastPoint,
                            embPoint_make((*stList).stitch.xx, (*stList).stitch.yy),
                        );
                    }
                }
                stList = (*stList).next;
            }
            if !pointList.is_null() {
                let mut currentPolyline: *mut EmbPolylineObject =
                    libc::malloc(::std::mem::size_of::<EmbPolylineObject>())
                        as (*mut EmbPolylineObject);
                if currentPolyline.is_null() {
                    _currentBlock = 19;
                    break;
                }
                (*currentPolyline).pointList = pointList;
                (*currentPolyline).color = color;
                (*currentPolyline).lineType = 1i32;
                if embPolylineObjectList_empty((*p).polylineObjList) != 0 {
                    (*p).polylineObjList = {
                        (*p).lastPolylineObj = embPolylineObjectList_create(currentPolyline);
                        (*p).lastPolylineObj
                    };
                } else {
                    (*p).lastPolylineObj =
                        embPolylineObjectList_add((*p).lastPolylineObj, currentPolyline);
                }
            }
            if stList.is_null() {
                continue;
            }
            stList = (*stList).next;
        }
        (if _currentBlock == 3 {
        } else {
            embLog_error!(
                 "emb-pattern.c embPattern_copyStitchListToPolylines(), cannot allocate memory for currentPolyline\n"
             );
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_copyPolylinesToStitchList(mut p: *mut EmbPattern) {
    let mut _currentBlock;
    let mut polyList: *mut EmbPolylineObjectList = 0i32 as (*mut EmbPolylineObjectList);
    let mut firstObject: i32 = 1i32;
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_copyPolylinesToStitchList(), p argument is null\n");
    } else {
        polyList = (*p).polylineObjList;
        'loop2: loop {
            if polyList.is_null() {
                _currentBlock = 3;
                break;
            }
            let mut currentPoly: *mut EmbPolylineObject = 0i32 as (*mut EmbPolylineObject);
            let mut currentPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
            let mut thread = embThread_getRandom();
            currentPoly = (*polyList).polylineObj;
            if currentPoly.is_null() {
                _currentBlock = 13;
                break;
            }
            currentPointList = (*currentPoly).pointList;
            if currentPointList.is_null() {
                _currentBlock = 12;
                break;
            }
            thread.catalogNumber = 0i32 as (*const i8);
            thread.color = (*currentPoly).color;
            thread.description = 0i32 as (*const i8);
            embPattern_addThread(p, thread);
            if firstObject == 0 {
                embPattern_addStitchAbs(
                    p,
                    (*currentPointList).point.xx,
                    (*currentPointList).point.yy,
                    2i32,
                    1i32,
                );
                embPattern_addStitchRel(p, 0.0f64, 0.0f64, 4i32, 1i32);
            }
            embPattern_addStitchAbs(
                p,
                (*currentPointList).point.xx,
                (*currentPointList).point.yy,
                1i32,
                1i32,
            );
            'loop9: loop {
                if currentPointList.is_null() {
                    break;
                }
                embPattern_addStitchAbs(
                    p,
                    (*currentPointList).point.xx,
                    (*currentPointList).point.yy,
                    0i32,
                    1i32,
                );
                currentPointList = (*currentPointList).next;
            }
            firstObject = 0i32;
            polyList = (*polyList).next;
        }
        (if _currentBlock == 3 {
            embPattern_addStitchRel(p, 0.0f64, 0.0f64, 16i32, 1i32);
        } else if _currentBlock == 12 {
            embLog_error!(
                "emb-pattern.c embPattern_copyPolylinesToStitchList(), currentPointList is null\n"
            );
        } else {
            embLog_error!(
                "emb-pattern.c embPattern_copyPolylinesToStitchList(), currentPoly is null\n"
            );
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_moveStitchListToPolylines(mut p: *mut EmbPattern) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_moveStitchListToPolylines(), p argument is null\n");
    } else {
        embPattern_copyStitchListToPolylines(p);
        embStitchList_free((*p).stitchList);
        (*p).stitchList = 0i32 as (*mut EmbStitchList);
        (*p).lastStitch = 0i32 as (*mut EmbStitchList);
        embThreadList_free((*p).threadList);
        (*p).threadList = 0i32 as (*mut EmbThreadList);
        (*p).lastThread = 0i32 as (*mut EmbThreadList);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_movePolylinesToStitchList(mut p: *mut EmbPattern) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_movePolylinesToStitchList(), p argument is null\n");
    } else {
        embPattern_copyPolylinesToStitchList(p);
        embPolylineObjectList_free((*p).polylineObjList);
        (*p).polylineObjList = 0i32 as (*mut EmbPolylineObjectList);
        (*p).lastPolylineObj = 0i32 as (*mut EmbPolylineObjectList);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addStitchAbs(
    mut p: *mut EmbPattern,
    mut x: f64,
    mut y: f64,
    mut flags: i32,
    mut isAutoColorIndex: i32,
) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addStitchAbs(), p argument is null\n");
    } else {
        if flags & 16i32 != 0 {
            if embStitchList_empty((*p).stitchList) != 0 {
                return;
            } else if (*(*p).lastStitch).stitch.flags & 16i32 != 0 {
                embLog_error!(
                    "emb-pattern.c embPattern_addStitchAbs(), found multiple END stitches\n"
                );
                return;
            } else {
                embPattern_fixColorCount(p);
            }
        }
        if flags & 4i32 != 0 {
            if embStitchList_empty((*p).stitchList) != 0 {
                return;
            } else if isAutoColorIndex != 0 {
                (*p).currentColorIndex = (*p).currentColorIndex + 1;
            }
        }
        if embStitchList_empty((*p).stitchList) != 0 {
            let mut home: EmbPoint = embSettings_home(&mut (*p).settings as (*mut EmbSettings));
            let mut h = EmbStitch {
                xx: home.xx,
                yy: home.yy,
                flags: 1i32,
                color: (*p).currentColorIndex,
            };
            (*p).stitchList = {
                (*p).lastStitch = embStitchList_create(h);
                (*p).lastStitch
            };
        }
        let mut s = EmbStitch {
            xx: x,
            yy: y,
            flags: flags,
            color: (*p).currentColorIndex,
        };
        (*p).lastStitch = embStitchList_add((*p).lastStitch, s);
        (*p).lastX = s.xx;
        (*p).lastY = s.yy;
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addStitchRel(
    mut p: *mut EmbPattern,
    mut dx: f64,
    mut dy: f64,
    mut flags: i32,
    mut isAutoColorIndex: i32,
) {
    let mut x: f64;
    let mut y: f64;
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addStitchRel(), p argument is null\n");
    } else {
        if embStitchList_empty((*p).stitchList) == 0 {
            x = (*p).lastX + dx;
            y = (*p).lastY + dy;
        } else {
            let mut home: EmbPoint = embSettings_home(&mut (*p).settings as (*mut EmbSettings));
            x = home.xx + dx;
            y = home.yy + dy;
        }
        embPattern_addStitchAbs(p, x, y, flags, isAutoColorIndex);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_changeColor(mut p: *mut EmbPattern, mut index: i32) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_changeColor(), p argument is null\n");
    } else {
        (*p).currentColorIndex = index;
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_read(
    mut pattern: *mut EmbPattern,
    mut fileName: *const u8,
) -> i32 {
    let mut reader: *mut EmbReaderWriter = 0i32 as (*mut EmbReaderWriter);
    let mut result: i32 = 0i32;
    if pattern.is_null() {
        embLog_error!("emb-pattern.c embPattern_read(), pattern argument is null\n");
        0i32
    } else if fileName.is_null() {
        embLog_error!("emb-pattern.c embPattern_read(), fileName argument is null\n");
        0i32
    } else {
        reader = embReaderWriter_getByFileName(fileName);
        (if reader.is_null() {
            embLog_error!(
                "emb-pattern.c embPattern_read(), unsupported read file type: {}\n",
                ffi::CStr::from_ptr(fileName as *const i8).to_string_lossy()
            );
            0i32
        } else {
            result = ((*reader).reader)(pattern, fileName);
            libc::free(reader as (*mut libc::c_void));
            reader = 0i32 as (*mut EmbReaderWriter);
            result
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_write(
    mut pattern: *mut EmbPattern,
    mut fileName: *const u8,
) -> i32 {
    let mut writer: *mut EmbReaderWriter = 0i32 as (*mut EmbReaderWriter);
    let mut result: i32 = 0i32;
    if pattern.is_null() {
        embLog_error!("emb-pattern.c embPattern_write(), pattern argument is null\n");
        0i32
    } else if fileName.is_null() {
        embLog_error!("emb-pattern.c embPattern_write(), fileName argument is null\n");
        0i32
    } else {
        writer = embReaderWriter_getByFileName(fileName);
        (if writer.is_null() {
            embLog_error!(
                "emb-pattern.c embPattern_write(), unsupported write file type: {}\n",
                ffi::CStr::from_ptr(fileName as *const i8).to_string_lossy()
            );
            0i32
        } else {
            result = ((*writer).writer)(pattern, fileName);
            libc::free(writer as (*mut libc::c_void));
            writer = 0i32 as (*mut EmbReaderWriter);
            result
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_scale(mut p: *mut EmbPattern, mut scale: f64) {
    let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_scale(), p argument is null\n");
    } else {
        pointer = (*p).stitchList;
        'loop2: loop {
            if pointer.is_null() {
                break;
            }
            (*pointer).stitch.xx = (*pointer).stitch.xx * scale;
            (*pointer).stitch.yy = (*pointer).stitch.yy * scale;
            pointer = (*pointer).next;
        }
    }
}


#[no_mangle]
pub unsafe extern "C" fn embPattern_calcBoundingBox(mut p: *mut EmbPattern) -> EmbRect {
    let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    let mut boundingRect = EmbRect {
        bottom: 0f64,
        top: 0f64,
        left: 0f64,
        right: 0f64,
    };
    let mut pt: EmbStitch;
    let mut aObjList: *mut EmbArcObjectList = 0i32 as (*mut EmbArcObjectList);
    let mut arc: EmbArc;
    let mut cObjList: *mut EmbCircleObjectList = 0i32 as (*mut EmbCircleObjectList);
    let mut circle: EmbCircle;
    let mut eObjList: *mut EmbEllipseObjectList = 0i32 as (*mut EmbEllipseObjectList);
    let mut ellipse: EmbEllipse;
    let mut liObjList: *mut EmbLineObjectList = 0i32 as (*mut EmbLineObjectList);
    let mut line: EmbLine;
    let mut pObjList: *mut EmbPointObjectList = 0i32 as (*mut EmbPointObjectList);
    let mut point: EmbPoint;
    let mut pogObjList: *mut EmbPolygonObjectList = 0i32 as (*mut EmbPolygonObjectList);
    let mut pogPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    let mut pogPoint: EmbPoint;
    let mut polObjList: *mut EmbPolylineObjectList = 0i32 as (*mut EmbPolylineObjectList);
    let mut polPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    let mut polPoint: EmbPoint;
    let mut rObjList: *mut EmbRectObjectList = 0i32 as (*mut EmbRectObjectList);
    let mut rect: EmbRect;
    let mut sObjList: *mut EmbSplineObjectList = 0i32 as (*mut EmbSplineObjectList);
    let mut bezier: EmbBezier;
    boundingRect.left = 0i32 as (f64);
    boundingRect.right = 0i32 as (f64);
    boundingRect.top = 0i32 as (f64);
    boundingRect.bottom = 0i32 as (f64);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_calcBoundingBox(), p argument is null\n");
        boundingRect
    } else if (embStitchList_empty((*p).stitchList) != 0)
        && (embArcObjectList_empty((*p).arcObjList) != 0)
        && (embCircleObjectList_empty((*p).circleObjList) != 0)
        && (embEllipseObjectList_empty((*p).ellipseObjList) != 0)
        && (embLineObjectList_empty((*p).lineObjList) != 0)
        && (embPointObjectList_empty((*p).pointObjList) != 0)
        && (embPolygonObjectList_empty((*p).polygonObjList) != 0)
        && (embPolylineObjectList_empty((*p).polylineObjList) != 0)
        && (embRectObjectList_empty((*p).rectObjList) != 0)
        && (embSplineObjectList_empty((*p).splineObjList) != 0)
    {
        boundingRect.top = 0.0f64;
        boundingRect.left = 0.0f64;
        boundingRect.bottom = 1.0f64;
        boundingRect.right = 1.0f64;
        boundingRect
    } else {
        boundingRect.left = 99999.0f64;
        boundingRect.top = 99999.0f64;
        boundingRect.right = -99999.0f64;
        boundingRect.bottom = -99999.0f64;
        pointer = (*p).stitchList;
        'loop3: loop {
            if pointer.is_null() {
                break;
            }
            pt = (*pointer).stitch;
            if pt.flags & 2i32 == 0 {
                boundingRect.left = if boundingRect.left < pt.xx {
                    boundingRect.left
                } else {
                    pt.xx
                };
                boundingRect.top = if boundingRect.top < pt.yy {
                    boundingRect.top
                } else {
                    pt.yy
                };
                boundingRect.right = if boundingRect.right > pt.xx {
                    boundingRect.right
                } else {
                    pt.xx
                };
                boundingRect.bottom = if boundingRect.bottom > pt.yy {
                    boundingRect.bottom
                } else {
                    pt.yy
                };
            }
            pointer = (*pointer).next;
        }
        aObjList = (*p).arcObjList;
        'loop5: loop {
            if aObjList.is_null() {
                break;
            }
            arc = (*aObjList).arcObj.arc;
            aObjList = (*aObjList).next;
        }
        cObjList = (*p).circleObjList;
        'loop7: loop {
            if cObjList.is_null() {
                break;
            }
            circle = (*cObjList).circleObj.circle;
            boundingRect.left = if boundingRect.left < circle.centerX - circle.radius {
                boundingRect.left
            } else {
                circle.centerX - circle.radius
            };
            boundingRect.top = if boundingRect.top < circle.centerY - circle.radius {
                boundingRect.top
            } else {
                circle.centerY - circle.radius
            };
            boundingRect.right = if boundingRect.right > circle.centerX + circle.radius {
                boundingRect.right
            } else {
                circle.centerX + circle.radius
            };
            boundingRect.bottom = if boundingRect.bottom > circle.centerY + circle.radius {
                boundingRect.bottom
            } else {
                circle.centerY + circle.radius
            };
            cObjList = (*cObjList).next;
        }
        eObjList = (*p).ellipseObjList;
        'loop9: loop {
            if eObjList.is_null() {
                break;
            }
            ellipse = (*eObjList).ellipseObj.ellipse;
            eObjList = (*eObjList).next;
        }
        liObjList = (*p).lineObjList;
        'loop11: loop {
            if liObjList.is_null() {
                break;
            }
            line = (*liObjList).lineObj.line;
            liObjList = (*liObjList).next;
        }
        pObjList = (*p).pointObjList;
        'loop13: loop {
            if pObjList.is_null() {
                break;
            }
            point = (*pObjList).pointObj.point;
            pObjList = (*pObjList).next;
        }
        pogObjList = (*p).polygonObjList;
        'loop15: loop {
            if pogObjList.is_null() {
                break;
            }
            pogPointList = (*(*pogObjList).polygonObj).pointList;
            'loop30: loop {
                if pogPointList.is_null() {
                    break;
                }
                pogPoint = (*pogPointList).point;
                pogPointList = (*pogPointList).next;
            }
            pogObjList = (*pogObjList).next;
        }
        polObjList = (*p).polylineObjList;
        'loop17: loop {
            if polObjList.is_null() {
                break;
            }
            polPointList = (*(*polObjList).polylineObj).pointList;
            'loop26: loop {
                if polPointList.is_null() {
                    break;
                }
                polPoint = (*polPointList).point;
                polPointList = (*polPointList).next;
            }
            polObjList = (*polObjList).next;
        }
        rObjList = (*p).rectObjList;
        'loop19: loop {
            if rObjList.is_null() {
                break;
            }
            rect = (*rObjList).rectObj.rect;
            rObjList = (*rObjList).next;
        }
        sObjList = (*p).splineObjList;
        'loop21: loop {
            if sObjList.is_null() {
                break;
            }
            bezier = (*sObjList).splineObj.bezier;
            sObjList = (*sObjList).next;
        }
        boundingRect
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_flipHorizontal(mut p: *mut EmbPattern) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_flipHorizontal(), p argument is null\n");
    } else {
        embPattern_flip(p, 1i32, 0i32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_flipVertical(mut p: *mut EmbPattern) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_flipVertical(), p argument is null\n");
    } else {
        embPattern_flip(p, 0i32, 1i32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_flip(mut p: *mut EmbPattern, mut horz: i32, mut vert: i32) {
    let mut stList: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    let mut aObjList: *mut EmbArcObjectList = 0i32 as (*mut EmbArcObjectList);
    let mut cObjList: *mut EmbCircleObjectList = 0i32 as (*mut EmbCircleObjectList);
    let mut eObjList: *mut EmbEllipseObjectList = 0i32 as (*mut EmbEllipseObjectList);
    let mut liObjList: *mut EmbLineObjectList = 0i32 as (*mut EmbLineObjectList);
    let mut paObjList: *mut EmbPathObjectList = 0i32 as (*mut EmbPathObjectList);
    let mut paPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    let mut pObjList: *mut EmbPointObjectList = 0i32 as (*mut EmbPointObjectList);
    let mut pogObjList: *mut EmbPolygonObjectList = 0i32 as (*mut EmbPolygonObjectList);
    let mut pogPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    let mut polObjList: *mut EmbPolylineObjectList = 0i32 as (*mut EmbPolylineObjectList);
    let mut polPointList: *mut EmbPointList = 0i32 as (*mut EmbPointList);
    let mut rObjList: *mut EmbRectObjectList = 0i32 as (*mut EmbRectObjectList);
    let mut sObjList: *mut EmbSplineObjectList = 0i32 as (*mut EmbSplineObjectList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_flip(), p argument is null\n");
    } else {
        stList = (*p).stitchList;
        'loop2: loop {
            if stList.is_null() {
                break;
            }
            if horz != 0 {
                (*stList).stitch.xx = -(*stList).stitch.xx;
            }
            if vert != 0 {
                (*stList).stitch.yy = -(*stList).stitch.yy;
            }
            stList = (*stList).next;
        }
        aObjList = (*p).arcObjList;
        'loop4: loop {
            if aObjList.is_null() {
                break;
            }
            aObjList = (*aObjList).next;
        }
        cObjList = (*p).circleObjList;
        'loop6: loop {
            if cObjList.is_null() {
                break;
            }
            if horz != 0 {
                (*cObjList).circleObj.circle.centerX = -(*cObjList).circleObj.circle.centerX;
            }
            if vert != 0 {
                (*cObjList).circleObj.circle.centerY = -(*cObjList).circleObj.circle.centerY;
            }
            cObjList = (*cObjList).next;
        }
        eObjList = (*p).ellipseObjList;
        'loop8: loop {
            if eObjList.is_null() {
                break;
            }
            if horz != 0 {
                (*eObjList).ellipseObj.ellipse.centerX = -(*eObjList).ellipseObj.ellipse.centerX;
            }
            if vert != 0 {
                (*eObjList).ellipseObj.ellipse.centerY = -(*eObjList).ellipseObj.ellipse.centerY;
            }
            eObjList = (*eObjList).next;
        }
        liObjList = (*p).lineObjList;
        'loop10: loop {
            if liObjList.is_null() {
                break;
            }
            if horz != 0 {
                (*liObjList).lineObj.line.x1 = -(*liObjList).lineObj.line.x1;
                (*liObjList).lineObj.line.x2 = -(*liObjList).lineObj.line.x2;
            }
            if vert != 0 {
                (*liObjList).lineObj.line.y1 = -(*liObjList).lineObj.line.y1;
                (*liObjList).lineObj.line.y2 = -(*liObjList).lineObj.line.y2;
            }
            liObjList = (*liObjList).next;
        }
        paObjList = (*p).pathObjList;
        'loop12: loop {
            if paObjList.is_null() {
                break;
            }
            paPointList = (*(*paObjList).pathObj).pointList;
            'loop52: loop {
                if paPointList.is_null() {
                    break;
                }
                if horz != 0 {
                    (*paPointList).point.xx = -(*paPointList).point.xx;
                }
                if vert != 0 {
                    (*paPointList).point.yy = -(*paPointList).point.yy;
                }
                paPointList = (*paPointList).next;
            }
            paObjList = (*paObjList).next;
        }
        pObjList = (*p).pointObjList;
        'loop14: loop {
            if pObjList.is_null() {
                break;
            }
            if horz != 0 {
                (*pObjList).pointObj.point.xx = -(*pObjList).pointObj.point.xx;
            }
            if vert != 0 {
                (*pObjList).pointObj.point.yy = -(*pObjList).pointObj.point.yy;
            }
            pObjList = (*pObjList).next;
        }
        pogObjList = (*p).polygonObjList;
        'loop16: loop {
            if pogObjList.is_null() {
                break;
            }
            pogPointList = (*(*pogObjList).polygonObj).pointList;
            'loop39: loop {
                if pogPointList.is_null() {
                    break;
                }
                if horz != 0 {
                    (*pogPointList).point.xx = -(*pogPointList).point.xx;
                }
                if vert != 0 {
                    (*pogPointList).point.yy = -(*pogPointList).point.yy;
                }
                pogPointList = (*pogPointList).next;
            }
            pogObjList = (*pogObjList).next;
        }
        polObjList = (*p).polylineObjList;
        'loop18: loop {
            if polObjList.is_null() {
                break;
            }
            polPointList = (*(*polObjList).polylineObj).pointList;
            'loop31: loop {
                if polPointList.is_null() {
                    break;
                }
                if horz != 0 {
                    (*polPointList).point.xx = -(*polPointList).point.xx;
                }
                if vert != 0 {
                    (*polPointList).point.yy = -(*polPointList).point.yy;
                }
                polPointList = (*polPointList).next;
            }
            polObjList = (*polObjList).next;
        }
        rObjList = (*p).rectObjList;
        'loop20: loop {
            if rObjList.is_null() {
                break;
            }
            if horz != 0 {
                (*rObjList).rectObj.rect.left = -(*rObjList).rectObj.rect.left;
                (*rObjList).rectObj.rect.right = -(*rObjList).rectObj.rect.right;
            }
            if vert != 0 {
                (*rObjList).rectObj.rect.top = -(*rObjList).rectObj.rect.top;
                (*rObjList).rectObj.rect.bottom = -(*rObjList).rectObj.rect.bottom;
            }
            rObjList = (*rObjList).next;
        }
        sObjList = (*p).splineObjList;
        'loop22: loop {
            if sObjList.is_null() {
                break;
            }
            sObjList = (*sObjList).next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_combineJumpStitches(mut p: *mut EmbPattern) {
    let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    let mut jumpCount: i32 = 0i32;
    let mut jumpListStart: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_combineJumpStitches(), p argument is null\n");
    } else {
        pointer = (*p).stitchList;
        'loop2: loop {
            if pointer.is_null() {
                break;
            }
            if (*pointer).stitch.flags & 1i32 != 0 {
                if jumpCount == 0i32 {
                    jumpListStart = pointer;
                }
                jumpCount = jumpCount + 1;
            } else if jumpCount > 0i32 {
                let mut removePointer: *mut EmbStitchList = (*jumpListStart).next;
                (*jumpListStart).stitch.xx = (*pointer).stitch.xx;
                (*jumpListStart).stitch.yy = (*pointer).stitch.yy;
                (*jumpListStart).next = pointer;
                'loop7: loop {
                    if !(jumpCount > 0i32) {
                        break;
                    }
                    let mut tempPointer: *mut EmbStitchList = (*removePointer).next;
                    libc::free(removePointer as (*mut libc::c_void));
                    removePointer = tempPointer;
                    jumpCount = jumpCount - 1;
                }
                jumpCount = 0i32;
            }
            pointer = (*pointer).next;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_correctForMaxStitchLength(
    mut p: *mut EmbPattern,
    mut maxStitchLength: f64,
    mut maxJumpLength: f64,
) {
    let mut _currentBlock;
    let mut j: i32 = 0i32;
    let mut splits: i32;
    let mut maxXY: f64;
    let mut maxLen: f64;
    let mut addX: f64;
    let mut addY: f64;
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_correctForMaxStitchLength(), p argument is null\n");
    } else {
        if embStitchList_count((*p).stitchList) > 1i32 {
            let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
            let mut prev: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
            prev = (*p).stitchList;
            pointer = (*prev).next;
            'loop3: loop {
                if pointer.is_null() {
                    _currentBlock = 4;
                    break;
                }
                let mut xx: f64 = (*prev).stitch.xx;
                let mut yy: f64 = (*prev).stitch.yy;
                let mut dx: f64 = (*pointer).stitch.xx - xx;
                let mut dy: f64 = (*pointer).stitch.yy - yy;
                if (dx).abs() > maxStitchLength || (dy).abs() > maxStitchLength {
                    maxXY = if (dx).abs() > (dy).abs() {
                        (dx).abs()
                    } else {
                        (dy).abs()
                    };
                    if (*pointer).stitch.flags & (1i32 | 2i32) != 0 {
                        maxLen = maxJumpLength;
                    } else {
                        maxLen = maxStitchLength;
                    }
                    splits = (maxXY / maxLen).ceil() as (i32);
                    if splits > 1i32 {
                        let mut flagsToUse: i32 = (*pointer).stitch.flags;
                        let mut colorToUse: i32 = (*pointer).stitch.color;
                        addX = dx / splits as (f64);
                        addY = dy / splits as (f64);
                        j = 1i32;
                        'loop13: loop {
                            if !(j < splits) {
                                break;
                            }
                            let mut item: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
                            let mut s = EmbStitch {
                                xx: xx + addX * j as (f64),
                                yy: yy + addY * j as (f64),
                                flags: flagsToUse,
                                color: colorToUse,
                            };
                            item = libc::malloc(::std::mem::size_of::<EmbStitchList>())
                                as (*mut EmbStitchList);
                            if item.is_null() {
                                _currentBlock = 19;
                                break 'loop3;
                            }
                            (*item).stitch = s;
                            (*item).next = pointer;
                            (*prev).next = item;
                            prev = item;
                            j = j + 1;
                        }
                    }
                }
                prev = pointer;
                if pointer.is_null() {
                    continue;
                }
                pointer = (*pointer).next;
            }
            if _currentBlock == 4 {
            } else {
                embLog_error!(
                    "emb-pattern.c embPattern_correctForMaxStitchLength(), cannot allocate memory for item\n"
                );
                return;
            }
        }
        if !(*p).lastStitch.is_null() && ((*(*p).lastStitch).stitch.flags != 16i32) {
            embPattern_addStitchAbs(
                p,
                (*(*p).lastStitch).stitch.xx,
                (*(*p).lastStitch).stitch.yy,
                16i32,
                1i32,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_center(mut p: *mut EmbPattern) {
    let mut moveLeft: i32;
    let mut moveTop: i32;
    let mut boundingRect: EmbRect;
    let mut pointer: *mut EmbStitchList = 0i32 as (*mut EmbStitchList);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_center(), p argument is null\n");
    } else {
        boundingRect = embPattern_calcBoundingBox(p);
        moveLeft = (boundingRect.left - embRect_width(boundingRect) / 2.0f64) as (i32);
        moveTop = (boundingRect.top - embRect_height(boundingRect) / 2.0f64) as (i32);
        pointer = (*p).stitchList;
        'loop2: loop {
            if pointer.is_null() {
                break;
            }
            let mut s: EmbStitch;
            s = (*pointer).stitch;
            s.xx = s.xx - moveLeft as (f64);
            s.yy = s.yy - moveTop as (f64);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_loadExternalColorFile(
    mut p: *mut EmbPattern,
    mut fileName: *const c_char,
) {
    let mut hasRead: u8 = 0u8;
    let mut colorFile: *mut EmbReaderWriter = 0i32 as (*mut EmbReaderWriter);
    let mut dotPos: *const u8 = strrchr(fileName as *const u8, b'.' as (i32)) as (*const u8);
    let mut extractName: *mut u8 = 0i32 as (*mut u8);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_loadExternalColorFile(), p argument is null\n");
    } else if fileName.is_null() {
        embLog_error!(
            "emb-pattern.c embPattern_loadExternalColorFile(), fileName argument is null\n"
        );
    } else {
        extractName = libc::malloc(
            ((dotPos as (isize)).wrapping_sub(fileName as (isize))
                / ::std::mem::size_of::<u8>() as (isize) + 5isize) as (usize),
        ) as (*mut u8);
        (if extractName.is_null() {
            embLog_error!(
                 "emb-pattern.c embPattern_loadExternalColorFile(), cannot allocate memory for extractName\n"
             );
        } else {
            extractName = libc::memcpy(
                extractName as (*mut libc::c_void),
                fileName as (*const libc::c_void),
                ((dotPos as (isize)).wrapping_sub(fileName as (isize))
                    / ::std::mem::size_of::<u8>() as (isize)) as (usize),
            ) as (*mut u8);
            *extractName.offset(
                (dotPos as (isize)).wrapping_sub(fileName as (isize))
                    / ::std::mem::size_of::<u8>() as (isize),
            ) = b'\0';
            strcat(extractName, (*b".edr\0").as_ptr());
            colorFile = embReaderWriter_getByFileName(extractName as (*const u8));
            if !colorFile.is_null() {
                hasRead = ((*colorFile).reader)(p, extractName as (*const u8)) as (u8);
            }
            if hasRead == 0 {
                libc::free(colorFile as (*mut libc::c_void));
                colorFile = 0i32 as (*mut EmbReaderWriter);
                extractName = libc::memcpy(
                    extractName as (*mut libc::c_void),
                    fileName as (*const libc::c_void),
                    ((dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize)) as (usize),
                ) as (*mut u8);
                *extractName.offset(
                    (dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize),
                ) = b'\0';
                strcat(extractName, (*b".rgb\0").as_ptr());
                colorFile = embReaderWriter_getByFileName(extractName as (*const u8));
                if !colorFile.is_null() {
                    hasRead = ((*colorFile).reader)(p, extractName as (*const u8)) as (u8);
                }
            }
            if hasRead == 0 {
                libc::free(colorFile as (*mut libc::c_void));
                colorFile = 0i32 as (*mut EmbReaderWriter);
                extractName = libc::memcpy(
                    extractName as (*mut libc::c_void),
                    fileName as (*const libc::c_void),
                    ((dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize)) as (usize),
                ) as (*mut u8);
                *extractName.offset(
                    (dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize),
                ) = b'\0';
                strcat(extractName, (*b".col\0").as_ptr());
                colorFile = embReaderWriter_getByFileName(extractName as (*const u8));
                if !colorFile.is_null() {
                    hasRead = ((*colorFile).reader)(p, extractName as (*const u8)) as (u8);
                }
            }
            if hasRead == 0 {
                libc::free(colorFile as (*mut libc::c_void));
                colorFile = 0i32 as (*mut EmbReaderWriter);
                extractName = libc::memcpy(
                    extractName as (*mut libc::c_void),
                    fileName as (*const libc::c_void),
                    ((dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize)) as (usize),
                ) as (*mut u8);
                *extractName.offset(
                    (dotPos as (isize)).wrapping_sub(fileName as (isize))
                        / ::std::mem::size_of::<u8>() as (isize),
                ) = b'\0';
                strcat(extractName, (*b".inf\0").as_ptr());
                colorFile = embReaderWriter_getByFileName(extractName as (*const u8));
                if !colorFile.is_null() {
                    hasRead = ((*colorFile).reader)(p, extractName as (*const u8)) as (u8);
                }
            }
            libc::free(colorFile as (*mut libc::c_void));
            colorFile = 0i32 as (*mut EmbReaderWriter);
            libc::free(extractName as (*mut libc::c_void));
            extractName = 0i32 as (*mut u8);
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_free(mut p: *mut EmbPattern) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_free(), p argument is null\n");
    } else {
        embStitchList_free((*p).stitchList);
        (*p).stitchList = 0i32 as (*mut EmbStitchList);
        (*p).lastStitch = 0i32 as (*mut EmbStitchList);
        embThreadList_free((*p).threadList);
        (*p).threadList = 0i32 as (*mut EmbThreadList);
        (*p).lastThread = 0i32 as (*mut EmbThreadList);
        embArcObjectList_free((*p).arcObjList);
        (*p).arcObjList = 0i32 as (*mut EmbArcObjectList);
        (*p).lastArcObj = 0i32 as (*mut EmbArcObjectList);
        embCircleObjectList_free((*p).circleObjList);
        (*p).circleObjList = 0i32 as (*mut EmbCircleObjectList);
        (*p).lastCircleObj = 0i32 as (*mut EmbCircleObjectList);
        embEllipseObjectList_free((*p).ellipseObjList);
        (*p).ellipseObjList = 0i32 as (*mut EmbEllipseObjectList);
        (*p).lastEllipseObj = 0i32 as (*mut EmbEllipseObjectList);
        embLineObjectList_free((*p).lineObjList);
        (*p).lineObjList = 0i32 as (*mut EmbLineObjectList);
        (*p).lastLineObj = 0i32 as (*mut EmbLineObjectList);
        embPathObjectList_free((*p).pathObjList);
        (*p).pathObjList = 0i32 as (*mut EmbPathObjectList);
        (*p).lastPathObj = 0i32 as (*mut EmbPathObjectList);
        embPointObjectList_free((*p).pointObjList);
        (*p).pointObjList = 0i32 as (*mut EmbPointObjectList);
        (*p).lastPointObj = 0i32 as (*mut EmbPointObjectList);
        embPolygonObjectList_free((*p).polygonObjList);
        (*p).polygonObjList = 0i32 as (*mut EmbPolygonObjectList);
        (*p).lastPolygonObj = 0i32 as (*mut EmbPolygonObjectList);
        embPolylineObjectList_free((*p).polylineObjList);
        (*p).polylineObjList = 0i32 as (*mut EmbPolylineObjectList);
        (*p).lastPolylineObj = 0i32 as (*mut EmbPolylineObjectList);
        embRectObjectList_free((*p).rectObjList);
        (*p).rectObjList = 0i32 as (*mut EmbRectObjectList);
        (*p).lastRectObj = 0i32 as (*mut EmbRectObjectList);
        libc::free(p as (*mut libc::c_void));
        p = 0i32 as (*mut EmbPattern);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addCircleObjectAbs(
    mut p: *mut EmbPattern,
    mut cx: f64,
    mut cy: f64,
    mut r: f64,
) {
    let mut circleObj: EmbCircleObject = embCircleObject_make(cx, cy, r);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addCircleObjectAbs(), p argument is null\n");
    } else if embCircleObjectList_empty((*p).circleObjList) != 0 {
        (*p).circleObjList = {
            (*p).lastCircleObj = embCircleObjectList_create(circleObj);
            (*p).lastCircleObj
        };
    } else {
        (*p).lastCircleObj = embCircleObjectList_add((*p).lastCircleObj, circleObj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addEllipseObjectAbs(
    mut p: *mut EmbPattern,
    mut cx: f64,
    mut cy: f64,
    mut rx: f64,
    mut ry: f64,
) {
    let mut ellipseObj: EmbEllipseObject = embEllipseObject_make(cx, cy, rx, ry);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addEllipseObjectAbs(), p argument is null\n");
    } else if embEllipseObjectList_empty((*p).ellipseObjList) != 0 {
        (*p).ellipseObjList = {
            (*p).lastEllipseObj = embEllipseObjectList_create(ellipseObj);
            (*p).lastEllipseObj
        };
    } else {
        (*p).lastEllipseObj = embEllipseObjectList_add((*p).lastEllipseObj, ellipseObj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addLineObjectAbs(
    mut p: *mut EmbPattern,
    mut x1: f64,
    mut y1: f64,
    mut x2: f64,
    mut y2: f64,
) {
    let mut lineObj: EmbLineObject = embLineObject_make(x1, y1, x2, y2);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addLineObjectAbs(), p argument is null\n");
    } else if embLineObjectList_empty((*p).lineObjList) != 0 {
        (*p).lineObjList = {
            (*p).lastLineObj = embLineObjectList_create(lineObj);
            (*p).lastLineObj
        };
    } else {
        (*p).lastLineObj = embLineObjectList_add((*p).lastLineObj, lineObj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addPathObjectAbs(
    mut p: *mut EmbPattern,
    mut obj: *mut EmbPathObject,
) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPathObjectAbs(), p argument is null\n");
    } else if obj.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPathObjectAbs(), obj argument is null\n");
    } else if embPointList_empty((*obj).pointList) != 0 {
        embLog_error!("emb-pattern.c embPattern_addPathObjectAbs(), obj->pointList is empty\n");
    } else if embPathObjectList_empty((*p).pathObjList) != 0 {
        (*p).pathObjList = {
            (*p).lastPathObj = embPathObjectList_create(obj);
            (*p).lastPathObj
        };
    } else {
        (*p).lastPathObj = embPathObjectList_add((*p).lastPathObj, obj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addPointObjectAbs(
    mut p: *mut EmbPattern,
    mut x: f64,
    mut y: f64,
) {
    let mut pointObj: EmbPointObject = embPointObject_make(x, y);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPointObjectAbs(), p argument is null\n");
    } else if embPointObjectList_empty((*p).pointObjList) != 0 {
        (*p).pointObjList = {
            (*p).lastPointObj = embPointObjectList_create(pointObj);
            (*p).lastPointObj
        };
    } else {
        (*p).lastPointObj = embPointObjectList_add((*p).lastPointObj, pointObj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addPolygonObjectAbs(
    mut p: *mut EmbPattern,
    mut obj: *mut EmbPolygonObject,
) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPolygonObjectAbs(), p argument is null\n");
    } else if obj.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPolygonObjectAbs(), obj argument is null\n");
    } else if embPointList_empty((*obj).pointList) != 0 {
        embLog_error!("emb-pattern.c embPattern_addPolygonObjectAbs(), obj->pointList is empty\n");
    } else if embPolygonObjectList_empty((*p).polygonObjList) != 0 {
        (*p).polygonObjList = {
            (*p).lastPolygonObj = embPolygonObjectList_create(obj);
            (*p).lastPolygonObj
        };
    } else {
        (*p).lastPolygonObj = embPolygonObjectList_add((*p).lastPolygonObj, obj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addPolylineObjectAbs(
    mut p: *mut EmbPattern,
    mut obj: *mut EmbPolylineObject,
) {
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPolylineObjectAbs(), p argument is null\n");
    } else if obj.is_null() {
        embLog_error!("emb-pattern.c embPattern_addPolylineObjectAbs(), obj argument is null\n");
    } else if embPointList_empty((*obj).pointList) != 0 {
        embLog_error!("emb-pattern.c embPattern_addPolylineObjectAbs(), obj->pointList is empty\n");
    } else if embPolylineObjectList_empty((*p).polylineObjList) != 0 {
        (*p).polylineObjList = {
            (*p).lastPolylineObj = embPolylineObjectList_create(obj);
            (*p).lastPolylineObj
        };
    } else {
        (*p).lastPolylineObj = embPolylineObjectList_add((*p).lastPolylineObj, obj);
    }
}

#[no_mangle]
pub unsafe extern "C" fn embPattern_addRectObjectAbs(
    mut p: *mut EmbPattern,
    mut x: f64,
    mut y: f64,
    mut w: f64,
    mut h: f64,
) {
    let mut rectObj: EmbRectObject = embRectObject_make(x, y, w, h);
    if p.is_null() {
        embLog_error!("emb-pattern.c embPattern_addRectObjectAbs(), p argument is null\n");
    } else if embRectObjectList_empty((*p).rectObjList) != 0 {
        (*p).rectObjList = {
            (*p).lastRectObj = embRectObjectList_create(rectObj);
            (*p).lastRectObj
        };
    } else {
        (*p).lastRectObj = embRectObjectList_add((*p).lastRectObj, rectObj);
    }
}
