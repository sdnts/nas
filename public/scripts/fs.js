const root = location.pathname.replace("/fs", "") || "/";
console.log(root);

const addFile = () => {
  console.log("add file");
};

const addDir = () => {
  const fileList = document.querySelector("#file-list");

  let newDirRow = document.createElement("li");
  newDirRow.classList.add("file-list__item");
  newDirRow.title = "New Folder";

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

const upload = (e) => {
  const files = document.querySelector("#image-file").files;
  const formData = new FormData();
  formData.append("file", files[0]);

  fetch("/fs/Movies", {
    method: "POST",
    body: JSON.stringify({
      name: "something",
      data: formData,
    }),
  }).then((response) => {
    console.log(response);
  });
};
