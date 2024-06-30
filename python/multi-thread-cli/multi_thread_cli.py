import argparse
import threading
import time

def print_numbers(name, count):
    for i in range(count):
        print(f"Thread {name}: {i}")
        time.sleep(1)

def main():
    parser = argparse.ArgumentParser(description='A multi-threaded Python CLI app.')
    parser.add_argument('--threads', type=int, default=2, help='Number of threads to spawn')
    parser.add_argument('--count', type=int, default=5, help='Count to print up to in each thread')
    args = parser.parse_args()

    threads = []
    for i in range(args.threads):
        thread = threading.Thread(target=print_numbers, args=(i, args.count))
        threads.append(thread)
        thread.start()

    for thread in threads:
        thread.join()

    print("All threads have completed.")

if __name__ == '__main__':
    main()
