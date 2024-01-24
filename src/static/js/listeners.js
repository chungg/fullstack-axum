const isKeyDown = (() => {
  // https://stackoverflow.com/a/48750898
  const state = {};

  // biome-ignore lint: let it mod
  window.addEventListener("keyup", (e) => (state[e.key] = false));
  // biome-ignore lint: let it mod
  window.addEventListener("keydown", (e) => (state[e.key] = true));

  return (key) => (Object.hasOwn(state, key) && state[key]) || false;
})();

export { isKeyDown };
