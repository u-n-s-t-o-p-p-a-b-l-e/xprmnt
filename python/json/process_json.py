import json

def read_and_modify_json(file_path):
    with open(file_path, 'r') as file:
        data = json.load(file)
