#-------------------------------------------------
#
# Project created by QtCreator 2020-01-02T17:12:47
#
#-------------------------------------------------

QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = RocksDB
TEMPLATE = app


SOURCES += main.cpp\
        mainwindow.cpp

HEADERS  += mainwindow.h

FORMS    += mainwindow.ui

win32 {
    LIBROCKS = $$PWD/librocks

    CONFIG(debug, debug|release) {
        LIBROCKS = $$PWD/librocks/debug
    }
    else {
        LIBROCKS = $$PWD/librocks/release
    }

    INCLUDEPATH += $$LIBROCKS
    LIBS += $$LIBROCKS/librocks.dll.lib
#    LIBS += $$LIBROCKS/librocks.dll
}

