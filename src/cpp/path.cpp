#include <QString>
#include <QStandardPaths>

#include "path.h"

// try as i might, this fuckass static enum of the class cannot be bridged to rust.
// so this cpp file stands here to bridge it
QString writableLocation() {
    return QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
}
