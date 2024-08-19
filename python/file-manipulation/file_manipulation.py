import os

def batch_rename(directory, prefix='', suffix=''):
    for filename in os.listdir(directory):
        if os.path.isfile(os.path.join(directory, filename)):
            new_name = f"{prefix}{filename}{suffix}"
            os.rename(os.path.join(directory, filename), os.path.join(directory, new_name))
    print(f"Renamed files in {directory}")

    # usage example
    directory = 'path/here'
    batch_rename(directory, prefix='new_')

    # this will add new_ as a prefix to all files in the directory = 'path/here'
