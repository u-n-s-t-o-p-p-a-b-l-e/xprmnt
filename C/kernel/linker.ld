ENTRY(start)

PHDRS
{
	text PT_LOAD FLAGS(0x5);
	data PT_LOAD FLAGS(0x6);
}

SECTIONS
{
	. = 0x100000;
	.text :
	{
		*(.multiboot)
		*(.text)
	} :text

	.data : { *(data) } :data

	.bss : { *(.bss) } :data

	. = ALIGN(0x1000);
	.stack :
	{
		. = . + 0x4000;
	}
}
