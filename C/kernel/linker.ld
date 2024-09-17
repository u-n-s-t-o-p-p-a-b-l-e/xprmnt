ENTRY(start)

PHDRS
{
	text PT_LOAD FLAGS(0x5);
	data PT_LOAD FLAGS(0x6);
}

SECTIONS
{
	. = 0x100000;
}
