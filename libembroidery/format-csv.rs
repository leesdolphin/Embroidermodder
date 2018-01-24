extern {
    fn atof(__nptr : *const u8) -> f64;
    fn atoi(__nptr : *const u8) -> i32;

    fn embFile_printf(
        stream : *mut EmbFile_, format : *const u8, ...
    ) -> i32;
    fn embLog_error(format : *const u8, ...);
    fn embPattern_addStitchAbs(
        p : *mut EmbPattern_,
        x : f64,
        y : f64,
        flags : i32,
        isAutoColorIndex : i32
    );
    fn embPattern_addStitchRel(
        p : *mut EmbPattern_,
        dx : f64,
        dy : f64,
        flags : i32,
        isAutoColorIndex : i32
    );
    fn embPattern_addThread(
        p : *mut EmbPattern_, thread : EmbThread_
    ) -> i32;
    fn embPattern_calcBoundingBox(p : *mut EmbPattern_) -> EmbRect_;
    fn embRect_height(rect : EmbRect_) -> f64;
    fn embRect_width(rect : EmbRect_) -> f64;
    fn embStitchList_count(pointer : *mut EmbStitchList_) -> i32;
    fn embThreadList_count(pointer : *mut EmbThreadList_) -> i32;
    fn embThread_getRandom() -> EmbThread_;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn malloc(__size : usize) -> *mut ::std::os::raw::c_void;
    fn realloc(
        __ptr : *mut ::std::os::raw::c_void, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
}

enum _IO_FILE {
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPoint_ {
    pub xx : f64,
    pub yy : f64,
}

impl Clone for EmbPoint_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbSettings_ {
    pub dstJumpsPerTrim : u32,
    pub home : EmbPoint_,
}

impl Clone for EmbSettings_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbHoop_ {
    pub width : f64,
    pub height : f64,
}

impl Clone for EmbHoop_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbStitch_ {
    pub flags : i32,
    pub xx : f64,
    pub yy : f64,
    pub color : i32,
}

impl Clone for EmbStitch_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbStitchList_ {
    pub stitch : EmbStitch_,
    pub next : *mut EmbStitchList_,
}

impl Clone for EmbStitchList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbColor_ {
    pub r : u8,
    pub g : u8,
    pub b : u8,
}

impl Clone for EmbColor_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbThread_ {
    pub color : EmbColor_,
    pub description : *const u8,
    pub catalogNumber : *const u8,
}

impl Clone for EmbThread_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbThreadList_ {
    pub thread : EmbThread_,
    pub next : *mut EmbThreadList_,
}

impl Clone for EmbThreadList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArc_ {
    pub startX : f64,
    pub startY : f64,
    pub midX : f64,
    pub midY : f64,
    pub endX : f64,
    pub endY : f64,
}

impl Clone for EmbArc_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArcObject_ {
    pub arc : EmbArc_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbArcObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbArcObjectList_ {
    pub arcObj : EmbArcObject_,
    pub next : *mut EmbArcObjectList_,
}

impl Clone for EmbArcObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircle_ {
    pub centerX : f64,
    pub centerY : f64,
    pub radius : f64,
}

impl Clone for EmbCircle_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircleObject_ {
    pub circle : EmbCircle_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbCircleObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbCircleObjectList_ {
    pub circleObj : EmbCircleObject_,
    pub next : *mut EmbCircleObjectList_,
}

impl Clone for EmbCircleObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipse_ {
    pub centerX : f64,
    pub centerY : f64,
    pub radiusX : f64,
    pub radiusY : f64,
}

impl Clone for EmbEllipse_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipseObject_ {
    pub ellipse : EmbEllipse_,
    pub rotation : f64,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbEllipseObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbEllipseObjectList_ {
    pub ellipseObj : EmbEllipseObject_,
    pub next : *mut EmbEllipseObjectList_,
}

impl Clone for EmbEllipseObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbLine_ {
    pub x1 : f64,
    pub y1 : f64,
    pub x2 : f64,
    pub y2 : f64,
}

impl Clone for EmbLine_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbLineObject_ {
    pub line : EmbLine_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbLineObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbLineObjectList_ {
    pub lineObj : EmbLineObject_,
    pub next : *mut EmbLineObjectList_,
}

impl Clone for EmbLineObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointList_ {
    pub point : EmbPoint_,
    pub next : *mut EmbPointList_,
}

impl Clone for EmbPointList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbFlagList_ {
    pub flag : i32,
    pub next : *mut EmbFlagList_,
}

impl Clone for EmbFlagList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPathObject_ {
    pub pointList : *mut EmbPointList_,
    pub flagList : *mut EmbFlagList_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbPathObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPathObjectList_ {
    pub pathObj : *mut EmbPathObject_,
    pub next : *mut EmbPathObjectList_,
}

impl Clone for EmbPathObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointObject_ {
    pub point : EmbPoint_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbPointObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPointObjectList_ {
    pub pointObj : EmbPointObject_,
    pub next : *mut EmbPointObjectList_,
}

impl Clone for EmbPointObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolygonObject_ {
    pub pointList : *mut EmbPointList_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbPolygonObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolygonObjectList_ {
    pub polygonObj : *mut EmbPolygonObject_,
    pub next : *mut EmbPolygonObjectList_,
}

impl Clone for EmbPolygonObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolylineObject_ {
    pub pointList : *mut EmbPointList_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbPolylineObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPolylineObjectList_ {
    pub polylineObj : *mut EmbPolylineObject_,
    pub next : *mut EmbPolylineObjectList_,
}

impl Clone for EmbPolylineObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbRect_ {
    pub top : f64,
    pub left : f64,
    pub bottom : f64,
    pub right : f64,
}

impl Clone for EmbRect_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbRectObject_ {
    pub rect : EmbRect_,
    pub rotation : f64,
    pub radius : f64,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbRectObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbRectObjectList_ {
    pub rectObj : EmbRectObject_,
    pub next : *mut EmbRectObjectList_,
}

impl Clone for EmbRectObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbBezier_ {
    pub startX : f64,
    pub startY : f64,
    pub control1X : f64,
    pub control1Y : f64,
    pub control2X : f64,
    pub control2Y : f64,
    pub endX : f64,
    pub endY : f64,
}

impl Clone for EmbBezier_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbSplineObject_ {
    pub bezier : EmbBezier_,
    pub next : *mut EmbSplineObject_,
    pub lineType : i32,
    pub color : EmbColor_,
}

impl Clone for EmbSplineObject_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbSplineObjectList_ {
    pub splineObj : EmbSplineObject_,
    pub next : *mut EmbSplineObjectList_,
}

impl Clone for EmbSplineObjectList_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbPattern_ {
    pub settings : EmbSettings_,
    pub hoop : EmbHoop_,
    pub stitchList : *mut EmbStitchList_,
    pub threadList : *mut EmbThreadList_,
    pub arcObjList : *mut EmbArcObjectList_,
    pub circleObjList : *mut EmbCircleObjectList_,
    pub ellipseObjList : *mut EmbEllipseObjectList_,
    pub lineObjList : *mut EmbLineObjectList_,
    pub pathObjList : *mut EmbPathObjectList_,
    pub pointObjList : *mut EmbPointObjectList_,
    pub polygonObjList : *mut EmbPolygonObjectList_,
    pub polylineObjList : *mut EmbPolylineObjectList_,
    pub rectObjList : *mut EmbRectObjectList_,
    pub splineObjList : *mut EmbSplineObjectList_,
    pub lastStitch : *mut EmbStitchList_,
    pub lastThread : *mut EmbThreadList_,
    pub lastArcObj : *mut EmbArcObjectList_,
    pub lastCircleObj : *mut EmbCircleObjectList_,
    pub lastEllipseObj : *mut EmbEllipseObjectList_,
    pub lastLineObj : *mut EmbLineObjectList_,
    pub lastPathObj : *mut EmbPathObjectList_,
    pub lastPointObj : *mut EmbPointObjectList_,
    pub lastPolygonObj : *mut EmbPolygonObjectList_,
    pub lastPolylineObj : *mut EmbPolylineObjectList_,
    pub lastRectObj : *mut EmbRectObjectList_,
    pub lastSplineObj : *mut EmbSplineObjectList_,
    pub currentColorIndex : i32,
    pub lastX : f64,
    pub lastY : f64,
}

impl Clone for EmbPattern_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct EmbFile_ {
    pub file : *mut _IO_FILE,
}

impl Clone for EmbFile_ {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum1 {
    CSV_MODE_NULL,
    CSV_MODE_COMMENT,
    CSV_MODE_VARIABLE,
    CSV_MODE_THREAD,
    CSV_MODE_STITCH,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum2 {
    CSV_EXPECT_NULL,
    CSV_EXPECT_QUOTE1,
    CSV_EXPECT_QUOTE2,
    CSV_EXPECT_COMMA,
}

unsafe extern fn csvStrToStitchFlag(mut str : *const u8) -> i32 {
    if str.is_null() {
        embLog_error(
            (*b"format-csv.c csvStrToStitchFlag(), str argument is null\n\0").as_ptr(
            )
        );
        -1i32
    } else if strcmp(str,(*b"STITCH\0").as_ptr()) == 0 {
        0i32
    } else if strcmp(str,(*b"JUMP\0").as_ptr()) == 0 {
        1i32
    } else if strcmp(str,(*b"TRIM\0").as_ptr()) == 0 {
        2i32
    } else if strcmp(str,(*b"COLOR\0").as_ptr()) == 0 {
        4i32
    } else if strcmp(str,(*b"END\0").as_ptr()) == 0 {
        16i32
    } else if strcmp(str,(*b"UNKNOWN\0").as_ptr()) == 0 {
        -1i32
    } else {
        -1i32
    }
}

#[no_mangle]
pub unsafe extern fn readCsv(
    mut pattern : *mut EmbPattern_, mut fileName : *const u8
) -> i32 {
    let mut _currentBlock;
    let mut file : *mut EmbFile_ = 0i32 as (*mut EmbFile_);
    let mut numColorChanges : i32 = 0i32;
    let mut size : i32 = 1024i32;
    let mut pos : i32 = 0i32;
    let mut c : i32 = 0i32;
    let mut cellNum : i32 = 0i32;
    let mut process : i32 = 0i32;
    let mut csvMode : i32 = Enum1::CSV_MODE_NULL as (i32);
    let mut expect : i32 = Enum2::CSV_EXPECT_QUOTE1 as (i32);
    let mut flags : i32 = 0i32;
    let mut xx : f64 = 0.0f64;
    let mut yy : f64 = 0.0f64;
    let mut r : u8 = 0u8;
    let mut g : u8 = 0u8;
    let mut b : u8 = 0u8;
    let mut buff : *mut u8 = 0i32 as (*mut u8);
    if pattern.is_null() {
        embLog_error(
            (*b"format-csv.c readCsv(), pattern argument is null\n\0").as_ptr()
        );
        0i32
    } else if fileName.is_null() {
        embLog_error(
            (*b"format-csv.c readCsv(), fileName argument is null\n\0").as_ptr(
            )
        );
        0i32
    } else {
        buff = malloc(size as (usize)) as (*mut u8);
        (if buff.is_null() {
             embLog_error(
                 (*b"format-csv.c readCsv(), unable to allocate memory for buff\n\0").as_ptr(
                 )
             );
             0i32
         } else {
             file = embFile_open(fileName,(*b"r\0").as_ptr());
             (if file.is_null() {
                  embLog_error(
                      (*b"format-csv.c readCsv(), cannot open %s for reading\n\0").as_ptr(
                      ),
                      fileName
                  );
                  0i32
              } else {
                  pos = 0i32;
                  'loop5: loop {
                      c = embFile_getc(file);
                      if c == b'\n' as (i32) {
                          if expect == Enum2::CSV_EXPECT_COMMA as (i32) {
                              process = 1i32;
                          } else if !(expect == Enum2::CSV_EXPECT_QUOTE1 as (i32)) {
                              _currentBlock = 17;
                              break;
                          }
                      } else if c == b',' as (i32) {
                          if expect == Enum2::CSV_EXPECT_COMMA as (i32) {
                              process = 1i32;
                          }
                      } else if c == b'\"' as (i32) {
                          if expect == Enum2::CSV_EXPECT_QUOTE1 as (i32) {
                              expect = Enum2::CSV_EXPECT_QUOTE2 as (i32);
                          } else if expect == Enum2::CSV_EXPECT_QUOTE2 as (i32) {
                              expect = Enum2::CSV_EXPECT_COMMA as (i32);
                          }
                      }
                      if pos >= size - 1i32 {
                          size = size * 2i32;
                          buff = realloc(
                                     buff as (*mut ::std::os::raw::c_void),
                                     size as (usize)
                                 ) as (*mut u8);
                          if buff.is_null() {
                              _currentBlock = 64;
                              break;
                          }
                      }
                      if process != 0 {
                          *buff.offset(pos as (isize)) = 0u8;
                          pos = 0i32;
                          process = 0i32;
                          cellNum = cellNum + 1;
                          expect = Enum2::CSV_EXPECT_QUOTE1 as (i32);
                          if csvMode == Enum1::CSV_MODE_NULL as (i32) {
                              if strcmp(buff as (*const u8),(*b"#\0").as_ptr()) == 0 {
                                  csvMode = Enum1::CSV_MODE_COMMENT as (i32);
                              } else if strcmp(buff as (*const u8),(*b">\0").as_ptr()) == 0 {
                                  csvMode = Enum1::CSV_MODE_VARIABLE as (i32);
                              } else if strcmp(buff as (*const u8),(*b"$\0").as_ptr()) == 0 {
                                  csvMode = Enum1::CSV_MODE_THREAD as (i32);
                              } else {
                                  if !(strcmp(buff as (*const u8),(*b"*\0").as_ptr()) == 0) {
                                      _currentBlock = 52;
                                      break;
                                  }
                                  csvMode = Enum1::CSV_MODE_STITCH as (i32);
                              }
                          } else if !(csvMode == Enum1::CSV_MODE_COMMENT as (i32)) {
                              if !(csvMode == Enum1::CSV_MODE_VARIABLE as (i32)) {
                                  if csvMode == Enum1::CSV_MODE_THREAD as (i32) {
                                      if !(cellNum == 2i32) {
                                          if cellNum == 3i32 {
                                              r = atoi(buff as (*const u8)) as (u8);
                                          } else if cellNum == 4i32 {
                                              g = atoi(buff as (*const u8)) as (u8);
                                          } else if cellNum == 5i32 {
                                              b = atoi(buff as (*const u8)) as (u8);
                                          } else if !(cellNum == 6i32) {
                                              if !(cellNum == 7i32) {
                                                  _currentBlock = 43;
                                                  break;
                                              }
                                              let mut t : EmbThread_;
                                              t.color.r = r;
                                              t.color.g = g;
                                              t.color.b = b;
                                              t.description = (*b"TODO:DESCRIPTION\0").as_ptr();
                                              t.catalogNumber = (*b"TODO:CATALOG_NUMBER\0").as_ptr(
                                                                );
                                              embPattern_addThread(pattern,t);
                                              csvMode = Enum1::CSV_MODE_NULL as (i32);
                                              cellNum = 0i32;
                                          }
                                      }
                                  } else if csvMode == Enum1::CSV_MODE_STITCH as (i32) {
                                      if cellNum == 2i32 {
                                          flags = csvStrToStitchFlag(buff as (*const u8));
                                          if flags == 4i32 {
                                              numColorChanges = numColorChanges + 1;
                                          }
                                      } else if cellNum == 3i32 {
                                          xx = atof(buff as (*const u8));
                                      } else {
                                          if !(cellNum == 4i32) {
                                              _currentBlock = 32;
                                              break;
                                          }
                                          yy = atof(buff as (*const u8));
                                          embPattern_addStitchAbs(pattern,xx,yy,flags,1i32);
                                          csvMode = Enum1::CSV_MODE_NULL as (i32);
                                          cellNum = 0i32;
                                      }
                                  }
                              }
                          }
                          if c == b'\n' as (i32) {
                              csvMode = Enum1::CSV_MODE_NULL as (i32);
                              cellNum = 0i32;
                          }
                      } else if expect == Enum2::CSV_EXPECT_QUOTE2 as (i32) && (c != b'\"' as (i32)) {
                          *buff.offset(
                               {
                                   let _old = pos;
                                   pos = pos + 1;
                                   _old
                               } as (isize)
                           ) = c as (u8);
                      }
                      if !(c != -1i32) {
                          _currentBlock = 60;
                          break;
                      }
                  }
                  (if _currentBlock == 17 {
                       embLog_error(
                           (*b"format-csv.c readCsv(), premature newline\n\0").as_ptr()
                       );
                       0i32
                   } else if _currentBlock == 32 {
                       0i32
                   } else if _currentBlock == 43 {
                       0i32
                   } else if _currentBlock == 52 {
                       0i32
                   } else if _currentBlock == 60 {
                       embFile_close(file);
                       'loop61: loop {
                           if !(embThreadList_count(
                                    (*pattern).threadList
                                ) < numColorChanges) {
                               break;
                           }
                           embPattern_addThread(pattern,embThread_getRandom());
                       }
                       free(buff as (*mut ::std::os::raw::c_void));
                       buff = 0i32 as (*mut u8);
                       1i32
                   } else {
                       embLog_error(
                           (*b"format-csv.c readCsv(), cannot re-allocate memory for buff\n\0").as_ptr(
                           )
                       );
                       0i32
                   })
              })
         })
    }
}

unsafe extern fn csvStitchFlagToStr(mut flags : i32) -> *mut u8 {
    if flags == 16i32 {
        (*b"END\0").as_ptr() as (*mut u8)
    } else if flags == 4i32 {
        (*b"COLOR\0").as_ptr() as (*mut u8)
    } else if flags == 2i32 {
        (*b"TRIM\0").as_ptr() as (*mut u8)
    } else if flags == 1i32 {
        (*b"JUMP\0").as_ptr() as (*mut u8)
    } else if flags == 0i32 {
        (*b"STITCH\0").as_ptr() as (*mut u8)
    } else {
        (*b"UNKNOWN\0").as_ptr() as (*mut u8)
    }
}

#[no_mangle]
pub unsafe extern fn writeCsv(
    mut pattern : *mut EmbPattern_, mut fileName : *const u8
) -> i32 {
    let mut file : *mut EmbFile_ = 0i32 as (*mut EmbFile_);
    let mut sList
        : *mut EmbStitchList_
        = 0i32 as (*mut EmbStitchList_);
    let mut tList
        : *mut EmbThreadList_
        = 0i32 as (*mut EmbThreadList_);
    let mut boundingRect : EmbRect_;
    let mut i : i32 = 0i32;
    let mut stitchCount : i32 = 0i32;
    let mut threadCount : i32 = 0i32;
    if pattern.is_null() {
        embLog_error(
            (*b"format-csv.c writeCsv(), pattern argument is null\n\0").as_ptr(
            )
        );
        0i32
    } else if fileName.is_null() {
        embLog_error(
            (*b"format-csv.c writeCsv(), fileName argument is null\n\0").as_ptr(
            )
        );
        0i32
    } else {
        sList = (*pattern).stitchList;
        stitchCount = embStitchList_count(sList);
        tList = (*pattern).threadList;
        threadCount = embThreadList_count(tList);
        boundingRect = embPattern_calcBoundingBox(pattern);
        (if stitchCount == 0 {
             embLog_error(
                 (*b"format-csv.c writeCsv(), pattern contains no stitches\n\0").as_ptr(
                 )
             );
             0i32
         } else {
             if (*(*pattern).lastStitch).stitch.flags != 16i32 {
                 embPattern_addStitchRel(
                     pattern,
                     0i32 as (f64),
                     0i32 as (f64),
                     16i32,
                     1i32
                 );
                 stitchCount = stitchCount + 1;
             }
             file = embFile_open(fileName,(*b"w\0").as_ptr());
             (if file.is_null() {
                  embLog_error(
                      (*b"format-csv.c writeCsv(), cannot open %s for writing\n\0").as_ptr(
                      ),
                      fileName
                  );
                  0i32
              } else {
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Embroidermodder 2 CSV Embroidery File\"\n\0").as_ptr()
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"http://embroidermodder.github.io\"\n\0").as_ptr()
                  );
                  embFile_printf(file,(*b"\n\0").as_ptr());
                  embFile_printf(file,(*b"\"#\",\"General Notes:\"\n\0").as_ptr());
                  embFile_printf(
                      file,
                      (*b"\"#\",\"This file can be read by Excel or LibreOffice as CSV (Comma Separated Value) or with a text editor.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Lines beginning with # are comments.\"\n\0").as_ptr()
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Lines beginning with > are variables: [VAR_NAME], [VAR_VALUE]\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Lines beginning with $ are threads: [THREAD_NUMBER], [RED], [GREEN], [BLUE], [DESCRIPTION], [CATALOG_NUMBER]\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Lines beginning with * are stitch entries: [STITCH_TYPE], [X], [Y]\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(file,(*b"\n\0").as_ptr());
                  embFile_printf(
                      file,
                      (*b"\"#\",\"Stitch Entry Notes:\"\n\0").as_ptr()
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"STITCH instructs the machine to move to the position [X][Y] and then make a stitch.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"JUMP instructs the machine to move to the position [X][Y] without making a stitch.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"TRIM instructs the machine to cut the thread before moving to the position [X][Y] without making a stitch.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"COLOR instructs the machine to stop temporarily so that the user can change to a different color thread before resuming.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"END instructs the machine that the design is completed and there are no further instructions.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"UNKNOWN encompasses instructions that may not be supported currently.\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(
                      file,
                      (*b"\"#\",\"[X] and [Y] are absolute coordinates in millimeters (mm).\"\n\0").as_ptr(
                      )
                  );
                  embFile_printf(file,(*b"\n\0").as_ptr());
                  embFile_printf(
                      file,
                      (*b"\"#\",\"[VAR_NAME]\",\"[VAR_VALUE]\"\n\0").as_ptr()
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"STITCH_COUNT:\",\"%u\"\n\0").as_ptr(),
                      stitchCount as (u32)
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"THREAD_COUNT:\",\"%u\"\n\0").as_ptr(),
                      threadCount as (u32)
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_LEFT:\",\"%f\"\n\0").as_ptr(),
                      boundingRect.left
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_TOP:\",\"%f\"\n\0").as_ptr(),
                      boundingRect.top
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_RIGHT:\",\"%f\"\n\0").as_ptr(),
                      boundingRect.right
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_BOTTOM:\",\"%f\"\n\0").as_ptr(),
                      boundingRect.bottom
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_WIDTH:\",\"%f\"\n\0").as_ptr(),
                      embRect_width(boundingRect)
                  );
                  embFile_printf(
                      file,
                      (*b"\">\",\"EXTENTS_HEIGHT:\",\"%f\"\n\0").as_ptr(),
                      embRect_height(boundingRect)
                  );
                  embFile_printf(file,(*b"\n\0").as_ptr());
                  embFile_printf(
                      file,
                      (*b"\"#\",\"[THREAD_NUMBER]\",\"[RED]\",\"[GREEN]\",\"[BLUE]\",\"[DESCRIPTION]\",\"[CATALOG_NUMBER]\"\n\0").as_ptr(
                      )
                  );
                  i = 1i32;
                  'loop7: loop {
                      if tList.is_null() {
                          break;
                      }
                      embFile_printf(
                          file,
                          (*b"\"$\",\"%d\",\"%d\",\"%d\",\"%d\",\"%s\",\"%s\"\n\0").as_ptr(),
                          i,
                          (*tList).thread.color.r as (i32),
                          (*tList).thread.color.g as (i32),
                          (*tList).thread.color.b as (i32),
                          (*tList).thread.description,
                          (*tList).thread.catalogNumber
                      );
                      i = i + 1;
                      tList = (*tList).next as (*mut EmbThreadList_);
                  }
                  embFile_printf(file,(*b"\n\0").as_ptr());
                  embFile_printf(
                      file,
                      (*b"\"#\",\"[STITCH_TYPE]\",\"[X]\",\"[Y]\"\n\0").as_ptr()
                  );
                  'loop9: loop {
                      if sList.is_null() {
                          break;
                      }
                      let mut s : EmbStitch_ = (*sList).stitch;
                      embFile_printf(
                          file,
                          (*b"\"*\",\"%s\",\"%f\",\"%f\"\n\0").as_ptr(),
                          csvStitchFlagToStr(s.flags),
                          s.xx,
                          s.yy
                      );
                      sList = (*sList).next as (*mut EmbStitchList_);
                  }
                  embFile_close(file);
                  1i32
              })
         })
    }
}
