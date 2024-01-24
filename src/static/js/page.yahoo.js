import { crosshairs } from "../../static/js/chart.plugins.crosshairs.js";

export default function setupYahooPage() {
  new Chart(document.getElementById("priceChart"), {
    type: "line",
    plugins: [crosshairs],
    options: {
      events: ["mousedown", "mouseup", "mousemove"],
      responsive: true,
      layout: {
        padding: {
          left: 15,
        },
      },
      scales: {
        x: {
          parsing: false,
          type: "time", // to convert iso date to english
          time: {
            unit: "day",
          },
        },
      },
      plugins: {
        colors: {
          forceOverride: true,
        },
        legend: {
          position: "top",
        },
        title: {
          display: false,
          text: "Security Prices",
        },
        tooltip: {
          position: "nearest",
        },
      },
      interaction: {
        axis: "x",
        intersect: false,
        mode: "nearest",
      },
    },
  });

  new Tabulator("#priceTable", {
    layout: "fitData",
    index: "date",
    pagination: true, //enable.
    paginationSize: 10, // this option can take any positive integer value
    placeholder: function () {
      //set placeholder based on if there are currently any header filters
      return this.initialised && this.getHeaderFilters().length ? "No Matching Data" : "No Data";
    },
  });
}
