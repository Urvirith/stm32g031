#![no_std]

use core::panic::PanicInfo;
mod stm32hal;
mod board;
mod led;

/* Set Clock In One Area */
const CLK:          stm32hal::common::MsiRange = stm32hal::common::MsiRange::Clk16MHz;


#[no_mangle]
pub extern "C" fn sys_init() {
    /* RCC Enabling of the bus */
    let rcc = board::rcc::Rcc::init(board::g031k8::RCC_BASE);

    rcc.write_iop_enr(board::g031k8::GPIOA_RCC_IOP_ENABLE);
    rcc.write_iop_enr(board::g031k8::GPIOB_RCC_IOP_ENABLE);
    rcc.write_iop_enr(board::g031k8::GPIOC_RCC_IOP_ENABLE);
    rcc.write_apb_enr1(board::g031k8::TIMER2_RCC_APBR1_ENABLE);
    rcc.write_apb_enr1(board::g031k8::TIMER3_RCC_APBR1_ENABLE);
}

#[no_mangle]
pub extern "C" fn start() {
    setup();
    let seq_timer   = stm32hal::timer::Timer::init(board::g031k8::TIMER2_BASE);
    let mut led = led::LEDManager::init();
    led.setup();

    loop {
        if seq_timer.get_flag() {
            seq_timer.clr_flag();
            led.switch();
        }
	}
}


#[no_mangle]
pub extern "C" fn TIM3_TIM4_Handler() {
    let gpioc       = stm32hal::gpio::Gpio::init(board::g031k8::GPIOC_BASE);
    let int_timer   = stm32hal::timer::Timer::init(board::g031k8::TIMER3_BASE);
    int_timer.clr_flag();

    if gpioc.get_pin(board::g031k8::USER_LED_BIT) {
        gpioc.clr_pin(board::g031k8::USER_LED_BIT);
    } else {
        gpioc.set_pin(board::g031k8::USER_LED_BIT);
    }

    int_timer.start();
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


// ALL MY PIN SETUPS AND CLOCKS
fn setup(){
    let freq = stm32hal::common::range(CLK);
    // Initialize the LED on L432KC board
    let gpioc       = stm32hal::gpio::Gpio::init(board::g031k8::GPIOC_BASE);
    let seq_timer   = stm32hal::timer::Timer::init(board::g031k8::TIMER2_BASE);
    let int_timer   = stm32hal::timer::Timer::init(board::g031k8::TIMER3_BASE);
    let mut nvic    = stm32hal::nvic::Nvic::init(board::g031k8::NVIC_BASE);

    gpioc.otype(board::g031k8::USER_LED, board::g031k8::LED_MODE, board::g031k8::LED_OTYPE, board::g031k8::LED_AF);

    seq_timer.open(stm32hal::timer::TimerType::Cont, stm32hal::timer::Direction::Upcount);
    seq_timer.set_scl(2000, freq, 1000);

    int_timer.open(stm32hal::timer::TimerType::Ons, stm32hal::timer::Direction::Upcount);
    int_timer.set_interrupt();
    int_timer.delay(1000, freq, 500);

    nvic.set_interrupt(board::g031k8::NvicIrq::TIM3TIM4Irq as u32);
    int_timer.set_scl(1000, freq, 500);
    seq_timer.start();
    int_timer.start();
}