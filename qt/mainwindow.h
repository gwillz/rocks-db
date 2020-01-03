#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

class QLabel;
class RockDB;

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow {
    Q_OBJECT
    
public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();
    
public slots:
    // Action events.
    void load();
    void reload();
    void about();
    void quit();
    
    // Button events.
    void convertSymbols();
    void copyToClipboard();
    
private:
    // Internal shorthand for managing the DB.
    void initDB(QString fileName);
    
    // UI elements.
    Ui::MainWindow *ui;
    QLabel *statusLabel;
    
    // Database.
    RockDB *db = 0;
    QString currentFileName;
};

#endif // MAINWINDOW_H
