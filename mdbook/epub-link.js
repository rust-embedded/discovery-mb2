window.addEventListener('DOMContentLoaded', () => {
  const rightButtons = document.querySelector(".right-buttons");
  if (!rightButtons) return;

  const epubLink = document.createElement("a");
  epubLink.href = "/static/Discovery.epub";
  epubLink.textContent = "📘ePUB";
  epubLink.download = "Discovery.epub"; // force download
  epubLink.classList.add("epub-download");

  rightButtons.appendChild(epubLink);
});

