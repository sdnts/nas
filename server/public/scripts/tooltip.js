(() => {
  let visible = false;

  const tooltip = document.querySelector(".tooltip");
  const button = document.querySelector(".tooltip button");
  const content = document.querySelector(".tooltip ul");

  if (!tooltip || !button || !content) {
    return;
  }

  const hideTooltip = () => {
    visible = false;
    content.style.visibility = "hidden";
  };

  const showTooltip = () => {
    visible = true;
    content.style.visibility = "visible";
  };

  const toggleTooltip = () => {
    visible ? hideTooltip() : showTooltip();
  };

  button.addEventListener("click", () => toggleTooltip());
  content.addEventListener("click", () => hideTooltip());
})();
