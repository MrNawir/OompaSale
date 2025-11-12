import QtQuick
import QtQuick.Controls

Canvas {
    id: chartCanvas
    width: 400
    height: 200

    property var dataPoints: []
    property real maxValue: 100

    onPaint: {
        var ctx = getContext("2d");
        ctx.clearRect(0, 0, width, height);

        if (dataPoints.length === 0) return;

        var barWidth = width / dataPoints.length;
        var maxBarHeight = height - 20;

        ctx.fillStyle = "#4CAF50";
        for (var i = 0; i < dataPoints.length; i++) {
            var barHeight = (dataPoints[i].value / maxValue) * maxBarHeight;
            var x = i * barWidth;
            var y = height - barHeight - 10;

            ctx.fillRect(x + 2, y, barWidth - 4, barHeight);

            // Label
            ctx.fillStyle = "#000000";
            ctx.font = "12px sans-serif";
            ctx.textAlign = "center";
            ctx.fillText(dataPoints[i].label, x + barWidth / 2, height - 5);
            ctx.fillStyle = "#4CAF50";
        }
    }

    function updateData(points) {
        dataPoints = points;
        maxValue = Math.max(...points.map(p => p.value));
        requestPaint();
    }
}
