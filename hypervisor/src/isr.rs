//! This module defines various interrupt service routines, or ISRs.
//! An interrupt service routine is called via the hardware when an interrupt
//! occurs.
//! These ISRs must be installed into the Interrupt Descriptor Table.
//! This module mostly names functions defined elsewhere in assembly.

/// The type of an Interrupt Service Routine assembly stub.
/// They are modeled as a no-return extern "C" function.
/// However, they have an unusual calling convention determined by the
/// hardware, and are not meant to be called directly by rust code.
/// Instead, generate the appropriate interrupt.
pub type InterruptServiceRoutine = unsafe extern "C" fn() -> !;

extern "C" {
    /// ISR 0, division by 0.
    fn _isr0() -> !;
    /// ISR 1, debug exception.
    fn _isr1() -> !;
    /// ISR 2, Non-maskabile interrupt.
    fn _isr2() -> !;
    /// ISR 3, breakpoint.
    fn _isr3() -> !;
    /// ISR 4, overflow.
    fn _isr4() -> !;
    /// ISR 5, bound range exceeded.
    fn _isr5() -> !;
    /// ISR 6, invalid opcode.
    fn _isr6() -> !;
    /// ISR 7, device unavailable.
    fn _isr7() -> !;
    /// ISR 8, double fault.
    fn _isr8() -> !;
    /// ISR 9, coprocessor segment overrun (deprecated).
    fn _isr9() -> !;
    /// ISR 10, invalid TSS.
    fn _isr10() -> !;
    /// ISR 11, segment not present.
    fn _isr11() -> !;
    /// ISR 12, stack segment fault.
    fn _isr12() -> !;
    /// ISR 13, general protection fault.
    fn _isr13() -> !;
    /// ISR 14, page fault.
    fn _isr14() -> !;
    /// ISR 15, reserved.
    fn _isr15() -> !;
    /// ISR 16, x87 floating point exception.
    fn _isr16() -> !;
    /// ISR 17, alignment check.
    fn _isr17() -> !;
    /// ISR 18, machine check.
    fn _isr18() -> !;
    /// ISR 19, simd floating point exception.
    fn _isr19() -> !;
    /// ISR 20, virtualization exception.
    fn _isr20() -> !;
    /// ISR 21, reserved.
    fn _isr21() -> !;
    /// ISR 22, reserved.
    fn _isr22() -> !;
    /// ISR 23, reserved.
    fn _isr23() -> !;
    /// ISR 24, reserved.
    fn _isr24() -> !;
    /// ISR 25, reserved.
    fn _isr25() -> !;
    /// ISR 26, reserved.
    fn _isr26() -> !;
    /// ISR 27, reserved.
    fn _isr27() -> !;
    /// ISR 28, reserved.
    fn _isr28() -> !;
    /// ISR 29, reserved.
    fn _isr29() -> !;
    /// ISR 30, security exception.
    fn _isr30() -> !;
    /// ISR 31, reserved.
    fn _isr31() -> !;
    fn _isr32() -> !;
    fn _isr33() -> !;
    fn _isr34() -> !;
    fn _isr35() -> !;
    fn _isr36() -> !;
    fn _isr37() -> !;
    fn _isr38() -> !;
    fn _isr39() -> !;
    fn _isr40() -> !;
    fn _isr41() -> !;
    fn _isr42() -> !;
    fn _isr43() -> !;
    fn _isr44() -> !;
    fn _isr45() -> !;
    fn _isr46() -> !;
    fn _isr47() -> !;
    fn _isr48() -> !;
    fn _isr49() -> !;
    fn _isr50() -> !;
    fn _isr51() -> !;
    fn _isr52() -> !;
    fn _isr53() -> !;
    fn _isr54() -> !;
    fn _isr55() -> !;
    fn _isr56() -> !;
    fn _isr57() -> !;
    fn _isr58() -> !;
    fn _isr59() -> !;
    fn _isr60() -> !;
    fn _isr61() -> !;
    fn _isr62() -> !;
    fn _isr63() -> !;
    fn _isr64() -> !;
    fn _isr65() -> !;
    fn _isr66() -> !;
    fn _isr67() -> !;
    fn _isr68() -> !;
    fn _isr69() -> !;
    fn _isr70() -> !;
    fn _isr71() -> !;
    fn _isr72() -> !;
    fn _isr73() -> !;
    fn _isr74() -> !;
    fn _isr75() -> !;
    fn _isr76() -> !;
    fn _isr77() -> !;
    fn _isr78() -> !;
    fn _isr79() -> !;
    fn _isr80() -> !;
    fn _isr81() -> !;
    fn _isr82() -> !;
    fn _isr83() -> !;
    fn _isr84() -> !;
    fn _isr85() -> !;
    fn _isr86() -> !;
    fn _isr87() -> !;
    fn _isr88() -> !;
    fn _isr89() -> !;
    fn _isr90() -> !;
    fn _isr91() -> !;
    fn _isr92() -> !;
    fn _isr93() -> !;
    fn _isr94() -> !;
    fn _isr95() -> !;
    fn _isr96() -> !;
    fn _isr97() -> !;
    fn _isr98() -> !;
    fn _isr99() -> !;
    fn _isr100() -> !;
    fn _isr101() -> !;
    fn _isr102() -> !;
    fn _isr103() -> !;
    fn _isr104() -> !;
    fn _isr105() -> !;
    fn _isr106() -> !;
    fn _isr107() -> !;
    fn _isr108() -> !;
    fn _isr109() -> !;
    fn _isr110() -> !;
    fn _isr111() -> !;
    fn _isr112() -> !;
    fn _isr113() -> !;
    fn _isr114() -> !;
    fn _isr115() -> !;
    fn _isr116() -> !;
    fn _isr117() -> !;
    fn _isr118() -> !;
    fn _isr119() -> !;
    fn _isr120() -> !;
    fn _isr121() -> !;
    fn _isr122() -> !;
    fn _isr123() -> !;
    fn _isr124() -> !;
    fn _isr125() -> !;
    fn _isr126() -> !;
    fn _isr127() -> !;
    fn _isr128() -> !;
    fn _isr129() -> !;
    fn _isr130() -> !;
    fn _isr131() -> !;
    fn _isr132() -> !;
    fn _isr133() -> !;
    fn _isr134() -> !;
    fn _isr135() -> !;
    fn _isr136() -> !;
    fn _isr137() -> !;
    fn _isr138() -> !;
    fn _isr139() -> !;
    fn _isr140() -> !;
    fn _isr141() -> !;
    fn _isr142() -> !;
    fn _isr143() -> !;
    fn _isr144() -> !;
    fn _isr145() -> !;
    fn _isr146() -> !;
    fn _isr147() -> !;
    fn _isr148() -> !;
    fn _isr149() -> !;
    fn _isr150() -> !;
    fn _isr151() -> !;
    fn _isr152() -> !;
    fn _isr153() -> !;
    fn _isr154() -> !;
    fn _isr155() -> !;
    fn _isr156() -> !;
    fn _isr157() -> !;
    fn _isr158() -> !;
    fn _isr159() -> !;
    fn _isr160() -> !;
    fn _isr161() -> !;
    fn _isr162() -> !;
    fn _isr163() -> !;
    fn _isr164() -> !;
    fn _isr165() -> !;
    fn _isr166() -> !;
    fn _isr167() -> !;
    fn _isr168() -> !;
    fn _isr169() -> !;
    fn _isr170() -> !;
    fn _isr171() -> !;
    fn _isr172() -> !;
    fn _isr173() -> !;
    fn _isr174() -> !;
    fn _isr175() -> !;
    fn _isr176() -> !;
    fn _isr177() -> !;
    fn _isr178() -> !;
    fn _isr179() -> !;
    fn _isr180() -> !;
    fn _isr181() -> !;
    fn _isr182() -> !;
    fn _isr183() -> !;
    fn _isr184() -> !;
    fn _isr185() -> !;
    fn _isr186() -> !;
    fn _isr187() -> !;
    fn _isr188() -> !;
    fn _isr189() -> !;
    fn _isr190() -> !;
    fn _isr191() -> !;
    fn _isr192() -> !;
    fn _isr193() -> !;
    fn _isr194() -> !;
    fn _isr195() -> !;
    fn _isr196() -> !;
    fn _isr197() -> !;
    fn _isr198() -> !;
    fn _isr199() -> !;
    fn _isr200() -> !;
    fn _isr201() -> !;
    fn _isr202() -> !;
    fn _isr203() -> !;
    fn _isr204() -> !;
    fn _isr205() -> !;
    fn _isr206() -> !;
    fn _isr207() -> !;
    fn _isr208() -> !;
    fn _isr209() -> !;
    fn _isr210() -> !;
    fn _isr211() -> !;
    fn _isr212() -> !;
    fn _isr213() -> !;
    fn _isr214() -> !;
    fn _isr215() -> !;
    fn _isr216() -> !;
    fn _isr217() -> !;
    fn _isr218() -> !;
    fn _isr219() -> !;
    fn _isr220() -> !;
    fn _isr221() -> !;
    fn _isr222() -> !;
    fn _isr223() -> !;
    fn _isr224() -> !;
    fn _isr225() -> !;
    fn _isr226() -> !;
    fn _isr227() -> !;
    fn _isr228() -> !;
    fn _isr229() -> !;
    fn _isr230() -> !;
    fn _isr231() -> !;
    fn _isr232() -> !;
    fn _isr233() -> !;
    fn _isr234() -> !;
    fn _isr235() -> !;
    fn _isr236() -> !;
    fn _isr237() -> !;
    fn _isr238() -> !;
    fn _isr239() -> !;
    fn _isr240() -> !;
    fn _isr241() -> !;
    fn _isr242() -> !;
    fn _isr243() -> !;
    fn _isr244() -> !;
    fn _isr245() -> !;
    fn _isr246() -> !;
    fn _isr247() -> !;
    fn _isr248() -> !;
    fn _isr249() -> !;
    fn _isr250() -> !;
    fn _isr251() -> !;
    fn _isr252() -> !;
    fn _isr253() -> !;
    fn _isr254() -> !;
    fn _isr255() -> !;
}

/// This table is used to iterate over the assocated ISRs when installing the Interrupt Descriptor Table.
pub const ISR: [InterruptServiceRoutine; 256] = [
    _isr0, _isr1, _isr2, _isr3, _isr4, _isr5, _isr6, _isr7, _isr8, _isr9, _isr10, _isr11, _isr12,
    _isr13, _isr14, _isr15, _isr16, _isr17, _isr18, _isr19, _isr20, _isr21, _isr22, _isr23, _isr24,
    _isr25, _isr26, _isr27, _isr28, _isr29, _isr30, _isr31, _isr32, _isr33, _isr34, _isr35, _isr36,
    _isr37, _isr38, _isr39, _isr40, _isr41, _isr42, _isr43, _isr44, _isr45, _isr46, _isr47, _isr48,
    _isr49, _isr50, _isr51, _isr52, _isr53, _isr54, _isr55, _isr56, _isr57, _isr58, _isr59, _isr60,
    _isr61, _isr62, _isr63, _isr64, _isr65, _isr66, _isr67, _isr68, _isr69, _isr70, _isr71, _isr72,
    _isr73, _isr74, _isr75, _isr76, _isr77, _isr78, _isr79, _isr80, _isr81, _isr82, _isr83, _isr84,
    _isr85, _isr86, _isr87, _isr88, _isr89, _isr90, _isr91, _isr92, _isr93, _isr94, _isr95, _isr96,
    _isr97, _isr98, _isr99, _isr100, _isr101, _isr102, _isr103, _isr104, _isr105, _isr106, _isr107,
    _isr108, _isr109, _isr110, _isr111, _isr112, _isr113, _isr114, _isr115, _isr116, _isr117,
    _isr118, _isr119, _isr120, _isr121, _isr122, _isr123, _isr124, _isr125, _isr126, _isr127,
    _isr128, _isr129, _isr130, _isr131, _isr132, _isr133, _isr134, _isr135, _isr136, _isr137,
    _isr138, _isr139, _isr140, _isr141, _isr142, _isr143, _isr144, _isr145, _isr146, _isr147,
    _isr148, _isr149, _isr150, _isr151, _isr152, _isr153, _isr154, _isr155, _isr156, _isr157,
    _isr158, _isr159, _isr160, _isr161, _isr162, _isr163, _isr164, _isr165, _isr166, _isr167,
    _isr168, _isr169, _isr170, _isr171, _isr172, _isr173, _isr174, _isr175, _isr176, _isr177,
    _isr178, _isr179, _isr180, _isr181, _isr182, _isr183, _isr184, _isr185, _isr186, _isr187,
    _isr188, _isr189, _isr190, _isr191, _isr192, _isr193, _isr194, _isr195, _isr196, _isr197,
    _isr198, _isr199, _isr200, _isr201, _isr202, _isr203, _isr204, _isr205, _isr206, _isr207,
    _isr208, _isr209, _isr210, _isr211, _isr212, _isr213, _isr214, _isr215, _isr216, _isr217,
    _isr218, _isr219, _isr220, _isr221, _isr222, _isr223, _isr224, _isr225, _isr226, _isr227,
    _isr228, _isr229, _isr230, _isr231, _isr232, _isr233, _isr234, _isr235, _isr236, _isr237,
    _isr238, _isr239, _isr240, _isr241, _isr242, _isr243, _isr244, _isr245, _isr246, _isr247,
    _isr248, _isr249, _isr250, _isr251, _isr252, _isr253, _isr254, _isr255,
];
