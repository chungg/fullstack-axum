import { isKeyDown } from "./listeners.js";

const chartStates = new WeakMap();
const arbLines = {
  id: "chartjs-arb-lines",

  defaults: {
    color: "black",
    lineThreshold: 15,
    lineWidth: 2,
    enableKey: "Control", // draw only when keypressed
    modifierKey: "Shift", // extends line to borders
  },

  beforeInit(chart) {
    chartStates.set(chart, {
      lines: [],
    });
  },

  // https://stackoverflow.com/a/77663018/23104322
  afterEvent(chart, args, options) {
    const { ctx, chartArea } = chart;
    const state = chartStates.get(chart);

    switch (args.event.type) {
      case "mousedown":
        if (args.replay !== true && isKeyDown(options.enableKey)) {
          // registers mousedown event twice if you "click", ignore one.
          state.startXY = { x: args.event.x, y: args.event.y };
        }
        break;
      case "mousemove":
        if (state.startXY) {
          ctx.setLineDash([]);
          ctx.beginPath();
          ctx.lineWidth = options.lineWidth;
          const line = getLineCoords(chartArea, {
            ...state.startXY,
            x2: args.event.x,
            y2: args.event.y,
            full: isKeyDown(options.modifierKey),
          });
          ctx.moveTo(line.x1, line.y1);
          ctx.lineTo(line.x2, line.y2);
          ctx.strokeStyle = "grey";
          ctx.stroke();
        }
        break;
      case "mouseup":
        if (
          isKeyDown(options.enableKey) &&
          Math.abs(state.startXY.x - args.event.x) + Math.abs(state.startXY.y - args.event.y) >
            options.lineThreshold
        ) {
          // don't draw tiny lines
          state.lines.push({
            ...state.startXY,
            x2: args.event.x,
            y2: args.event.y,
            full: isKeyDown(options.modifierKey),
          });
        }
        state.startXY = null;
        break;
    }
  },

  afterDatasetsDraw(chart, args, options) {
    const { ctx, chartArea } = chart;
    const state = chartStates.get(chart);
    ctx.setLineDash([]);
    for (const line of state.lines) {
      ctx.beginPath();
      ctx.lineWidth = options.lineWidth;
      const drawLine = getLineCoords(chartArea, line);
      ctx.moveTo(drawLine.x1, drawLine.y1);
      ctx.lineTo(drawLine.x2, drawLine.y2);
      ctx.strokeStyle = options.color;
      ctx.closePath();
      ctx.stroke();
    }
  },
};

function getLineCoords(chartArea, line) {
  if (line.full === false) {
    return {
      x1: line.x,
      y1: line.y,
      x2: line.x2,
      y2: line.y2,
    };
  }
  // NOTE: i graduated highschool and i had to google how to y = mx + b ...
  const slope = (line.y - line.y2) / (line.x - line.x2);
  const intercept = line.y - slope * line.x;
  let x1 = chartArea.left;
  if (slope * chartArea.left + intercept < chartArea.top) {
    x1 = (chartArea.top - intercept) / slope;
  } else if (slope * chartArea.left + intercept > chartArea.bottom) {
    x1 = (chartArea.bottom - intercept) / slope;
  }
  let x2 = chartArea.right;
  if (slope * chartArea.right + intercept < chartArea.top) {
    x2 = (chartArea.top - intercept) / slope;
  } else if (slope * chartArea.right + intercept > chartArea.bottom) {
    x2 = (chartArea.bottom - intercept) / slope;
  }
  return {
    x1: x1,
    y1: Math.min(Math.max(slope * x1 + intercept, chartArea.top), chartArea.bottom),
    x2: x2,
    y2: Math.min(Math.max(slope * x2 + intercept, chartArea.top), chartArea.bottom),
  };
}

export { arbLines };
