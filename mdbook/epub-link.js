// Wait for the full DOM to ensure .right-buttons exists
window.addEventListener('DOMContentLoaded', () => {
  const rightButtons = document.querySelector(".right-buttons");
  const epubLink = document.createElement("a");
  epubLink.href = "Discovery.epub";
  epubLink.textContent = "📘EPUB";
  epubLink.target = "_blank";

  // Style it clearly for now (you can adjust later)
  // epubLink.style.marginLeft = "1rem";
  // epubLink.style.fontWeight = "bold";
  // epubLink.style.fontSize = "14px";
  // epubLink.style.color = "blue";
  // epubLink.style.textDecoration = "underline";

  rightButtons.appendChild(epubLink);
});
