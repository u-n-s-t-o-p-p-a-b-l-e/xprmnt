#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

#define MAX_WORD_LENGTH 100

int main() {
  FILE *file;
  char filename[100];
  char word[MAX_WORD_LENGTH];
  int word_count = 0;
  int in_word = 0;
  char c;

  printf("Enter the filename: ");
  scanf("%s", filename);

  file = fopen(filename, "r");
  if (file == NULL) {
    printf("Could not open file %s\n", filename);
    return 1;
  }

  while ((c = fgetc(file)) != EOF) {
    if (isalnum(c)) {
      if (!in_word) {
        in_word = 1;
        word_count++;
      }
    } else {
      in_word = 0;
    }
  }
   fclose(file);

   printf("The file contains %d words.\n", word_count);

   return 0;
}

