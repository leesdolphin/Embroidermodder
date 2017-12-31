#include "format-svg.h"
#include "emb-file.h"
#include "emb-logging.h"
#include "helpers-misc.h"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>

/* Includes for NanoSVG */
#include <stdio.h>
#include <string.h>
#include <math.h>
#define NANOSVG_ALL_COLOR_KEYWORDS    /* Include full list of color keywords. */
#define NANOSVG_IMPLEMENTATION        /* Expands implementation */
#include "libraries/nanosvg.h"
/* End includes. */


#define EMB_SVG_DPI 96.

#define EMB_SVG_AS_MM(val) (val)
#define EMB_SVG_OUTPUT_MM(val, tmp) emb_optOut1d(val, tmp)

EmbColor parseSvgPaintColor(NSVGpaint stroke)
{
    /* If no color found; default to black. */
    unsigned int color = 0;
    printf("Stroke type: %d %08x\n", stroke.type, stroke.color);
    if ((stroke.type == NSVG_PAINT_LINEAR_GRADIENT ||
         stroke.type == NSVG_PAINT_RADIAL_GRADIENT) &&
        stroke.gradient->nstops > 0)
    {
        /* No support for Gradients. Pick the first stop color */
        color = stroke.gradient->stops[0].color;
    }
    else if (stroke.type == NSVG_PAINT_COLOR)
    {
        color = stroke.color;
    }
    return embColor_make(color & 0xff, (color >> 8) & 0xff, (color >> 16) & 0xff);
}


/*! Reads a file with the given \a fileName and loads the data into \a pattern.
 *  Returns \c true if successful, otherwise returns \c false. */
int readSvg(EmbPattern* pattern, const char* fileName)
{
    EmbFile* file = 0;
    size_t size = 0;
    char* buff = 0;
    NSVGimage* image = NULL;
    NSVGshape* shape = NULL;
    NSVGpath* path = NULL;
    int i;
    double* p = NULL;
    int colorChanges = 0;
    int generateColorStitch = 0;
    EmbColor oldColor, color;
    EmbThread t;

    if(!pattern) { embLog_error("format-svg.c readSvg(), pattern argument is null\n"); return 0; }
    if(!fileName) { embLog_error("format-svg.c readSvg(), fileName argument is null\n"); return 0; }

    /* Pre-flip incase of multiple reads on the same pattern */
    embPattern_flipVertical(pattern);

    file = embFile_open(fileName, "rb");
    if (!file)  { embLog_error("format-svg.c readSvg(), failed to open file\n"); return 0; }

    embFile_seek(file, 0, SEEK_END);
    size = embFile_tell(file);
    embFile_seek(file, 0, SEEK_SET);
    buff = (char *)malloc(size + 1);
    if (buff == NULL) { embLog_error("format-svg.c readSvg(), unable to allocate buffer.\n"); return 0; }
    if (embFile_read(buff, 1, size, file) != size) { embLog_error("format-svg.c readSvg(), reading into buffer failed.\n"); return 0; }

    buff[size] = '\0'; /* Must be null terminated. */
    embFile_close(file);

    /* 96 DPI, so 96px=1in and 37.8px=1cm */
    image = nsvgParse(buff, "mm", EMB_SVG_DPI);
    free(buff);

    printf("size: %f x %f\n", image->width, image->height);
    for (shape = image->shapes; shape != NULL; shape = shape->next)
    {
        color = parseSvgPaintColor(shape->stroke);
        if (colorChanges == 0 || color.r != oldColor.r || color.g != oldColor.g || color.b != oldColor.b)
        {
            colorChanges++;
            t.color = color;
            printf("Changing color: %x %x %x\n", color.r, color.g, color.b);
            t.description = "TODO:DESCRIPTION";
            t.catalogNumber = "TODO:CATALOG_NUMBER";
            embPattern_addThread(pattern, t);
            if (colorChanges > 1) {
                printf("Changing color stich: %x %x %x\n", color.r, color.g, color.b);
                generateColorStitch = 1;
            }
            oldColor = color;
        }
        printf("Color: %x %x %x\n", color.r, color.g, color.b);
        for (path = shape->paths; path != NULL; path = path->next)
        {
            for (i = 0; i < path->npts-1; i += 3)
            {
                /*
                 * Every set of 4 points is a Cubic Bezier.
                 * Ignoring that and assuming lines from start to finish
                 */
                p = &path->pts[i * 2];
                if (i == 0) {
                    if (generateColorStitch) {
                        printf("Generating color stich: %x %x %x\n", color.r, color.g, color.b);
                        embPattern_addStitchAbs(pattern, p[0], p[1], STOP, 1);
                        generateColorStitch = 0;
                    }
                    printf("Generating jump stich\n");
                    embPattern_addStitchAbs(pattern, p[0], p[1], JUMP, 1);
                    embPattern_addStitchAbs(pattern, p[0], p[1], NORMAL, 1);
                }
                embPattern_addStitchAbs(pattern, p[6], p[7], NORMAL, 1);
/*
                if ((p[0] == p[6]) && (p[1] == p[7]))
                {
                    embPattern_addPointObjectAbs(pattern, p[0], p[1]);
                }
                else
                {
                    embPattern_addLineObjectAbs(pattern, p[0], p[1], p[6], p[7]);
                    pattern->lastLineObj->lineObj.lineType = LINETO;
                    pattern->lastLineObj->lineObj.color = color;
                    printf("(%03.2f, %03.2f), (%03.2f, %03.2f), (%03.2f, %03.2f), (%03.2f, %03.2f)\n", p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);
                }
  */          }
        }
    }
    nsvgDelete(image);

    /* Flip the pattern since SVG Y+ is down and libembroidery Y+ is up. */
    embPattern_flipVertical(pattern);

    return 1; /*TODO: finish readSvg */
}

/* kate: bom off; indent-mode cstyle; indent-width 4; replace-trailing-space-save on; */

/*! Writes the data from \a pattern to a file with the given \a fileName.
 *  Returns \c true if successful, otherwise returns \c false. */
int writeSvg(EmbPattern *pattern, const char *fileName)
{
    EmbFile *file = 0;
    EmbRect boundingRect;
    EmbStitchList *stList;
    EmbCircleObjectList *cObjList = 0;
    EmbCircle circle;
    EmbEllipseObjectList *eObjList = 0;
    EmbEllipse ellipse;
    EmbLineObjectList *liObjList = 0;
    EmbLine line;
    EmbPointObjectList *poObjList = 0;
    EmbPoint point;
    EmbPolygonObjectList *pogObjList = 0;
    EmbPointList *pogPointList = 0;
    EmbPolylineObjectList *polObjList = 0;
    EmbPointList *polPointList = 0;
    EmbRectObjectList *rObjList = 0;
    EmbRect rect;
    EmbColor color;

    char tmpX[32];
    char tmpY[32];

    if (!pattern)
    {
        embLog_error("format-svg.c writeSvg(), pattern argument is null\n");
        return 0;
    }
    if (!fileName)
    {
        embLog_error("format-svg.c writeSvg(), fileName argument is null\n");
        return 0;
    }

    file = embFile_open(fileName, "w");
    if (!file)
    {
        embLog_error("format-svg.c writeSvg(), cannot open %s for writing\n", fileName);
        return 0;
    }

    /* Pre-flip the pattern since SVG Y+ is down and libembroidery Y+ is up. */
    embPattern_flipVertical(pattern);

    boundingRect = embPattern_calcBoundingBox(pattern);
    embFile_printf(file, "<?xml version=\"1.0\"?>\n");
    embFile_printf(file, "<!-- Embroidermodder 2 SVG Embroidery File -->\n");
    embFile_printf(file, "<!-- http://embroidermodder.github.io -->\n");
    embFile_printf(file, "<svg ");

    /* TODO: See the SVG Tiny Version 1.2 Specification Section 7.14.
     *       Until all of the formats and API is stable, the width, height and viewBox attributes need to be left unspecified.
     *       If the attribute values are incorrect, some applications wont open it at all.
     */
    embFile_printf(file, "viewBox=\"%s %s",
                   EMB_SVG_OUTPUT_MM(boundingRect.left, tmpX),
                   EMB_SVG_OUTPUT_MM(boundingRect.top, tmpY));
    embFile_printf(file, " %s %s\"",
                   EMB_SVG_OUTPUT_MM(embRect_width(boundingRect), tmpX),
                   EMB_SVG_OUTPUT_MM(embRect_height(boundingRect), tmpY));
    embFile_printf(file, " width=\"%smm\" height=\"%smm\" ",
                   emb_optOut(embRect_width(boundingRect), tmpX),
                   emb_optOut(embRect_height(boundingRect), tmpY));

    embFile_printf(file, "xmlns=\"http://www.w3.org/2000/svg\" version=\"1.2\" baseProfile=\"tiny\">");

    /*TODO: Low Priority Optimization:
    *      Using %g in embFile_printf just doesn't work good enough at trimming trailing zeroes.
    *      It's precision refers to significant digits, not decimal places (which is what we want).
    *      We need to roll our own function for trimming trailing zeroes to keep
    *      the precision as high as possible if needed, but help reduce file size also. */

    /*TODO: Low Priority Optimization:
    *      Make sure that the line length that is output doesn't exceed 1000 characters. */

    /*TODO: Low Priority: Indent output properly. */

    /* write circles */
    cObjList = pattern->circleObjList;
    while (cObjList)
    {
        circle = cObjList->circleObj.circle;
        color = cObjList->circleObj.color;
        /* TODO: use proper thread width for stoke-width rather than just 0.2 */
        embFile_printf(file, "\n<circle stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" cx=\"%s\" cy=\"%s\"",
                       color.r,
                       color.g,
                       color.b,
                       EMB_SVG_OUTPUT_MM(circle.centerX, tmpX),
                       EMB_SVG_OUTPUT_MM(circle.centerY, tmpY));
        embFile_printf(file, " r=\"%s\" />",
                       EMB_SVG_OUTPUT_MM(circle.radius, tmpX));
        cObjList = cObjList->next;
    }

    /* write ellipses */
    eObjList = pattern->ellipseObjList;
    while (eObjList)
    {
        ellipse = eObjList->ellipseObj.ellipse;
        color = eObjList->ellipseObj.color;
        /* TODO: use proper thread width for stoke-width rather than just 0.2 */
        embFile_printf(file, "\n<ellipse stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" cx=\"%s\" cy=\"%s\"",
                       color.r,
                       color.g,
                       color.b,
                       EMB_SVG_OUTPUT_MM(ellipse.centerX, tmpX),
                       EMB_SVG_OUTPUT_MM(ellipse.centerY, tmpY));
        embFile_printf(file, " rx=\"%s\" ry=\"%s\" />",
                       EMB_SVG_OUTPUT_MM(ellipse.radiusX, tmpX),
                       EMB_SVG_OUTPUT_MM(ellipse.radiusY, tmpY));
        eObjList = eObjList->next;
    }

    /* write lines */
    liObjList = pattern->lineObjList;
    while (liObjList)
    {
        line = liObjList->lineObj.line;
        color = liObjList->lineObj.color;
        /* TODO: use proper thread width for stoke-width rather than just 0.2 */
        embFile_printf(file, "\n<line stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" x1=\"%s\" y1=\"%s\"",
                       color.r,
                       color.g,
                       color.b,
                       EMB_SVG_OUTPUT_MM(line.x1, tmpX),
                       EMB_SVG_OUTPUT_MM(line.y1, tmpY));
        embFile_printf(file, " x2=\"%s\" y2=\"%s\" />",
                       EMB_SVG_OUTPUT_MM(line.x2, tmpX),
                       EMB_SVG_OUTPUT_MM(line.y2, tmpY));
        liObjList = liObjList->next;
    }

    /* write points */
    poObjList = pattern->pointObjList;
    while (poObjList)
    {
        point = poObjList->pointObj.point;
        color = poObjList->pointObj.color;
        /* See SVG Tiny 1.2 Spec:
        * Section 9.5 The 'line' element
        * Section C.6 'path' element implementation notes */
        /* TODO: use proper thread width for stoke-width rather than just 0.2 */
        embFile_printf(file, "\n<line stroke-linecap=\"round\" stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" x1=\"%s\" y1=\"%s\"",
                       color.r,
                       color.g,
                       color.b,
                       EMB_SVG_OUTPUT_MM(point.xx, tmpX),
                       EMB_SVG_OUTPUT_MM(point.yy, tmpY));
        embFile_printf(file,  " x2=\"%s\" y2=\"%s\" />",
                       EMB_SVG_OUTPUT_MM(point.xx, tmpX),
                       EMB_SVG_OUTPUT_MM(point.yy, tmpY));
        poObjList = poObjList->next;
    }

    /* write polygons */
    pogObjList = pattern->polygonObjList;
    while (pogObjList)
    {
        pogPointList = pogObjList->polygonObj->pointList;
        if (pogPointList)
        {
            color = pogObjList->polygonObj->color;
            /* TODO: use proper thread width for stoke-width rather than just 0.2 */
            embFile_printf(file, "\n<polygon stroke-linejoin=\"round\" stroke-linecap=\"round\" stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" points=\"%s,%s",
                           color.r,
                           color.g,
                           color.b,
                           EMB_SVG_OUTPUT_MM(pogPointList->point.xx, tmpX),
                           EMB_SVG_OUTPUT_MM(pogPointList->point.yy, tmpY));
            pogPointList = pogPointList->next;
            while (pogPointList)
            {
                embFile_printf(file, " %s,%s", emb_optOut(pogPointList->point.xx, tmpX), emb_optOut(pogPointList->point.yy, tmpY));
                pogPointList = pogPointList->next;
            }
            embFile_printf(file, "\" />");
        }
        pogObjList = pogObjList->next;
    }

    /* write polylines */
    polObjList = pattern->polylineObjList;
    while (polObjList)
    {
        polPointList = polObjList->polylineObj->pointList;
        if (polPointList)
        {
            color = polObjList->polylineObj->color;
            /* TODO: use proper thread width for stoke-width rather than just 0.2 */
            embFile_printf(file, "\n<polyline stroke-linejoin=\"round\" stroke-linecap=\"round\" stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" points=\"%s,%s",
                           color.r,
                           color.g,
                           color.b,
                           EMB_SVG_OUTPUT_MM(polPointList->point.xx, tmpX),
                           EMB_SVG_OUTPUT_MM(polPointList->point.yy, tmpY));
            polPointList = polPointList->next;
            while (polPointList)
            {
                embFile_printf(file, " %s,%s", EMB_SVG_OUTPUT_MM(polPointList->point.xx, tmpX), EMB_SVG_OUTPUT_MM(polPointList->point.yy, tmpY));
                polPointList = polPointList->next;
            }
            embFile_printf(file, "\" />");
        }
        polObjList = polObjList->next;
    }

    /* write rects */
    rObjList = pattern->rectObjList;
    while (rObjList)
    {
        rect = rObjList->rectObj.rect;
        color = rObjList->rectObj.color;
        /* TODO: use proper thread width for stoke-width rather than just 0.2 */
        embFile_printf(file, "\n<rect stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" x=\"%f\" y=\"%f\"",
                       color.r,
                       color.g,
                       color.b,
                       EMB_SVG_OUTPUT_MM(embRect_x(rect), tmpX),
                       EMB_SVG_OUTPUT_MM(embRect_y(rect), tmpY));
        embFile_printf(file, " width=\"%f\" height=\"%f\" />",
                       EMB_SVG_OUTPUT_MM(embRect_width(rect), tmpX),
                       EMB_SVG_OUTPUT_MM(embRect_height(rect), tmpY));
        rObjList = rObjList->next;
    }

    stList = pattern->stitchList;
    if (stList)
    {
        /*TODO: #ifdef SVG_DEBUG for Josh which outputs JUMPS/TRIMS instead of chopping them out */
        char isNormal = 0;
        int closed = -1;
        while (stList)
        {
            if (stList->stitch.flags == NORMAL && !isNormal)
            {
                isNormal = 1;
                color = embThreadList_getAt(pattern->threadList, stList->stitch.color).color;
                /* TODO: use proper thread width for stoke-width rather than just 0.2 */
                embFile_printf(file, "\n<polyline stroke-linejoin=\"round\" stroke-linecap=\"round\" stroke-width=\"0.2px\" stroke=\"#%02x%02x%02x\" fill=\"none\" points=\"%s,%s",
                               color.r,
                               color.g,
                               color.b,
                               EMB_SVG_OUTPUT_MM(stList->stitch.xx, tmpX),
                               EMB_SVG_OUTPUT_MM(stList->stitch.yy, tmpY));
                closed = 0;
            }
            else if (stList->stitch.flags == NORMAL && isNormal)
            {
                embFile_printf(file, " %s,%s", EMB_SVG_OUTPUT_MM(stList->stitch.xx, tmpX), EMB_SVG_OUTPUT_MM(stList->stitch.yy, tmpY));
            }
            else if (stList->stitch.flags != NORMAL && isNormal)
            {
                isNormal = 0;
                closed = !0;
                embFile_printf(file, "\" />");
            }

            stList = stList->next;
        }
        if (!closed) embFile_printf(file, "\" />");
    }
    embFile_printf(file, "\n</svg>\n");
    embFile_close(file);

    /* Reset the pattern so future writes(regardless of format) are not flipped */
    embPattern_flipVertical(pattern);

    return 1;
}

/* kate: bom off; indent-mode cstyle; indent-width 4; replace-trailing-space-save on; */
