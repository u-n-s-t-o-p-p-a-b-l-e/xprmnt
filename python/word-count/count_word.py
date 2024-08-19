from collections import Counter
import re

def count_word_frequency(file_path):
    with open(file_path, 'r') as file:
        text = file.read().lower()

    words = re.findall(r'\b\w+\b', text)
    frequency = Counter(words)

    for word, count in frequency.most_common():
        print(f'{word}: {count}')

# Usage example
file_path = 'example.txt'
count_word_frequency(file_path)
