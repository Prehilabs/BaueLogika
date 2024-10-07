const {invoke} = window.__TAURI__.tauri;

window.onload = function() {
    var queryParams = new URLSearchParams(window.location.search);
    var problemName = queryParams.get("problem");
    invoke("load_problem", {problemName: problemName})
    .then((problem) => {
        document.getElementById("problem-title").innerHTML = problem.info.name;
        document.getElementById("problem-description").innerHTML = problem.info.description;
        document.getElementById("example").innerHTML += makeExampleDiv(problem.info.example_case);
        addTestCases(problem.test_cases);
    })
};

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
    <div class="test-case">
        <p class="test-case-tittle">Test case ${index}</p>
        <p>Input: ${testCase.inputs}</p>
        <p>Output: ${testCase.expected}</p>
    </div>
    `;
    return html;
}

function makeHiddenTestCase(index)
{
    const html = `
    <div class="test-case">
        <p class="test-case-tittle">Test case ${index}</p>
        <p class="test-case-text">Information hidden</p>
    </div>
    `;
    return html;
}