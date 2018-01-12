/*! @file emb-file.h */
#ifndef EMB_FILE_H
#define EMB_FILE_H

#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct EmbFile_
{
    FILE *file;
} EmbFile;

extern EmbFile *embFile_open(const char *fileName, const char *mode);
extern int embFile_close(EmbFile *stream);
extern int embFile_eof(EmbFile *stream);
extern int embFile_getc(EmbFile *stream);
extern size_t embFile_read(void *ptr, size_t size, size_t nmemb, EmbFile *stream);
extern size_t embFile_write(const void *ptr, size_t size, size_t nmemb, EmbFile *stream);
extern int embFile_seek(EmbFile *stream, long offset, int origin);
extern long embFile_tell(EmbFile *stream);
extern EmbFile *embFile_tmpfile(void);
extern int embFile_putc(int ch, EmbFile *stream);

extern int embFile_printf(EmbFile *stream, const char *format, ...);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* EMB_FILE_H */
