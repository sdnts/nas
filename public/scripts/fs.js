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

  let icon = document.createElement("div");
  icon.classList.add("icon");
  icon.classList.add("file-list__item__icon");
  icon.setAttribute("data-icon", "dir");
  newDirRow.appendChild(icon);

  let form = document.createElement("form");
  form.action = `/fs${root}`;
  form.method = "POST";
  newDirRow.appendChild(form);

  let input = document.createElement("input");
  input.name = "dirname";
  input.classList.add("input");
  input.classList.add("file-list__item__name");
  form.appendChild(input);

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

const renamePath = () => {};

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
