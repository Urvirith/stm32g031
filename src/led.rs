use super::stm32hal;
use super::board;

pub struct LEDManager {
    step:       usize,
    cmd:        bool,
    status:     bool
}

impl LEDManager{
    pub fn init() -> LEDManager {
        return LEDManager {
            step:   0,
            cmd:    false,
            status: false
        }
    }

    /* ADD MORE LEDS HERE - DEFINE THEM IN THE BOARD AS THEY ARE BOARD SPECIFIC PINS AND ADD THE INITIATIZATION HERE*/
    pub fn setup(&self) {
        let gpioa       = stm32hal::gpio::Gpio::init(board::g031k8::GPIOA_BASE);
        let gpiob       = stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE);

        gpiob.otype(board::g031k8::LED1, board::g031k8::LED_MODE, board::g031k8::LED_OTYPE, board::g031k8::LED_AF);
        gpiob.otype(board::g031k8::LED2, board::g031k8::LED_MODE, board::g031k8::LED_OTYPE, board::g031k8::LED_AF);
        gpiob.otype(board::g031k8::LED3, board::g031k8::LED_MODE, board::g031k8::LED_OTYPE, board::g031k8::LED_AF);
        gpioa.otype(board::g031k8::LED4, board::g031k8::LED_MODE, board::g031k8::LED_OTYPE, board::g031k8::LED_AF);
    }

    // &self is required to access local elements in the structure, LEDManager
    // & is required to borrow, you use self without & you consume the instance
    // use &mut self if you want to write to your elements
    pub fn switch(&mut self) {
        if self.step == 0 {
            self.led1_on();
            self.led2_off();
            self.led3_on();
            self.led4_off();
        } else {
            self.led1_off();
            self.led2_on();
            self.led3_off();
            self.led4_on();
        }

        self.step += 1;

        if self.step > 1 {
            self.step = 0;
        }
    }

    pub fn led1_on(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).set_pin(board::g031k8::LED1_BIT);
    }

    pub fn led1_off(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).clr_pin(board::g031k8::LED1_BIT);
    }

    pub fn led2_on(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).set_pin(board::g031k8::LED2_BIT);
    }

    pub fn led2_off(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).clr_pin(board::g031k8::LED2_BIT);
    }

    pub fn led3_on(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).set_pin(board::g031k8::LED3_BIT);
    }

    pub fn led3_off(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOB_BASE).clr_pin(board::g031k8::LED3_BIT);
    }

    pub fn led4_on(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOA_BASE).set_pin(board::g031k8::LED4_BIT);
    }

    pub fn led4_off(&self) {
        stm32hal::gpio::Gpio::init(board::g031k8::GPIOA_BASE).clr_pin(board::g031k8::LED4_BIT);
    }
}