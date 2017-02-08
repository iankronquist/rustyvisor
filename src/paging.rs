// Credit where it's due: this file draws heavily from Philip Opperman's
// wonderful tutorial: http://os.phil-opp.com/modifying-page-tables.html
// It's a really slick way to use Rust's type system to express the page
// tables.

use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::Unique;
use collections::boxed::Box;
use spin::RwLock;
use os;
use vmx;

const TABLE_ENTRY_COUNT: usize = 512;
const UNUSED_ENTRY: u64 = 0xcccccccc;


pub const PAGE_PRESENT: u64 = 1;
pub const PAGE_WRITABLE: u64 = 1 << 1;
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

pub struct PhysicalAddress(u64);
pub struct VirtualAddress(u64);

#[derive(Clone, Copy, Default)]
pub struct PageTableEntry(u64);


impl PageTableEntry {
    pub fn is_present(&self) -> bool {
        self.0 & PAGE_PRESENT == 0
    }

    pub fn set_absent(&mut self) {
        self.0 = UNUSED_ENTRY;
    }

    pub fn set(&mut self, flags: u64) {
        self.0 = flags;
    }

    fn physical_address(&self) -> Option<PhysicalAddress> {
        if (self.0 & PAGE_PRESENT) == 0 {
            Some(PhysicalAddress(self.0 & 0x000fffff_fffff000))
        } else {
            None
        }
    }
}

impl VirtualAddress {
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
        (self.0 & 0o777) as usize
    }

    fn mask(&self) -> u64 {
        self.0 & 0x000fffff_fffff000
    }
}


pub struct PageTable<L: PageTableLevel> {
    entries: [PageTableEntry; TABLE_ENTRY_COUNT],
    level: PhantomData<L>,
}


impl<L: PageTableLevel> Default for PageTable<L> {
    fn default() -> Self {
        info!("Making default page table");
        PageTable {
            entries: [Default::default(); TABLE_ENTRY_COUNT],
            level: PhantomData,
        }
    }
}


impl PageTable<Level4> {
    fn new() -> Self {
        info!("Making default");
        let mut pt4: Self = Default::default();
        info!("Setting fractal");
        pt4.set_fractal();
        info!("ret");
        pt4
    }
}

pub unsafe fn load_physical(pa: PhysicalAddress) {
    vmx::write_cr3(pa.0);
}



impl PageTable<Level4> {
    pub unsafe fn load(&mut self) {
        let pa = ACTIVE_PAGE_TABLE.write().virt_to_phys(VirtualAddress(self.entries.as_ptr() as u64)).expect("Fractal paging not working");
        load_physical(pa);
    }


    pub fn set_fractal(&mut self) {
        let pa = ACTIVE_PAGE_TABLE.write().virt_to_phys(VirtualAddress(self.entries.as_ptr() as u64))
            .expect("Page table unmapped?");
        self.entries[TABLE_ENTRY_COUNT - 1].set(pa.0);
    }



    pub unsafe fn from_cpu() -> PhysicalAddress {
        PhysicalAddress(vmx::read_cr3())
    }
}


pub fn invlpg(addr: u64) {
    unsafe {
        asm!("invlpg ($0)" ::"r" (addr) : "memory");
    }
}


pub const P4: *mut PageTable<Level4> = 0o177777_777_777_777_777_0000 as *mut _;


pub struct ActivePageTable {
    linear_offset: isize,
    p4: Unique<PageTable<Level4>>,
}

lazy_static! {
    static ref ACTIVE_PAGE_TABLE: RwLock<ActivePageTable> = {
        unsafe {
            RwLock::new(ActivePageTable { p4: Unique::new(P4), linear_offset: 0 })
        }
    };
}

impl ActivePageTable {
    pub fn p4(&self) -> &PageTable<Level4> {
        unsafe { self.p4.get() }
    }

    pub fn p4_mut(&mut self) -> &mut PageTable<Level4> {
        unsafe { self.p4.get_mut() }
    }

    pub fn init(&mut self, host_offset: isize) {
        self.linear_offset = host_offset;
    }


    pub fn virt_to_phys(&self, page: VirtualAddress) -> Option<PhysicalAddress> {

        let p3 = self.p4().next_layer(page.p4_index());
        info!("Recursing from p3");

        p3.and_then(|p3| p3.next_layer(page.p3_index()))
            .and_then(|p2| p2.next_layer(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].physical_address())
    }
}

impl<L: PageTableLevel> Index<usize> for PageTable<L> {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &PageTableEntry {
        &self.entries[index]
    }
}


impl<L: PageTableLevel> IndexMut<usize> for PageTable<L> {
    fn index_mut(&mut self, index: usize) -> &mut PageTableEntry {
        &mut self.entries[index]
    }
}


impl<L: PageTableLevel> PageTable<L> {
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_absent();
        }
    }
}


impl<L: HierarchicalLevel> PageTable<L> {
    fn next_layer_address(&self, index: usize) -> Option<usize> {
        let flags = &self[index];
        if ((flags.0 & PAGE_PRESENT) == 0) && !(flags.0 & PAGE_HUGE == 0) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    pub fn next_layer(&self, index: usize) -> Option<&PageTable<L::NextLevel>> {
        self.next_layer_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_layer_mut(&mut self, index: usize) -> Option<&mut PageTable<L::NextLevel>> {
        self.next_layer_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_layer_create(&mut self, index: usize) -> Option<&mut PageTable<L::NextLevel>> {
        if self.next_layer(index).is_none() {
            assert!((self.entries[index].0 & PAGE_HUGE) == 0);
            let address = PageTable::<L>::allocate_page();
            self.entries[index].set(address.mask() | PAGE_PRESENT | PAGE_WRITABLE);
            return self.next_layer_mut(index);
        }
        self.next_layer_mut(index)
    }

    pub fn allocate_page() -> VirtualAddress {
        let page: Box<PageTable<L>> = Box::new(Default::default());
        VirtualAddress(Box::into_raw(page) as u64)
    }
}


pub fn map_page(virt: VirtualAddress, phys: PhysicalAddress, flags: u64) -> Result<(), ()> {
    let mut apt = ACTIVE_PAGE_TABLE.write();
    let p1 = apt.p4_mut()
        .next_layer_create(virt.p4_index())
        .and_then(|mut p3| p3.next_layer_create(virt.p3_index()))
        .and_then(|mut p2| p2.next_layer_create(virt.p2_index()));
    match p1 {
        Some(entry) => {
            assert!(entry[virt.p1_index()].is_present());
            entry[virt.p1_index()].set(phys.0 | flags | PAGE_PRESENT);
            invlpg(virt.0);
            Ok(())
        }
        None => Err(()),
    }
}

pub fn map_hypervisor(translations: *const os::Translation, translation_size: u64) -> Result<(), ()> {
    for i in 0..translation_size {
        info!("mapping translation {}", i);
        let result: Result<(), ()>;
        unsafe {
            info!("{} -> {}", (*translations.offset(i as isize)).virt,
                              (*translations.offset(i as isize)).phys);
            result = map_page(VirtualAddress((*translations.offset(i as isize)).virt),
                PhysicalAddress((*translations.offset(i as isize)).phys),
                PAGE_WRITABLE);
        }
        match result {
            Err(()) => return Err(()),
            _ => continue,
        }
    }
    Ok(())
}

pub fn get_host_linear_offset(translations: *const os::Translation, translation_size: u64) -> isize {
    assert!(translation_size > 0);
    unsafe {
        ((*translations).virt - (*translations).phys) as isize
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {


    use super::*;

    #[cfg(feature = "runtime_tests")]
    pub fn run(translations: *const os::Translation, translation_size: u64) {
        info!("Executing paging tests...");
        test_load_and_restore_page_tables(translations, translation_size);
        info!("Paging tests succeeded");
    }

    fn test_load_and_restore_page_tables(translations: *const os::Translation, translation_size: u64) {
        info!("Making new page table");
        let mut new = PageTable::new();
        info!("Mapping hypervisor");
        let result = map_hypervisor(translations, translation_size);
        assert_eq!(result, Ok(()));

        unsafe {
            info!("Getting original page table");
            let original = PageTable::from_cpu();
            info!("Loading new page table");
            new.load();
            info!("Reloading original");
            load_physical(original);
        }
    }
}
