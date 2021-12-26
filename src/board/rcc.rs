/* Reset and Clock Control (RCC) */
/* Manual Page 195 */

use crate::stm32hal::pointer;

pub struct Rcc {
    cr:             *mut u32,       // Clock Control Register
    icscr:          *mut u32,       // Internal Clock Sources Calibration Register
    cfgr:           *mut u32,       // Clock Configuration Register 
    pll_cfgr:       *mut u32,       // PLL Configuration Register
    crrcr:          *mut u32,       // Clock Recovery RC Register
    cier:           *mut u32,       // Clock Interrupt Enable Register
    cifr:           *mut u32,       // Clock Interrupt Flag Status Register
    cicr:           *mut u32,       // Clock Interrupt Clear Register
    iop_rstr:       *mut u32,       // I/O Port Reset Register
    ahb_rstr:       *mut u32,       // AHB Peripheral Reset Register
    apb_rstr1:      *mut u32,       // APB Peripheral Reset Register 1
    apb_rstr2:      *mut u32,       // APB Peripheral Reset Register 2
    iop_enr:        *mut u32,       // I/O Port Enable Register
    ahb_enr:        *mut u32,       // AHB Peripheral Enable Register
    apb_enr1:       *mut u32,       // APB Peripheral Enable Register 1
    apb_enr2:       *mut u32,       // APB Peripheral Enable Register 2
    iop_sm_enr:     *mut u32,       // I/O Port Sleep And Stop Modes Enable Register
    ahb_sm_enr:     *mut u32,       // AHB Peripheral Sleep And Stop Modes Enable Register
    apb_sm_enr1:    *mut u32,       // APB Peripheral Sleep And Stop Modes Enable Register 1
    apb_sm_enr2:    *mut u32,       // APB Peripheral Sleep And Stop Modes Enable Register 2
    ccipr1:         *mut u32,       // Peripherals Independent Clock Configuration Register 1
    ccipr2:         *mut u32,       // Peripherals Independent Clock Configuration Register 2
    bdcr:           *mut u32,       // Backup Domain Control Register
    csr:            *mut u32,       // Control Status Register
}

/* Register Offset */
const CR:               u32 = 0x00;
const ICSCR:            u32 = 0x04;
const CFGR:             u32 = 0x08;
const PLL_CFGR:         u32 = 0x0C;
const CRRCR:            u32 = 0x14;
const CIER:             u32 = 0x18;
const CIFR:             u32 = 0x1C;
const CICR:             u32 = 0x20;
const IOP_RSTR:         u32 = 0x24;
const AHB_RSTR:         u32 = 0x28;
const APB_RSTR1:        u32 = 0x2C;
const APB_RSTR2:        u32 = 0x30;
const IOP_ENR:          u32 = 0x34;
const AHB_ENR:          u32 = 0x38;
const APB_ENR1:         u32 = 0x3C;
const APB_ENR2:         u32 = 0x40;
const IOP_SM_ENR:       u32 = 0x44;
const AHB_SM_ENR:       u32 = 0x48;
const APB_SM_ENR1:      u32 = 0x4C;
const APB_SM_ENR2:      u32 = 0x50;
const CCIPR1:           u32 = 0x54;
const CCIPR2:           u32 = 0x58;
const BDCR:             u32 = 0x5C;
const CSR:              u32 = 0x60;

impl Rcc {
    /* Initialize The Structure */
    pub fn init(base: u32) -> Rcc {
        return Rcc {
            cr:             (base + CR)             as *mut u32,
            icscr:          (base + ICSCR)          as *mut u32,
            cfgr:           (base + CFGR)           as *mut u32,
            pll_cfgr:       (base + PLL_CFGR)       as *mut u32,
            crrcr:          (base + CRRCR)          as *mut u32,
            cier:           (base + CIER)           as *mut u32,
            cifr:           (base + CIFR)           as *mut u32,
            cicr:           (base + CICR)           as *mut u32,
            iop_rstr:       (base + IOP_RSTR)       as *mut u32,
            ahb_rstr:       (base + AHB_RSTR)       as *mut u32,
            apb_rstr1:      (base + APB_RSTR1)      as *mut u32,
            apb_rstr2:      (base + APB_RSTR2)      as *mut u32,
            iop_enr:        (base + IOP_ENR)        as *mut u32,
            ahb_enr:        (base + AHB_ENR)        as *mut u32,
            apb_enr1:       (base + APB_ENR1)       as *mut u32,
            apb_enr2:       (base + APB_ENR2)       as *mut u32,
            iop_sm_enr:     (base + IOP_SM_ENR)     as *mut u32,
            ahb_sm_enr:     (base + AHB_SM_ENR)     as *mut u32,
            apb_sm_enr1:    (base + APB_SM_ENR1)    as *mut u32,
            apb_sm_enr2:    (base + APB_SM_ENR2)    as *mut u32,
            ccipr1:         (base + CCIPR1)         as *mut u32,
            ccipr2:         (base + CCIPR2)         as *mut u32,
            bdcr:           (base + BDCR)           as *mut u32,
            csr:            (base + CSR)            as *mut u32,
        };
    }

    pub fn write_iop_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.iop_enr, val);
    }
    
    pub fn write_ahb_enr(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.ahb_enr, val);
    }
    
    pub fn write_apb_enr1(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.apb_enr1, val);
    }
    
    pub fn write_apb_enr2(&self, val: u32) {
        pointer::set_ptr_vol_bit_u32(self.apb_enr2, val);
    }
}
