#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QDebug>

QStringList loadFile(QString fileName);

MainWindow::MainWindow(QWidget *parent) :
        QMainWindow(parent),
        ui(new Ui::MainWindow) {
    ui->setupUi(this);

    initDB("description-database.txt");
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::initDB(QString fileName) {
    if (!fileName.isEmpty()) {
//        if (db != nullptr) {
//            free(db);
//        }

        QByteArray ba = fileName.toUtf8();
        const char* c_fileName = ba.constData();

        currentFileName = fileName;
        db = rocks_load(c_fileName);

        if (db != nullptr) {
            currentFileName = fileName;
            statusBar()->setStatusTip(fileName);
        }
        // @todo failed to load dialog.
    }
    // @todo file name empty dialog.
}

void MainWindow::load() {
    QString fileName = QFileDialog::getOpenFileName(this);
    initDB(fileName);
}

void MainWindow::reload() {
    if (!currentFileName.isEmpty()) {
        initDB(currentFileName);
    }
    else {
        load();
    }
}

void MainWindow::quit() {
    qApp->exit();
}
