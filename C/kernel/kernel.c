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

void kmain(void) {
    for (int i = 0; i < 80 * 25; i++) {
        print_char(' ', 0x07);
    }

    vga_index = 0;
    print_string("Hey there, kernel here!", 0x0F);
}
