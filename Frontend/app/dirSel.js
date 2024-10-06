const {invoke} = window.__TAURI__.tauri;

function getDir() {
  invoke("choose_directory").then((dir) => {
    const problem_menu = document.getElementById("problem-menu");
    const card = makeProblemCard("Problem 1");
    problem_menu.innerHTML = card;
  });
}

function makeProblemCard(problem){
  const html = `
  <div class="problem-card">
    <h3>${problem}</h3>
    <p>Problem Description</p>
    <button>View</button>
  </div>
  `;
  return html;
}