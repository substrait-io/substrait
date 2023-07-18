
// Add additional behavior to jagged-box admonitions to make them appear as
// call to actions.
window.addEventListener("load", (event) => {
  const matches = document.documentElement.querySelectorAll('.jagged-box')
  matches.forEach((elem) => {
	  var elemTitle = elem.children[0].innerText;
	  var elemText = elem.children[1].innerText;

	  // Create the new title with the contents from the old text paragraph.
	  var newTitle = elem.children[1].cloneNode(true);
	  newTitle.classList.add("admonition-title");

	  // Remove all of the existing elements so we can rework it.
	  while (elem.hasChildNodes()) {
		  elem.removeChild(elem.firstChild);
	  }

	  // Add the new title as the first node.
	  elem.appendChild(newTitle);

	  // Create the close button.
	  var newText = document.createElement("p");
	  newText.classList.add("admonition-calltoaction-close");
	  let text2 = document.createTextNode("\u{00d7}");
	  newText.appendChild(text2);
	  elem.appendChild(newText);
	  newText.onclick = function () {
		  elem.style.display = "none";
		  localStorage.setItem(elemTitle, "DISMISSED");
	  };

	  // Determine if we have been previously dismissed.
	  let currentState = localStorage.getItem(elemTitle);
	  if (currentState == "DISMISSED") {
		  // This call to action has already been seen.
		  elem.style.display = "none";
	  } else {
		  elem.style.display = "block";
	  }
  });
});
