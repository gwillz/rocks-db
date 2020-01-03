#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QFileDialog>
#include <QClipboard>

QStringList loadFile(QString fileName);

MainWindow::MainWindow(QWidget *parent) :
        QMainWindow(parent),
        ui(new Ui::MainWindow) {
    
    ui->setupUi(this);
    statusLabel = new QLabel(this);
    statusBar()->addPermanentWidget(statusLabel);
    
    // Initial load file.
    initDB("description-database.txt");
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::initDB(QString fileName) {
    if (!fileName.isEmpty()) {
        
        // Dealloc if necessary.
        if (db != nullptr) {
            free(db);
        }
        
        // Convert file name into friendly (short) and c-string.
        QString friendlyFileName = QFile(fileName).fileName();
        QByteArray ba = fileName.toUtf8();
        const char* c_fileName = ba.constData();
        
        // Replace the DB (after dealloc above).
        db = rocks_load(c_fileName);
        
        if (db != nullptr) {
            // This filename is now the active DB.
            currentFileName = fileName;
            
            Fragments fragments = rocks_fragments(db);
            
            statusBar()->showMessage("Successfully loaded DB.", 5000);
            statusLabel->setText(QString("%1 [%2 symbols]")
                                 .arg(friendlyFileName)
                                 .arg(fragments.size));
        }
        else {
            statusBar()->showMessage(QString("Failed to load %1.")
                                     .arg(friendlyFileName), 5000);
            statusLabel->setText("No DB loaded.");
        }
    }
    else {
        statusBar()->clearMessage();
        statusLabel->setText("No DB loaded.");
    }
}

void MainWindow::load() {
    // Open an new DB file.
    
    QString fileName = QFileDialog::getOpenFileName(this);
    
    if (!fileName.isEmpty()) {
        initDB(fileName);
    }
}

void MainWindow::reload() {
    // Reload the current file.
    
    if (!currentFileName.isEmpty()) {
        initDB(currentFileName);
    }
    else {
        load();
    }
}

void MainWindow::convertSymbols() {
    if (db != nullptr) {
        // Create c-string descriptors.
        QByteArray ba = ui->descriptorEdit->document()->toPlainText().toUtf8();
        const char* c_phrases = ba.constData();
        
        // Convert.
        const char* c_symbols = rocks_convert(db, c_phrases);
        
        // Update symbols.
        ui->symbolsEdit->document()->setPlainText(QString::fromUtf8(c_symbols));
    }
}

void MainWindow::copyToClipboard() {
    // Copy the current symbols to the clipboard.
    
    QString symbols = ui->symbolsEdit->document()->toPlainText();
    qApp->clipboard()->setText(symbols);
}

void MainWindow::quit() {
    qApp->exit();
}
