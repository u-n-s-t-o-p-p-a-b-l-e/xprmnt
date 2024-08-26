#include <stdio.h>
#include <stdlib.h>

#define BUFFER_SIZE 1024

void process_file(FILE *input, FILE *output, char key) {
    unsigned char buffer[BUFFER_SIZE];
    size_t bytes_read;

    while ((bytes_read = fread(buffer, 1, BUFFER_SIZE, input)) > 0) {
    for (size_t i = 0; i < bytes_read; i++) {
        buffer[i] ^= key;
    }
    fwrite(buffer, 1, bytes_read, output);
    }
}

int main(int argc, char *argv[]) {
    if (argc != 4) {
        fprintf(stderr, "Usage: %s <input_file> <output_file> <key>\n", argv[0]);
        return 1;
    }

    FILE *input = fopen(argv[1], "rb");
    FILE *output = fopen(argv[2], "wb");
    char key = atoi(argv[3]);

    if (!input || !output) {
        fprintf(stderr, "Error opening files.\n");
        return 1;
    }

    process_file(input, output, key);

    fclose(input);
    fclose(output);
    printf("File processed succesfully\n");

    return 0;
}
