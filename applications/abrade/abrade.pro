QT += quick
QT += dbus

CONFIG += c++11
CONFIG -= app_bundle

# The following define makes your compiler emit warnings if you use
# any feature of Qt which as been marked deprecated (the exact warnings
# depend on your compiler). Please consult the documentation of the
# deprecated API in order to know how to port your code away from it.
DEFINES += QT_DEPRECATED_WARNINGS

# You can also make your code fail to compile if you use deprecated APIs.
# In order to do so, uncomment the following line.
# You can also select to disable deprecated APIs only up to a certain version of Qt.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += main.cpp \
    controller.cpp \
    eventdevice.cpp \
    view.cpp \
    keyboard.cpp \
    evdev.cpp \
    fb2png.cpp

TARGET=abrade

target.path = /opt/bin
INSTALLS += target

DISTFILES +=

RESOURCES += \
    resources.qrc

HEADERS += \
    controller.h \
    eventdevice.h \
    view.h \
    keyboard.h \
    evdev.h \
    events.h \
    events.h \
    events.h \
    events.h \
    fb2png.h

linux-oe-g++ {
    LIBS += -lqsgepaper
    LIBS += -lpng16
}
