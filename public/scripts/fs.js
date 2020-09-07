const root = location.pathname.replace("/fs", "") || "/";

const addFile = () => {
  const fileInput = document.createElement("input");
  fileInput.setAttribute("type", "file");
  fileInput.onchange = uploadFile;
  fileInput.click();
};

const addDir = () => {
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

  let input = document.createElement("input");
  input.name = "dirname";
  input.classList.add("input");
  input.classList.add("file-list__item__name");
  input.onsubmit = (e) => {
    console.log("create", e.target.value);
  };
  input.oncancel = (e) => {
    console.log("cancel");
  };
  newDirRow.appendChild(input);

  fileList.insertBefore(newDirRow, fileList.children[1]);

  input.focus();
};

const uploadFile = (e) => {
  const file = e.target.files[0];

  if (!file) {
    return;
  }

  const uploadButton = document.querySelector("#upload-button");
  uploadButton.style.display = "none";

  const uploadProgress = document.querySelector("#upload-progress");
  uploadProgress.style.display = "flex";

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

  const formData = new FormData();
  formData.append("file", file);
  request.send(formData);
};

const uploadDir = () => {};

const startRenamePath = (editButton) => {
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
      e.preventDefault();
      nameInput.blur();
    } else if (e.key === "Escape") {
      e.preventDefault();
      listItem.replaceChild(nameElement, nameInput);
    }
  };
  nameInput.onblur = (e) => renamePath(currentName, e.target.value);
  listItem.replaceChild(nameInput, nameElement);

  nameInput.focus();
  nameInput.select();
};

const renamePath = (currentName, newName) =>
  fetch(`${location.pathname}/${currentName}`, {
    method: "PUT",
    body: newName,
  }).then((res) => {
    if (res.status === 200) {
      location.reload();
    } else {
      console.error("Something went wrong");
    }
  });

const confirmDeletePath = (removeButton) => {
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
