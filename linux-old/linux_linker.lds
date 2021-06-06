SECTIONS
{

	__module_start = .;

	.text BLOCK(4K) : ALIGN(4K)
	{
		__text_start = .;
		*(.text)
		__text_end = .;
	}

	.rela BLOCK(4K) : ALIGN(4K)
	{
		__rela_start = .;
		*(.rela)
		__rela_end = .;
	}

	.rodata BLOCK(4K) : ALIGN(4K)
	{
		__rodata_start = .;
		*(.rodata)
		__rodata_end = .;
	}

	.eh_frame BLOCK(4K) : ALIGN(4K)
	{
		__eh_frame_start = .;
		*(.eh_frame)
		__eh_frame_end = .;
	}

	.gcc_except_table BLOCK(4K) : ALIGN(4K)
	{
		__gcc_except_table_start = .;
		*(.gcc_except_table)
		__gcc_except_table_end = .;
	}

	.data BLOCK(4K) : ALIGN(4K)
	{
		__data_start = .;
		*(.data)
		__data_end = .;
	}

	.bss BLOCK(4K) : ALIGN(4K)
	{
		__bss_start = .;
		*(COMMON)
		*(.bss)
		__bss_end = .;
	}

	.debug BLOCK(4K) : ALIGN(4K)
	{
		__debug_start = .;
		*(.debug)
		__debug_end = .;
	}

	.strtab BLOCK(4K) : ALIGN(4K)
	{
		__strtab_start = .;
		*(.strtab)
		__strtab_end = .;
	}

	__module_end = .;
}
