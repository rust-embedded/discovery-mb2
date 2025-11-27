window.addEventListener('DOMContentLoaded', () => {
  const rightButtons = document.querySelector(".right-buttons");
  if (!rightButtons) return;

  const epubLink = document.createElement("a");
  epubLink.href = "/discovery-mb2/Rust Embedded MB2 Discovery Book.epub";
  epubLink.textContent = "📘ePUB";
  epubLink.download = "Rust Embedded MB2 Discovery Book.epub"; // force download
  epubLink.classList.add("epub-download");

  rightButtons.appendChild(epubLink);
});
