#include <QString>
#include <QStandardPaths>

#include "path.h"

namespace cxxbridge1 {
    QString writableLocation() {
        return QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
    }
}
