#include <linux/module.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <linux/tboot.h>

#include <asm/msr.h>
#include <asm/processor.h>
#include <asm/tlbflush.h>
#include <asm/virtext.h>


#define MODULE_NAME "RustyVisor"
#define MB (0x1000 * 0x1000)
#define HEAP_SIZE (1 * 0x1000)

extern u32 entry(char *heap, char *vmx_region, u64 phys_vmx_region);

char *vmx_region = NULL;
char *heap = NULL;
phys_addr_t phys_vmx_region;
const size_t vmcs_size = 0x1000;
const size_t vmx_region_size = 0x1000;



static int __init hype_init(void) {
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

	err = entry(heap, vmx_region, phys_vmx_region);
	if (err != 0)
		return -1;

	return 0;
}


static void __exit hype_exit(void) {
	kfree(heap);
	kfree(vmx_region);
}


module_init(hype_init);
module_exit(hype_exit);


MODULE_AUTHOR("Ian Kronquist <iankronquist@gmail.com>");
MODULE_LICENSE("Dual MIT/GPL");
