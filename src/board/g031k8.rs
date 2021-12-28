use crate::stm32hal::{common, gpio, usart};

/* Register Base */
/* Reset and Clock Control (RCC) */
pub const RCC_BASE:                 u32 = 0x40021000;

/* General Purpose I/O */
pub const GPIOA_BASE:               u32 = 0x50000000;
pub const GPIOB_BASE:               u32 = 0x50000400;
pub const GPIOC_BASE:               u32 = 0x50000800;
pub const GPIOD_BASE:               u32 = 0x50000C00;
pub const GPIOE_BASE:               u32 = 0x50001000;
pub const GPIOF_BASE:               u32 = 0x50001400;

/* Timers */
pub const TIMER1_BASE:              u32 = 0x40012C00;
pub const TIMER2_BASE:              u32 = 0x40000000;
pub const TIMER3_BASE:              u32 = 0x40000400;
pub const TIMER4_BASE:              u32 = 0x40000800;
pub const TIMER6_BASE:              u32 = 0x40001000;
pub const TIMER7_BASE:              u32 = 0x40001400;
pub const TIMER14_BASE:             u32 = 0x40002000;
pub const TIMER15_BASE:             u32 = 0x40014000;
pub const TIMER16_BASE:             u32 = 0x40014400;
pub const TIMER17_BASE:             u32 = 0x40014800;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART1_BASE:              u32 = 0x40013800;
pub const USART2_BASE:              u32 = 0x40004400;
pub const USART3_BASE:              u32 = 0x40004800;
pub const USART4_BASE:              u32 = 0x40004C00;
pub const USART5_BASE:              u32 = 0x40005000;
pub const USART6_BASE:              u32 = 0x40013C00;

/* Inter-Integrated Circuit (I2C) */
pub const I2C1_BASE:                u32 = 0x40005400;
pub const I2C2_BASE:                u32 = 0x40005800;
pub const I2C3_BASE:                u32 = 0x40008800;

/* Serial Peripheral Interface */
pub const SPI1_BASE:                u32 = 0x40013000;
pub const SPI2_BASE:                u32 = 0x40003800;
pub const SPI3_BASE:                u32 = 0x40003C00;

/* Controller Area Networl */
pub const FDCAN1_BASE:              u32 = 0x40006400;
pub const FDCAN2_BASE:              u32 = 0x40006800;
pub const FDCAN_RAM_BASE:           u32 = 0x4000B400;
/* NVIC */
pub const NVIC_BASE:                u32 = 0xE000E100;


/* Control Bits */
/* Reset and Clock Control (RCC) */
pub const GPIOA_RCC_IOP_ENABLE:    u32 = common::BIT_0;
pub const GPIOB_RCC_IOP_ENABLE:    u32 = common::BIT_1;
pub const GPIOC_RCC_IOP_ENABLE:    u32 = common::BIT_2;

/* General Purpose I/O */
pub const USER_LED:                 u32 = 6;
pub const USER_LED_BIT:             u32 = common::BIT_6;

/* LED OUTPUTS */
pub const PORTB_PIN4:               u32 = 4;                    // D12
pub const LED1:                     u32 = PORTB_PIN4;           // D12
pub const LED1_BIT:                 u32 = common::BIT_4;        // D12

/* LED OUTPUTS */
pub const PORTB_PIN5:               u32 = 5;                    // D11
pub const LED2:                     u32 = PORTB_PIN5;           // D11
pub const LED2_BIT:                 u32 = common::BIT_5;        // D11

/* LED OUTPUTS */
pub const PORTB_PIN9:               u32 = 9;                    // D10
pub const LED3:                     u32 = PORTB_PIN9;           // D10
pub const LED3_BIT:                 u32 = common::BIT_9;        // D10

/* LED OUTPUTS */
pub const PORTA_PIN8:               u32 = 8;                    // D09
pub const LED4:                     u32 = PORTA_PIN8;           // D09
pub const LED4_BIT:                 u32 = common::BIT_8;        // D09

/* GPIO SETUP */
pub const LED_MODE:                 gpio::Mode = gpio::Mode::Out;
pub const LED_OTYPE:                gpio::OType = gpio::OType::PushPull;
pub const LED_AF:                   gpio::AltFunc = gpio::AltFunc::Af0;

/* Timer */
pub const TIMER2_RCC_APBR1_ENABLE: u32 = common::BIT_0;
pub const TIMER3_RCC_APBR1_ENABLE: u32 = common::BIT_1;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
//pub const USART2_RCC_APB1R1_ENABLE: u32 = common::BIT_17;
//pub const PORTA_PIN2:               u32 = 2;    //A7    TX
//pub const PORTA_PIN3:               u32 = 3;    //A2    RX
//pub const USART2_TX:                u32 = PORTA_PIN2;
//pub const USART2_RX:                u32 = PORTA_PIN3;

/* GPIO SETUP */
pub const USART_MODE:               gpio::Mode = gpio::Mode::Alt;
pub const USART_OTYPE:              gpio::OType = gpio::OType::PushPull;
pub const USART_AF:                 gpio::AltFunc = gpio::AltFunc::Af7;

/* I2C 1*/
pub const I2C1_RCC_APB1R1_ENABLE:   u32 = common::BIT_21;
pub const PORTB_PIN6:               u32 = 6;    //D5    SCL
pub const PORTB_PIN7:               u32 = 7;    //D4    SDA
pub const I2C1_SCL:                 u32 = PORTB_PIN6;
pub const I2C1_SDA:                 u32 = PORTB_PIN7;

/* CAN */
pub const CAN_RCC_APB1R1_ENABLE:    u32 = common::BIT_25;
pub const PORTA_PIN11:              u32 = 11;   //D10   RX
pub const PORTA_PIN12:              u32 = 12;   //D2    TX
pub const CAN_RX:                   u32 = PORTA_PIN11;
pub const CAN_TX:                   u32 = PORTA_PIN12;

/* GPIO SETUP */
pub const CAN_MODE:                 gpio::Mode = gpio::Mode::Alt;
pub const CAN_OTYPE:                gpio::OType = gpio::OType::PushPull;
pub const CAN_AF:                   gpio::AltFunc = gpio::AltFunc::Af9;
pub const CAN_OSPEED:               gpio::OSpeed = gpio::OSpeed::High;
pub const CAN_PUPD:                 gpio::Pupd = gpio::Pupd::Pu;

/* SPI 1*/
/* RCC */
pub const SPI1_RCC_APB2R_ENABLE:    u32 = common::BIT_12;

/* SPI 2*/
/* RCC */
// pub const SPI2_RCC_APB1R1_ENABLE:   u32 = common::BIT_14; // NOT AVAILABLE 432KC

/* SPI 3*/
/* RCC */
pub const SPI3_RCC_APB1R1_ENABLE:   u32 = common::BIT_15;

/* NVIC Enumerated Set Of All Interrupts On STM32L4 controllers */
pub enum NvicIrq {
    WWDGIrq,	                    /*  0  Window Watchdog interrupt */
    PVDIrq,	                        /*  1  Power voltage detector interrupt */
    RTCTAMPIrq,	                    /*  2  RTC and TAMP interrupts */
    FLASHIrq,	                    /*  3  Flash global interrupt */
    RCCIrq,	                        /*  4  RCC global interrupt */
    EXTI01Irq,	                    /*  5  EXTI Line0 - 1 interrupt */
    EXTI23Irq,	                    /*  6  EXTI Line2  -3 interrupt */
    EXTI415Irq,	                    /*  7  EXTI Line4 - 15 interrupt */
    USBIrq,	                        /*  8  UCPD and USB global interrupt */
    DMA1Ch1Irq,	                    /*  9  DMA1 channel 1 interrupt */
    DMA1Ch2Ch3Irq,	                /* 10  DMA1 channel 2 & 3 interrupts */
    DMA1Ch4Ch7DMA2Ch1Ch5Irq,	    /* 11  DMA1 channel 4, 5, 6, 7, DMAMUX, DMA2 channel 1, 2, 3, 4, 5 interrupts */
    ADCCOMPIrq,	                    /* 12  ADC and COMP interrupts */
    TIM1BRKUPTRGCOMIrq,	            /* 13  TIM1 break, update, trigger and commutation interrupts */
    TIM1CCIrq,	                    /* 14  TIM1 Capture Compare interrupt  */
    TIM2Irq,	                    /* 15  TIM2 global interrupt */
    TIM3TIM4Irq,	                /* 16  TIM3 global interrupt  */
    TIM6DACIrq,	                    /* 17  TIM6, LPTIM1 and DAC global interrupt */
    TIM7Irq,	                    /* 18  TIM7 and LPTIM2 global interrupt */
    TIM14Irq,	                    /* 19  TIM14 global interrupt  */
    TIM15Irq,	                    /* 20  TIM15 global interrupt */
    TIM16FDCAN0Irq,	                /* 21  TIM16 and FDCAN_IT0 global interrupt */
    TIM17FDCAN1Irq,	                /* 22  TIM17 and FDCAN_IT1 global interrupt */
    I2C1Irq,	                    /* 23  I2C1 global interrupt */
    I2C2I2C3Irq,	                /* 24  I2C2 and I2C3 global interrupt */
    SPI1Irq,	                    /* 25  SPI1 global interrupt  */
    SPI2SPI3Irq,	                /* 26  SPI2 SPI3 global interrupt */
    USART1Irq,	                    /* 27  USART1 global interrupt  */
    USART2Irq,	                    /* 28  USART2 and LPUART2 global interrupt */
    USART3456Irq,	                /* 29  USART3/4/5/6 and LPUART1 global interrupt */
    CECIrq,	                        /* 30  CEC global interrupt */
    AESRNGIrq,	                    /* 31  AES and RNG global interrupts  */
}
