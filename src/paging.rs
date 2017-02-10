use core::marker::PhantomData;
use core::mem;
//use core::ptr::Unique;
use os;
use spin::{Once, RwLock};

const PAGE_TABLE_SIZE: usize =  512;

static OFFSET: Once<isize> = Once::new();

// FIXME make enum
pub const PAGE_PRESENT: u64 = 1;
pub const PAGE_WRITABLE: u64 = 1 << 1;
/*
//pub const PAGE_WRITABLE: u64 = (1 << 1) | (1 << 63);
pub const PAGE_READ_ONLY: u64 = 1 << 63;
pub const PAGE_USER_ACCESSIBLE: u64 = 1 << 2;
pub const PAGE_CACHE_WRITE_THROUGH: u64 = 1 << 3;
pub const PAGE_NO_CACHE: u64 = 1 << 4;
pub const PAGE_ACCESSED: u64 = 1 << 5;
pub const PAGE_DIRTY: u64 = 1 << 6;
pub const PAGE_HUGE: u64 = 1 << 7;
pub const PAGE_GLOBAL: u64 = 1 << 8;
pub const PAGE_NO_EXECUTE: u64 = 1 << 63;
*/

unsafe fn write_cr3(val: u64) {
    asm!(
        "mov $0, %cr3"
        :
        : "r"(val)
        :
        );
}

unsafe fn read_cr3() -> u64 {
    let ret: u64;
    asm!(
        "mov %cr3, $0"
        : "=r"(ret)
        :
        :
        );
    ret
}

lazy_static! {
    static ref CURRENT_PAGE_TABLE: RwLock<CurrentPageTable> = {
        //unsafe {
            RwLock::new(CurrentPageTable {
                //pt: Unique::new(CurrentPageTable::from_cpu())
            })
        //}
    };
}

#[derive(Clone, Copy, Default)]
struct PageTableEntry(u64);

pub trait PageTableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl PageTableLevel for Level4 {}
impl PageTableLevel for Level3 {}
impl PageTableLevel for Level2 {}
impl PageTableLevel for Level1 {}

pub trait HierarchicalLevel: PageTableLevel {
    type NextLevel: PageTableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}


struct PageTableLayer<L: PageTableLevel> {
    entries: [PageTableEntry; PAGE_TABLE_SIZE],
    level: PhantomData<L>,
}

#[derive(Default)]
struct PageTable(PageTableLayer<Level4>);


struct CurrentPageTable {
    //pt: Unique<PageTable>,
}

pub struct VirtualAddress(u64);
pub struct PhysicalAddress(u64);

fn init_offset(translations: *const os::Translation, translation_size: u64) {
    assert!(!translations.is_null());
    assert!(translation_size > 0);

    OFFSET.call_once(|| {
        unsafe {
            ((*translations).virt - (*translations).phys) as isize
        }
    });
}

fn get_offset() -> isize {
    *OFFSET.call_once(|| panic!("Must initialize offset before using it"))
}

pub fn init(translations: *const os::Translation, translation_size: u64) -> Result<(), ()> {
    if translations.is_null() || translations.is_null() {
        return Err(());
    }

    init_offset(translations, translation_size);
    Ok(())
}

impl PageTableEntry {
    fn set(&mut self, flags: u64) {
        self.0 = flags;
    }

    fn is_present(&mut self) -> bool {
        self.0 & PAGE_PRESENT != 0
    }

    fn as_virtual_address(&self) -> VirtualAddress {
        VirtualAddress(self.0 & 0x000fffff_fffff000)
    }
}

impl PageTable {
    fn map(&mut self, virt: VirtualAddress, phys: PhysicalAddress, flags: u64) {
        let mut p3 = self.0.next_layer_create(virt.p4_index());
        let mut p2 = p3.next_layer_create(virt.p3_index());
        let mut p1 = p2.next_layer_create(virt.p2_index());

        assert!(!p1.entries[virt.p1_index()].is_present());
        p1.entries[virt.p1_index()].set(phys.0 | flags | PAGE_PRESENT);
    }

    pub fn map_hypervisor(&mut self, translations: *const os::Translation, translation_size: u64) -> Result<(), ()> {
        assert!(!translations.is_null());
        assert!(translation_size != 0);
        for off in 0..translation_size {
            let virt;
            let phys;
            unsafe {
                phys = PhysicalAddress((*translations.offset(off as isize)).phys);
                virt = VirtualAddress((*translations.offset(off as isize)).virt);
            }
            self.map(virt, phys, PAGE_WRITABLE);

        }
        Err(())
    }


}

impl CurrentPageTable {
    /*
    fn get_top_level(&mut self) -> &mut PageTableLayer<Level4> {
        unsafe {
            &mut self.pt.get_mut().0
        }
    }
    */

    unsafe fn from_cpu() -> *mut PageTable {
        read_cr3() as *mut PageTable
    }

    fn load(&mut self, pt: *mut PageTable) -> *mut PageTable {
        let cur;
        unsafe {
            cur = CurrentPageTable::from_cpu();
            write_cr3(pt as u64);
        }
        cur
    }
}


impl<L: PageTableLevel> Default for PageTableLayer<L> {
    fn default() -> Self {
        PageTableLayer {
            entries: [Default::default(); PAGE_TABLE_SIZE],
            level: PhantomData,
        }
    }
}

impl<L: PageTableLevel> PageTableLayer<L> {
    fn as_virtual_address(&self) -> VirtualAddress {
        VirtualAddress(self as *const Self as u64)
    }
}


impl<L: HierarchicalLevel> PageTableLayer<L> {

    /*
    fn next_layer_mut(&mut self, index: usize) -> Option<&mut PageTableLayer<L::NextLevel>> {
        if self.entries[index].is_present() {
            let virt = self.entries[index].as_virtual_address();
            unsafe {
                Some(&mut *(virt.0 as * mut _))
            }
        } else {
            None
        }
    }
    */

    fn next_layer_create(&mut self, index: usize) -> &mut PageTableLayer<L::NextLevel> {
        if self.entries[index].is_present() {
            let virt = self.entries[index].as_virtual_address();
            unsafe {
                &mut *(virt.0 as * mut _)
            }
        } else {
            let mut new_layer: PageTableLayer<L::NextLevel> = Default::default();
            let virt = new_layer.as_virtual_address();
            let phys = virt.as_physical_address();
            self.entries[index].set(phys.0 | PAGE_WRITABLE);
            unsafe {
                mem::transmute(&mut new_layer)
            }
        }
    }

    /*
    unsafe fn from_virtual_address<'a>(virt: VirtualAddress) -> Option<&'a PageTableLayer<L>> {
        if virt.0 == 0 {
            None
        } else {
            Some(&*(virt.0 as *const _))
        }
    }
    */
}

impl PhysicalAddress {
    pub fn as_virtual_address(&self) -> VirtualAddress {
        VirtualAddress(self.0 + get_offset() as u64)
    }
}

impl VirtualAddress {
    pub fn as_physical_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.0 - get_offset() as u64)
    }

    fn p4_index(&self) -> usize {
        ((self.0 >> 27) & 0o777) as usize
    }

    fn p3_index(&self) -> usize {
        ((self.0 >> 18) & 0o777) as usize
    }

    fn p2_index(&self) -> usize {
        ((self.0 >> 9) & 0o777) as usize
    }

    fn p1_index(&self) -> usize {
        ((self.0 >> 0) & 0o777) as usize
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {
    use os;
    use paging;

    #[cfg(feature = "runtime_tests")]
    pub fn run(translations: *const os::Translation, translation_size: u64) {
        info!("Executing paging tests...");
        test_load_and_restore_page_tables(translations, translation_size);
        info!("Interrupt paging succeeded");
    }

    fn test_load_and_restore_page_tables(translations: *const os::Translation, translation_size: u64) {
        assert_eq!(paging::init(translations, translation_size), Ok(()));
        let mut pt: paging::PageTable = Default::default();
        let mut current_page_table = paging::CURRENT_PAGE_TABLE.write();
        assert_eq!(pt.map_hypervisor(translations, translation_size), Ok(()));
        unsafe {
            let original_pt = paging::CurrentPageTable::from_cpu();
            current_page_table.load(&mut pt);
            current_page_table.load(original_pt);
        }
    }
}
