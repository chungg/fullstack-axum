import { handler } from "../../static/js/api.js";
import setupYahooPage from "../../static/js/page.yahoo.js";

const router = (evt) => {
  let func;
  switch (window.location.pathname) {
    case "/analytics":
      // dynamic modules example. not sure this is relevant if you bundle js
      import("../../static/js/page.analytics.js").then((module) => {
        module.default();
        // custom trigger as this will lose race with load event
        htmx.trigger("body", "pageReady", {});
      });
      break;
    case "/yahoo":
      func = setupYahooPage();
      // custom event so that setup logic doesn't race hx-trigger: load
      // technically not needed unless setup logic is absurdly long
      document.body.dispatchEvent(new Event("pageReady"));
      break;
  }
};

// NOTE: addlistener should target named func to avoid dup listeners if called multiple times
// handle full page load
window.addEventListener("DOMContentLoaded", router);
// handle swap load
window.addEventListener("initPage", router);
// handle browser back/fwd. location.reload() on popstate works as well but is terrible solution
window.addEventListener("htmx:historyRestore", router);

window.addEventListener("apiResponse", handler);
