import json
import os

if __name__ == "__main__":
    name = input("Insert problem's name: ")
    description = input("Insert problem's description: ")
    tests_number = int(input("Insert number of tests: "))
    input_number = int(input("Insert input number: "))
    output_number = int(input("Insert output number: "))
    
    os.system('cls' if os.name == 'nt' else 'clear')
    tests = list()
    for i in range(tests_number):
        print("Making test number", i+1)
        inputs = ""
        for j in range(input_number):
            inputs += input("Insert input {}: ".format(j + 1)) + "\n"
        
        outputs = ""
        for j in range(output_number):
            outputs += input("Insert output {}: ".format(j + 1)) + "\n" 
            
        inputs = inputs[:-1]
        outputs = outputs[:-1]
        
        hidden = input("Will be this test hidden? (y/n)")
        
        tests.append({
            "inputs": inputs,
            "expected": outputs,
            "passed" : False,
            "hidden": hidden == "y"
        })
        
    problem_json = {
        "name": name,
        "description": description,
        "example_case" : [
            tests[0]["inputs"],
            tests[0]["expected"]
        ],
        "runner": {
            "path_to_exe": "",
            "test_cases": tests,
            "passed_cases": 0
        }
    }
    
    if not os.path.exists("problems"):
        os.makedirs("problems")
    with open(f"problems\\{name.replace(" ", "_")}.json", "w") as file:
        json.dump(problem_json, file, indent=4)