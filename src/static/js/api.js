function updateChart(data, target) {
  const chart = Chart.getChart(target);
  for (const dataset of data.datasets) {
    chart.data.datasets.push(dataset);
  }
  if (data.labels != null) {
    chart.data.labels = data.labels;
  }
  chart.update();
}

function displayPrices(data) {
  const chart = Chart.getChart("priceChart");
  const table = Tabulator.findTable("#priceTable")[0];
  const prices = data.indicators.quote[0].close;
  // ideally, should check overlap of labels between datasets
  table.updateOrAddData(data.timestamp.map((a, i) => ({ date: a, [data.meta.symbol]: prices[i] })));
  if (!table.columnManager.columns.length) {
    table.addColumn({ title: "Date", field: "date", frozen: true }, true);
  }
  table.addColumn({
    title: data.meta.symbol,
    field: data.meta.symbol,
    hozAlign: "right",
    headerSort: false,
    formatter: "money",
    formatterParams: { thousand: false },
  });
  chart.data.labels = data.timestamp;
  // change prices to percentage so it's comparable
  chart.data.datasets.push({
    label: data.meta.symbol,
    data: prices.map((x) => ((x - prices[0]) / prices[0]) * 100),
  });
  chart.update();
}

function handler(evt) {
  // https://www.reddit.com/r/htmx/comments/10sdk43/comment/j72m2j7/
  const data = JSON.parse(document.getElementById(evt.detail.dataId).textContent);
  // TODO: improve this so we don't have a giant switch. also need to handle origin
  // in case same endpoint displayed differently
  switch (evt.detail.path) {
    case "/hx/v1/data/sales":
      updateChart(data, "lineChartId");
      break;
    case "/hx/v1/data/random":
      updateChart(data, "chartId");
      break;
    case "/hx/v1/data/market/prices":
      displayPrices(data);
      break;
  }
  document.getElementById(evt.detail.dataId).remove();
}

export { handler };
