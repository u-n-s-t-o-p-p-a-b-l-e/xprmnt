import csv

def process_csv(input_file, output_file):
    with open(input_file, mode='r') as infile, open(output_file, mode='w', newline='') as outfile:
        reader = csv.reader(infile)
        writer = csv.writer(outfile)

        header = next(reader)
        writer.writerow(header + ['Processed Data'])

        for row in reader:
            processed_data = int(row[1]) * 2
            writer.writerow(row + [processed_data])

# Usage example
input_file = 'input.csv'
output_file = 'output.csv'
process_csv(input_file, output_file)
