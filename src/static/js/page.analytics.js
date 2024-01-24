import { arbLines } from "../../static/js/chart.plugins.arblines.js";
import { crosshairs } from "../../static/js/chart.plugins.crosshairs.js";
import { toggleView } from "../../static/js/chart.utils.js";

// https://stackoverflow.com/questions/67402685/vanilla-js-spa-how-to-load-specific-scripts-under-the-inserted-html-of-the-spa
// https://stackoverflow.com/questions/68554391/how-to-load-javascript-after-all-dom-element-rendered-in-vanilla-javascript
// https://stackoverflow.com/questions/54231533/how-to-create-a-vanilla-js-routing-for-spa
export default function setupAnalyticsPage() {
  window.toggleView = toggleView;

  new Chart(document.getElementById("chartId"), {
    type: "bar",
    options: {
      responsive: true,
      title: {
        display: false,
        text: "",
      },
    },
    data: {
      labels: ["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
    },
  });

  new Chart(document.getElementById("lineChartId"), {
    type: "line",
    plugins: [arbLines, crosshairs],
    options: {
      events: ["mousedown", "mouseup", "mousemove"],
      responsive: true,
      plugins: {
        colors: {
          forceOverride: true,
        },
        legend: {
          position: "top",
        },
        title: {
          display: true,
          text: "Vehicles Sales",
        },
        tooltip: {
          position: "average",
        },
      },
      interaction: {
        axis: "x",
        intersect: false,
        mode: "nearest",
      },
    },
  });
}
