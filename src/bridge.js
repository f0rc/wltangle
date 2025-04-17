var windowManager = {
    calculateGeometry: function (position, width, height) {
        if (position === 0) {
            return [0, 0, width / 2, height];
        } else if (position === 1) {
            return [width / 2, 0, width / 2, height];
        }
        return [0, 0, width, height];
    },

    positionWindow: function (windowId, position) {
        var width = window.innerWidth;
        console.log("Window " + windowId + " positioned to " + position);
    }
};