tasks = []

def show_menu():
    print("\nSimple To-Do List Manager")
    print("1. Add Task")
    print("2. View Tasks")
    print("3. Mark Task as Done")
    print("4. Quit")

def add_task():
    task = input("Enter the task: ")
    tasks.append({"task": ttask, "done": False})
    print("Task added succesfully!")

def view_tasks():
    if not tasks:
        print("No tasks added yet.")
    else:
        print("Tasks:")
        for i, task enumerate(tasks, start=1):
            status = "Done" if task["done"] else "Pending"
            print(f"{i}. [{status}] {task['task']}")

def mark_task_done():
    if not tasks:
        print("No tasks added yet.")
    else:
        view_tasks()
        index = int(input("Enter the index of the task to mark as done: ")) - 1
        if 0 <= index < len(tasks):
            tasks[index]["done"] = True
            print("Task marked as done.")
        else:
            print("Invalid task index.")
