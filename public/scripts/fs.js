const createFile = () => {
  // Create an invisible file selector
  const fileInput = document.createElement("input");
  fileInput.setAttribute("type", "file");
  fileInput.onchange = (e) => {
    // When its value changes (when you've selected a file, upload it)
    const file = e.target.files[0];

    if (!file) {
      return;
    }

    // Replace the upload button with a progress bar
    const uploadButton = document.querySelector("#upload-button");
    uploadButton.style.display = "none";

    const uploadProgress = document.querySelector("#upload-progress");
    uploadProgress.style.display = "flex";

    // And upload the file
    const url = `${location.pathname}/${file.name}`;
    // Use XHR to send the file, because we want a progress bar
    const request = new XMLHttpRequest();
    request.open("post", url);
    request.upload.addEventListener("progress", function (e) {
      uploadProgress.innerText = `${Math.floor((e.loaded / e.total) * 100)}%`;
    });

    request.addEventListener("load", function (e) {
      if (request.status !== 200) {
        console.error("Something went wrong");
      }
      location.reload();
    });

    request.send(file);
  };
  fileInput.click();
};

const createDir = () => {
  // Create a new (temp) directory row right below the `..` row
  const fileList = document.querySelector("#file-list");

  let newDirRow = document.createElement("li");
  newDirRow.classList.add("file-list__item");
  newDirRow.title = "New Directory";

  let marker = document.createElement("div");
  marker.classList.add("file-list__item__marker");
  newDirRow.appendChild(marker);

  let icon = document.createElement("div");
  icon.classList.add("icon");
  icon.classList.add("file-list__item__icon");
  icon.setAttribute("data-icon", "directory");
  newDirRow.appendChild(icon);

  let nameInput = document.createElement("input");
  nameInput.name = "dirname";
  nameInput.classList.add("input");
  nameInput.classList.add("file-list__item__name");
  nameInput.onkeyup = (e) => {
    if (e.key === "Enter") {
      // When you press enter on the input, create the dir
      e.preventDefault();
      nameInput.blur();
    } else if (e.key === "Escape") {
      // If you press Esc on the input, cancel creation
      e.preventDefault();
      fileList.removeChild(newDirRow);
    }
  };
  // If you click outside the input, create the dir
  nameInput.onblur = (e) =>
    fetch(`${location.pathname}/${e.target.value}`, {
      method: "POST",
    }).then((res) => {
      if (res.status === 200) {
        location.reload();
      } else {
        console.error("Something went wrong");
      }
    });
  newDirRow.appendChild(nameInput);

  fileList.insertBefore(newDirRow, fileList.children[1]);

  nameInput.focus();
};

const renamePath = (editButton, extension) => {
  // Replace the row's name you clicked on with an input (while saving the old name)
  let listItem = editButton.parentElement;
  let nameElement = listItem.querySelector(".file-list__item__name");
  let currentName = nameElement.innerText;

  let nameInput = document.createElement("input");
  nameInput.name = "dirname";
  nameInput.classList.add("input");
  nameInput.classList.add("file-list__item__name");
  nameInput.value = currentName;
  nameInput.onkeyup = (e) => {
    if (e.key === "Enter") {
      // When you press enter on the input, rename the path
      e.preventDefault();
      nameInput.blur();
    } else if (e.key === "Escape") {
      // If you press Esc on the input, cancel rename
      e.preventDefault();
      listItem.replaceChild(nameElement, nameInput);
    }
  };
  // If you click outside the input, rename the path
  nameInput.onblur = (e) =>
    fetch(`${location.pathname}/${currentName}`, {
      method: "PUT",
      body: e.target.value,
    }).then((res) => {
      if (res.status === 200) {
        location.reload();
      } else {
        console.error("Something went wrong");
      }
    });

  listItem.replaceChild(nameInput, nameElement);

  nameInput.focus();

  // Select only the name
  if (extension) {
    const indexOfBeginningOfExtension = nameInput.value.lastIndexOf(extension);
    nameInput.setSelectionRange(0, indexOfBeginningOfExtension - 1, "backward"); // -1 to account for dot before extension
  } else {
    nameInput.select();
  }
};

const tryDeletePath = (removeButton) => {
  // When you press the delete button, replace it with another button that will have to be clicked again to confirm deletion
  const listItem = removeButton.parentElement;
  const confirmButton = listItem.querySelector("#confirm-remove-button");

  // Memory leak is fine
  listItem.addEventListener("mouseleave", () => {
    removeButton.style.display = "block";
    confirmButton.style.display = "none";
  });

  removeButton.style.display = "none";
  confirmButton.style.display = "block";
};

const deletePath = (name) => {
  fetch(`${location.pathname}/${name}`, {
    method: "DELETE",
  }).then((res) => {
    if (res.status === 200) {
      location.reload();
    } else {
      console.error("Something went wrong");
    }
  });
};
