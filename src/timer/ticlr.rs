use bit_field::BitField;
use crate::impl_read_csr;
use crate::impl_write_csr;
use crate::impl_define_csr;

impl_define_csr!(TIClr,"Timer Interrupt Clearing \n\
                        The software clears the timed interrupt signal set by the timer by writing 1 to bit 0 of this register.");
impl_read_csr!(0x44, TIClr);
impl_write_csr!(0x44, TIClr);
impl TIClr {
    /// Clear the timed interrupt signal.
    pub fn clear_timer(&mut self) -> &mut Self {
        self.bits.set_bit(0, true);
        self
    }
}
