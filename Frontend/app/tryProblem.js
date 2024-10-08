const {invoke} = window.__TAURI__.tauri;
const {listen} = window.__TAURI__.event;
const dropArea = document.getElementById("drop-area");
var problem_obj = null;

window.onload = function() {
    var queryParams = new URLSearchParams(window.location.search);
    var problemName = queryParams.get("problem");
    invoke("load_problem", {problemName: problemName})
    .then((problem) => {
        problem_obj = problem;
        document.getElementById("problem-title").innerHTML = problem.info.name;
        document.getElementById("problem-description").innerHTML = problem.info.description;
        document.getElementById("example").innerHTML += makeExampleDiv(problem.info.example_case);
    })
    .catch((e) => {
        invoke("show_error", {message: e});
        window.location.href = "problemSel.html";
    });
};

listen('tauri://file-drop', event => {
  invoke("run_problem", {problem: problem_obj.info.name, exePath: event.payload[0], testCases: problem_obj.test_cases});
})

function makeExampleDiv(example)
{
    const html = `
    <p class="example-title">Example</p>
    <p class="regular-text">Input: ${example[0]}</p>
    <p class="regular-text">Output: ${example[1]}</p>
    `;
    return html;
}

function addTestCases(testCases)
{
    const testCasesDiv = document.getElementById("test-cases");
    for (var i = 0; i < testCases.length; i++)
    {
        if (testCases[i].hidden)
        {
            testCasesDiv.innerHTML += makeHiddenTestCase(i + 1);
        }
        else
        {
            testCasesDiv.innerHTML += makeVisibleTestCase(testCases[i], i + 1);
        }
    }
}

function makeVisibleTestCase(testCase, index)
{
    const html = `
    <div class="visible-test-case">
        <div class="test-case-text">
            <p>Input: ${testCase.inputs}</p>
            <p>Output: ${testCase.expected}</p>
        </div>
        <div class="test-case-title">
            <p>Test case ${index}</p>
        </div>
    </div>
    `;
    return html;
}

function makeHiddenTestCase(index)
{
    const html = `
    <div class="hidden-test-case">
        <div class="test-case-title">
            <p>Test case ${index}</p>
        </div>
        <div class="test-case-text">
            <p>HIDDEN</p>
        </div>
    </div>
    `;
    return html;
}