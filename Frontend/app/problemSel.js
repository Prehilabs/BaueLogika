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
      <p class="problem-header">${problem.name}</p>
      <hr>
      <p class="problem-body">${problem.description}</p>
      <button onclick=\"tryOutProblem('${problem.name}')\>Try it out! ></button>
    </div>
    `;
    return html;
}

function tryOutProblem(problem)
{
    window.location.href = "problemInfo.html";
}