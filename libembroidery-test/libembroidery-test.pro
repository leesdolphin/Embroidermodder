TEMPLATE = app
CONFIG -= debug_and_release qt
CONFIG += console debug
#CONFIG += console release
CONFIG -= app_bundle

TARGET = libembroidery-test

OBJECTS_DIR = .obj
MOC_DIR = .moc

INCLUDEPATH += \
../libembroidery \
../libcgeometry \
$$PWD \

SOURCES += \
../libembroidery-test/libembroidery-test-main.c \
../libembroidery-test/libembroidery-test-hus-compress.c \

HEADERS += \
../libembroidery-test/libembroidery-test-hus-compress.h \


include( ../libembroidery/libembroidery.pri )

