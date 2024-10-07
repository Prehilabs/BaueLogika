const { invoke } = window.__TAURI__.tauri;

//Code executed on page load
invoke("get_problems_info")
.then((problems) => {
    makeCards(problems);
})
.catch( () => {
    const error_html =`
    <p>No problems found in the problem path</p>
    <button onclick=\"window.location.href = 'index.html'\">Go back</button>
    `;
    document.body.innerHTML = error_html; 
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
      <button onclick=\"tryOutProblem()\">Try it out! ></button>
    </div>
    `;
    return html;
}

function tryOutProblem(problemName)
{
    window.location.href = "problemInfo.html?problem=" + problemName;
}