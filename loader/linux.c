#include <linux/module.h>
#include <linux/vmalloc.h>

#define MODULE_NAME "RustyVisor"
#define KB (1024)
#define MB (1024 * KB)
#define HEAP_SIZE (256 * KB)

struct translation {
	u64 physical;
	u64 virtual;
};


struct kernel_data {
	u64 heap_size;
	u64 translations_count;
	u8 *heap;
	struct translation *translations;
};


struct kernel_data kernel_data;


extern u32 rustyvisor_load(struct kernel_data *);
extern u32 rustyvisor_unload(void);

extern char __module_start;
extern char __module_end;


static int build_translations(struct kernel_data *info) {
	int i = 0;
	u8 *heap_page;
	char *module_page;
	printk("build translations entered\n");

	info->translations_count = (info->heap_size + (uintptr_t)&__module_end -
			(uintptr_t)&__module_start) / PAGE_SIZE;
	info->translations = vmalloc(info->translations_count * sizeof(struct translation));
	if (info->translations == 0) {
		printk("build translations vmalloc failed\n");

		return -1;
	}
	printk("build translations vmalloc'd %p\n", info->translations);

	for (heap_page = info->heap;
			heap_page < info->heap + info->heap_size;
			heap_page += PAGE_SIZE, ++i) {
		info->translations[i].virtual = (u64)heap_page;
		info->translations[i].physical = virt_to_phys(heap_page);
		printk("heap page %p\n", heap_page);

	}
	printk("build translations heap translated\n");

	for (module_page = &__module_start;
			module_page < &__module_end;
			module_page += PAGE_SIZE, ++i) {
		info->translations[i].virtual = (u64)module_page;
		info->translations[i].physical = virt_to_phys(module_page);
		printk("module page %p\n", module_page);

	}
	printk("build translations module translated\n");

	return 0;
}


static int __init rustyvisor_init(void) {
	int err;

	kernel_data.heap_size = HEAP_SIZE;
	kernel_data.heap = vmalloc(HEAP_SIZE);
	if (kernel_data.heap == NULL)
		return -ENOMEM;
	printk("heap alloc'd\n");

	err = build_translations(&kernel_data);
	if (err != 0)
		return -ENOMEM;

	printk("build translations alloc'd\n");

	err = rustyvisor_load(&kernel_data);
	if (err != 0)
		return -1;

	return 0;
}


static void __exit rustyvisor_exit(void) {
	rustyvisor_unload();
	vfree(kernel_data.heap);
	vfree(kernel_data.translations);
}


module_init(rustyvisor_init);
module_exit(rustyvisor_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
