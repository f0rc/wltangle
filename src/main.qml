// main.qml
import QtQuick 2.0
import org.kde.kwin 2.0
import org.kde.plasma.core 2.0 as PlasmaCore
import org.kde.plasma.components 2.0 as PlasmaComponents
import org.kde.plasma.extras 2.0 as PlasmaExtras
import "rust_bridge.js" as RustBridge

Item {
    id: root
    
    Component.onCompleted: {
        registerShortcut("MoveWindowToLeft", "Move Window to Left", "Meta+Left", 
                         function() { positionWindow(0); });
        registerShortcut("MoveWindowToRight", "Move Window to Right", "Meta+Right", 
                         function() { positionWindow(1); });
    }
    
    function positionWindow(position) {
        var client = workspace.activeClient;
        if (!client) return;
        
        var screen = client.screen;
        var screenGeometry = workspace.clientArea(KWin.ScreenArea, screen, workspace.currentDesktop);
        
        var geometry = RustBridge.windowManager.calculateGeometry(
            position, 
            screenGeometry.width, 
            screenGeometry.height
        );
        client.geometry = Qt.rect(
            screenGeometry.x + geometry[0], 
            screenGeometry.y + geometry[1], 
            geometry[2], 
            geometry[3]
        );
        RustBridge.windowManager.positionWindow(client.internalId, position);
    }
}