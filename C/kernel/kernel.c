unsigned short *vga_buffer = (unsigned short *)0xB8000;
int vga_index = 0;

void print_char(char ch, int color) {
    vga_buffer[vga_index] = (unsigned short) (ch | color << 8);
    vga_index++;
}

void print_string(const char *str, int color) {
    for (int i = 0; str[i] != '\0'; i++) {
        print_char(str[i], color);
    }
}
