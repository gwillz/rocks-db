#include "cheekytextedit.h"

CheekyTextEdit::CheekyTextEdit(QWidget *parent) :
        QPlainTextEdit(parent) {}

void CheekyTextEdit::keyPressEvent(QKeyEvent *event) {
    
    // Emit an enter() signal on any modifier + Enter.
    // E.g. Ctrl+Enter, Alt+Enter, etc.
    
    if (event->key() == Qt::Key_Return &&
        event->modifiers() != Qt::NoModifier) {
        emit enter();
        return;
    }
    
    // Otherwise.
    QPlainTextEdit::keyPressEvent(event);
}
