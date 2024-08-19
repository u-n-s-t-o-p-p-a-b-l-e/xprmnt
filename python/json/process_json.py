import json

def read_and_modify_json(file_path):
    with open(file_path, 'r') as file:
        data = json.load(file)

    data['new_key'] = 'new_value'

    data['name'] = 'Jane Doe'

    new_project = {
        "name" : "Project Gamma",
        "description": "A new exciting project",
        "completed": False
    }
    data['projects'].append(new_project)

    with open(file_path, 'w') as file:
        json.dump(data, file, indent=4)
