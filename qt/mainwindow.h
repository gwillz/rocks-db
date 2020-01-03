#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include "librocks.h"

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();

public slots:
    void load();
    void reload();
    void quit();

private:
    void initDB(QString fileName);

    Ui::MainWindow *ui;
    RockDB *db;
    QString currentFileName;
};

#endif // MAINWINDOW_H
