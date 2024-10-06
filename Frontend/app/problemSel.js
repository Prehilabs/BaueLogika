const { invoke } = window.__TAURI__.tauri;

//Code executed on page load
invoke("get_problems").then((problems) => {
    makeCards(problems);
});

function makeCards(problems){
    const problem_menu = document.getElementById("problem-menu");
    for (const problem of problems)
    {
        const card = makeProblemCard(problem);
        problem_menu.innerHTML += card;
    }
}


function makeProblemCard(problem){
    const html = `
    <div class="problem-card">
      <h3>${problem.name}</h3>
      <p>${problem.description}</p>
      <button>Try out! ></button>
    </div>
    `;
    return html;
  }