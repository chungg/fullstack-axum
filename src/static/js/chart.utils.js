let og_datasets;

function toggleView(elem, chart) {
  if (elem.checked) {
    og_datasets = chart.data.datasets.map((dataset) => {
      return dataset.data;
    });
    const total = og_datasets.reduce((r, a) => r.map((b, i) => a[i] + b));
    for (const [index, data] of og_datasets.entries()) {
      chart.data.datasets[index].data = data.map((a, i) => (a / total[i]) * 100);
    }
  } else {
    for (const [index, data] of og_datasets.entries()) {
      chart.data.datasets[index].data = data;
    }
  }
  chart.update();
}

export { toggleView };
