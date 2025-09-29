#include <QString>
#include <QStandardPaths>

#include "path.h"

QString writableLocation() {
    return QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
}
