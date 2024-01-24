const chartStates = new WeakMap();
const crosshairs = {
  id: "chartjs-crosshairs",

  defaults: {
    color: "black",
    lineWidth: 1,
  },

  beforeInit(chart) {},

  // https://stackoverflow.com/a/77663018/23104322
  afterEvent(chart, args, options) {
    const { ctx, chartArea } = chart;

    switch (args.event.type) {
      case "mousemove":
        drawCrosshair(chart, { x: args.event.x, y: args.event.y });
        break;
    }
  },

  afterDatasetsDraw(chart, args, options) {},
};

function drawCrosshair(chart, coord) {
  const { canvas, ctx, chartArea, scales } = chart;
  if (!scales.y) {
    return;
  }
  if (
    chartArea.right >= coord.x &&
    coord.x >= chartArea.left &&
    chartArea.bottom >= coord.y &&
    coord.y >= chartArea.top
  ) {
    canvas.style.cursor = "crosshair";

    ctx.lineWidth = 1;
    ctx.setLineDash([3, 3]);

    ctx.beginPath();
    ctx.moveTo(chartArea.left, coord.y);
    ctx.lineTo(chartArea.right, coord.y);
    ctx.stroke();
    ctx.closePath();

    // draw y-value
    const yTipHeight = 10;
    ctx.beginPath();
    ctx.fillStyle = "grey";
    ctx.moveTo(chartArea.left, coord.y);
    ctx.lineTo(chartArea.left - yTipHeight / 2, coord.y + yTipHeight);
    ctx.lineTo(0, coord.y + yTipHeight);
    ctx.lineTo(0, coord.y - yTipHeight);
    ctx.lineTo(chartArea.left - yTipHeight / 2, coord.y - yTipHeight);
    ctx.fill();
    ctx.closePath();
    ctx.fillStyle = "white";
    ctx.textBaseline = "middle";
    ctx.textAlign = "right";
    ctx.fillText(
      scales.y.getValueForPixel(coord.y).toFixed(2),
      chartArea.left - yTipHeight / 2,
      coord.y,
    );

    ctx.beginPath();
    ctx.moveTo(coord.x, chartArea.top);
    ctx.lineTo(coord.x, chartArea.bottom);
    ctx.stroke();
    ctx.closePath();
  } else {
    canvas.style.cursor = "default";
  }
}

export { crosshairs };
