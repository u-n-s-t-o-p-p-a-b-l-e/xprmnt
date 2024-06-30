import argparse
import threading
import time

def print_numbers(name, count):
    for i in range(count):
        print(f"Thread {name}: {i}")
        time.sleep(1)
