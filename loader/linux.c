#include <linux/module.h>
#include <linux/slab.h>

#define MODULE_NAME "RustyVisor"
#define KB (0x1000)
#define MB (0x1000 * KB)
#define HEAP_SIZE (256 * KB)

extern u32 rustyvisor_load(char *heap, u64 heap_size, char *vmx_region, u64 phys_vmx_region);
extern u32 rustyvisor_unload(void);

extern u32 __module_start;
extern u32 __module_end;

char *vmx_region = NULL;
char *heap = NULL;
phys_addr_t phys_vmx_region;
const size_t vmcs_size = 0x1000;
const size_t vmx_region_size = 0x1000;


static int __init rustyvisor_init(void) {
	int err;

	heap = kmalloc(HEAP_SIZE, GFP_KERNEL);
	if (heap == NULL)
		return -ENOMEM;

	vmx_region = kmalloc(vmx_region_size, GFP_KERNEL);
	if (vmx_region == NULL) {
		kfree(heap);
		return -ENOMEM;
	}

	phys_vmx_region = virt_to_phys(vmx_region);

	err = rustyvisor_load(heap, HEAP_SIZE, vmx_region, phys_vmx_region);
	if (err != 0)
		return -1;

	return 0;
}


static void __exit rustyvisor_exit(void) {
	rustyvisor_unload();
	kfree(heap);
	kfree(vmx_region);
}


module_init(rustyvisor_init);
module_exit(rustyvisor_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
