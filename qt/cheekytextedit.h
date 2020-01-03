#ifndef CHEEKYTEXTEDIT_H
#define CHEEKYTEXTEDIT_H

#include <QPlainTextEdit>

class QKeyEvent;

class CheekyTextEdit : public QPlainTextEdit
{
    Q_OBJECT

public:
    explicit CheekyTextEdit(QWidget *parent = 0);
    void keyPressEvent(QKeyEvent *event);

signals:
    void enter();
};

#endif // CHEEKYTEXTEDIT_H
