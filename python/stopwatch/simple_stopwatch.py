import time

def stopwatch():
    print("Press Enter to start the stopwatch. Press ctrl+C to stop it.")
    input()
    start_time = time.time()
    print("Started. Press Ctrl+C to stop.")
    try: 
        while True:
            elapsed_time = time.time() - start_time
            print(f"\rElapsed time: {elapsed_time:.2f} seconds", end="")
            time.sleep(0.1)
    except KeyboardInterrupt:
        print(f"\nStopped. Total time: {elapsed_time:.2f} seconds")

#usage example
stopwatch()
