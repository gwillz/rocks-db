#-------------------------------------------------
#
# Project created by QtCreator 2020-01-02T17:12:47
#
#-------------------------------------------------

QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = RocksDB
TEMPLATE = app

QMAKE_CXXFLAGS += -std=c++0x

SOURCES += main.cpp\
        mainwindow.cpp \
    cheekytextedit.cpp

HEADERS  += mainwindow.h \
    cheekytextedit.h

FORMS    += mainwindow.ui

OTHER_FILES += \
    install.nsi

LIBROCKS_DIR = $$PWD/../lib/target
INCLUDEPATH += $$LIBROCKS_DIR

win32 {
    LIBROCKS_LIB = librocks.dll
}
else {
    LIBROCKS_LIB = librocks.so
}

CONFIG(debug, debug|release) {
    LIBS += $$LIBROCKS_DIR/debug/$$LIBROCKS_LIB
}
else {
    LIBS += $$LIBROCKS_DIR/release/$$LIBROCKS_LIB
}

