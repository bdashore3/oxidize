#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlEngine>
#include <QtPlugin>
#include <QtQuick>
#include <QQuickView>
#include <QList>
#include "view/mainview.h"
#include "widget/keyboardhandler.h"

#ifdef __arm__
Q_IMPORT_PLUGIN(QsgEpaperPlugin)
#endif

const char *qt_version = qVersion();

int main(int argc, char *argv[]) {
    if (strcmp(qt_version, QT_VERSION_STR) != 0){
        qDebug() << "Version mismatch, Runtime: " << qt_version << ", Build: " << QT_VERSION_STR;
    }
#ifdef __arm__
    // Setup epaper
    qputenv("QMLSCENE_DEVICE", "epaper");
    qputenv("QT_QPA_PLATFORM", "epaper:enable_fonts");
    qputenv("QT_QPA_EVDEV_TOUCHSCREEN_PARAMETERS", "rotate=180");
    qputenv("QT_QPA_GENERIC_PLUGINS", "evdevtablet");
//    qputenv("QT_DEBUG_BACKINGSTORE", "1");
#endif
    qmlRegisterType<KeyboardHandler>("KeyboardHandler", 1, 0, "KeyboardHandler");
    QGuiApplication app(argc, argv);
    // Load font
    int id = QFontDatabase::addApplicationFont(":/src/font/Roboto-NotoEmoji-Regular.ttf");
    QString family = QFontDatabase::applicationFontFamilies(id).at(0);
    QFont font = QFont(family, 10, 1);
    font.setFamily(font.defaultFamily());
    app.setFont(font);
    // Load QML
    QQmlApplicationEngine engine;
    MainView view(&engine);
    QQmlContext* context = view.rootContext();
    context->setContextProperty("screenGeometry", app.primaryScreen()->geometry());
    context->setContextProperty("cwd", "/");
    view.setSource(QUrl(QStringLiteral("qrc:/src/view/main.qml")));
    QQuickItem* root = view.rootObject();
    if(root->children().isEmpty()){
        qDebug() << "Nothing to display";
        return 1;
    }
    // Allow quitting
    QObject::connect((QObject*)view.engine(), SIGNAL(quit()), &app, SLOT(quit()));
    // Start
    view.show();
    qDebug() << "View shown";
    KeyboardHandler* keyboard = root->findChild<KeyboardHandler*>("keyboard");
    if(!keyboard){
        qDebug() << "No keyboard handler";
        return 1;
    }
    keyboard->view = &view;
    return app.exec();
}
