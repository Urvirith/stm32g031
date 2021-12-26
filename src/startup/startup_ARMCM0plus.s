/**************************************************************************//**
 * @file     startup_ARMCM0plus.S
 * @brief    CMSIS-Core(M) Device Startup File for Cortex-M0plus Device
 * @version  V2.2.0
 * @date     26. May 2021
 ******************************************************************************/
/*
 * Copyright (c) 2009-2021 Arm Limited. All rights reserved.
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Licensed under the Apache License, Version 2.0 (the License); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an AS IS BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

                .syntax  unified
                .arch    armv6-m

                .section .vectors
                .align   2
                .globl   __Vectors
                .globl   __Vectors_End
                .globl   __Vectors_Size
__Vectors:
                .long    __StackTop                         /*     Top of Stack */
                .long    Reset_Handler                      /*     Reset Handler */
                .long    NMI_Handler                        /* -14 NMI Handler */
                .long    HardFault_Handler                  /* -13 Hard Fault Handler */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    SVC_Handler                        /*  -5 SVCall Handler */
                .long    0                                  /*     Reserved */
                .long    0                                  /*     Reserved */
                .long    PendSV_Handler                     /*  -2 PendSV Handler */
                .long    SysTick_Handler                    /*  -1 SysTick Handler */

                /* Interrupts */
                .long    WWDG_Handler 	                    /*  0  Window Watchdog interrupt */
                .long    PVD_Handler 	                    /*  1  Power voltage detector interrupt */
                .long    RTCTAMP_Handler 	                /*  2  RTC and TAMP interrupts */
                .long    FLASH_Handler 	                    /*  3  Flash global interrupt */
                .long    RCC_Handler 	                    /*  4  RCC global interrupt */
                .long    EXTI0_1_Handler 	                /*  5  EXTI Line0 - 1 interrupt */
                .long    EXTI2_3_Handler 	                /*  6  EXTI Line2  -3 interrupt */
                .long    EXTI4_15_Handler 	                /*  7  EXTI Line4 - 15 interrupt */
                .long    USB_Handler 	                    /*  8  UCPD and USB global interrupt */
                .long    DMA1Ch1_Handler 	                /*  9  DMA1 channel 1 interrupt */
                .long    DMA1Ch2Ch3_Handler 	            /* 10  DMA1 channel 2 & 3 interrupts */
                .long    DMA1Ch4Ch7_DMA2Ch1Ch5_Handler 	    /* 11  DMA1 channel 4, 5, 6, 7, DMAMUX, DMA2 channel 1, 2, 3, 4, 5 interrupts */
                .long    ADC_COMP_Handler 	                /* 12  ADC and COMP interrupts */
                .long    TIM1_BRK_UP_TRG_Handler 	        /* 13  TIM1 break, update, trigger and commutation interrupts */
                .long    TIM1_CC_Handler 	                /* 14  TIM1 Capture Compare interrupt  */
                .long    TIM2_Handler 	                    /* 15  TIM2 global interrupt */
                .long    TIM3_TIM4_Handler 	                /* 16  TIM3 global interrupt  */
                .long    TIM6_DAC_Handler 	                /* 17  TIM6, LPTIM1 and DAC global interrupt */
                .long    TIM7_Handler	                    /* 18  TIM7 and LPTIM2 global interrupt */
                .long    TIM14_Handler	                    /* 19  TIM14 global interrupt  */
                .long    TIM15_Handler	                    /* 20  TIM15 global interrupt */
                .long    TIM16_FDCAN0_Handler	            /* 21  TIM16 and FDCAN_IT0 global interrupt */
                .long    TIM17_FDCAN1_Handler	            /* 22  TIM17 and FDCAN_IT1 global interrupt */
                .long    I2C1_Handler	                    /* 23  I2C1 global interrupt */
                .long    I2C2_I2C3_Handler	                /* 24  I2C2 and I2C3 global interrupt */
                .long    SPI1_Handler	                    /* 25  SPI1 global interrupt  */
                .long    SPI2_SPI3_Handler	                /* 26  SPI2 SPI3 global interrupt */
                .long    USART1_Handler	                    /* 27  USART1 global interrupt  */
                .long    USART2_Handler	                    /* 28  USART2 and LPUART2 global interrupt */
                .long    USART_3_4_5_6_Handler              /* 29  USART3/4/5/6 and LPUART1 global interrupt */
                .long    CEC_Handler	                    /* 30  CEC global interrupt */
                .long    AES_RNG_Handler	                /* 31  AES and RNG global interrupts  */
                .long    0

__Vectors_End:
                .equ     __Vectors_Size, __Vectors_End - __Vectors
                .size    __Vectors, . - __Vectors


                .thumb
                .section .text
                .align   2

                .thumb_func
                .type    Reset_Handler, %function
                .globl   Reset_Handler
                .fnstart
Reset_Handler:
                bl       sys_init

                ldr      r4, =__copy_table_start__
                ldr      r5, =__copy_table_end__

.L_loop0:
                cmp      r4, r5
                bge      .L_loop0_done
                ldr      r1, [r4]                /* source address */
                ldr      r2, [r4, #4]            /* destination address */
                ldr      r3, [r4, #8]            /* word count */
                lsls     r3, r3, #2              /* byte count */

.L_loop0_0:
                subs     r3, #4                  /* decrement byte count */
                blt      .L_loop0_0_done
                ldr      r0, [r1, r3]
                str      r0, [r2, r3]
                b        .L_loop0_0

.L_loop0_0_done:
                adds     r4, #12
                b        .L_loop0

.L_loop0_done:

                ldr      r3, =__zero_table_start__
                ldr      r4, =__zero_table_end__

.L_loop2:
                cmp      r3, r4
                bge      .L_loop2_done
                ldr      r1, [r3]                /* destination address */
                ldr      r2, [r3, #4]            /* word count */
                lsls     r2, r2, #2              /* byte count */
                movs     r0, 0

.L_loop2_0:
                subs     r2, #4                  /* decrement byte count */
                blt      .L_loop2_0_done
                str      r0, [r1, r2]
                b        .L_loop2_0
.L_loop2_0_done:

                adds     r3, #8
                b        .L_loop2
.L_loop2_done:

                bl       start

                .fnend
                .size    Reset_Handler, . - Reset_Handler

/* The default macro is not used for HardFault_Handler
 * because this results in a poor debug illusion.
 */
                .thumb_func
                .type    HardFault_Handler, %function
                .weak    HardFault_Handler
                .fnstart
HardFault_Handler:
                b        .
                .fnend
                .size    HardFault_Handler, . - HardFault_Handler

                .thumb_func
                .type    Default_Handler, %function
                .weak    Default_Handler
                .fnstart
Default_Handler:
                b        .
                .fnend
                .size    Default_Handler, . - Default_Handler

/* Macro to define default exception/interrupt handlers.
 * Default handler are weak symbols with an endless loop.
 * They can be overwritten by real handlers.
 */
                .macro   Set_Default_Handler  Handler_Name
                .weak    \Handler_Name
                .set     \Handler_Name, Default_Handler
                .endm


/* Default exception/interrupt handler */

                Set_Default_Handler  NMI_Handler
                Set_Default_Handler  SVC_Handler
                Set_Default_Handler  PendSV_Handler
                Set_Default_Handler  SysTick_Handler

                Set_Default_Handler  WWDG_Handler	                    /*  0  Window Watchdog interrupt */
                Set_Default_Handler  PVD_Handler 	                    /*  1  Power voltage detector interrupt */
                Set_Default_Handler  RTCTAMP_Handler 	                /*  2  RTC and TAMP interrupts */
                Set_Default_Handler  FLASH_Handler 	                    /*  3  Flash global interrupt */
                Set_Default_Handler  RCC_Handler 	                    /*  4  RCC global interrupt */
                Set_Default_Handler  EXTI0_1_Handler 	                /*  5  EXTI Line0 - 1 interrupt */
                Set_Default_Handler  EXTI2_3_Handler 	                /*  6  EXTI Line2  -3 interrupt */
                Set_Default_Handler  EXTI4_15_Handler 	                /*  7  EXTI Line4 - 15 interrupt */
                Set_Default_Handler  USB_Handler 	                    /*  8  UCPD and USB global interrupt */
                Set_Default_Handler  DMA1Ch1_Handler 	                /*  9  DMA1 channel 1 interrupt */
                Set_Default_Handler  DMA1Ch2Ch3_Handler 	            /* 10  DMA1 channel 2 & 3 interrupts */
                Set_Default_Handler  DMA1Ch4Ch7_DMA2Ch1Ch5_Handler 	    /* 11  DMA1 channel 4, 5, 6, 7, DMAMUX, DMA2 channel 1, 2, 3, 4, 5 interrupts */
                Set_Default_Handler  ADC_COMP_Handler	                /* 12  ADC and COMP interrupts */
                Set_Default_Handler  TIM1_BRK_UP_TRG_Handler	        /* 13  TIM1 break, update, trigger and commutation interrupts */
                Set_Default_Handler  TIM1_CC_Handler	                /* 14  TIM1 Capture Compare interrupt  */
                Set_Default_Handler  TIM2_Handler	                    /* 15  TIM2 global interrupt */
                Set_Default_Handler  TIM3_TIM4_Handler	                /* 16  TIM3 global interrupt  */
                Set_Default_Handler  TIM6_DAC_Handler	                /* 17  TIM6, LPTIM1 and DAC global interrupt */
                Set_Default_Handler  TIM7_Handler	                    /* 18  TIM7 and LPTIM2 global interrupt */
                Set_Default_Handler  TIM14_Handler	                    /* 19  TIM14 global interrupt  */
                Set_Default_Handler  TIM15_Handler	                    /* 20  TIM15 global interrupt */
                Set_Default_Handler  TIM16_FDCAN0_Handler	            /* 21  TIM16 and FDCAN_IT0 global interrupt */
                Set_Default_Handler  TIM17_FDCAN1_Handler	            /* 22  TIM17 and FDCAN_IT1 global interrupt */
                Set_Default_Handler  I2C1_Handler	                    /* 23  I2C1 global interrupt */
                Set_Default_Handler  I2C2_I2C3_Handler	                /* 24  I2C2 and I2C3 global interrupt */
                Set_Default_Handler  SPI1_Handler	                    /* 25  SPI1 global interrupt  */
                Set_Default_Handler  SPI2_SPI3_Handler	                /* 26  SPI2 SPI3 global interrupt */
                Set_Default_Handler  USART1_Handler	                    /* 27  USART1 global interrupt  */
                Set_Default_Handler  USART2_Handler	                    /* 28  USART2 and LPUART2 global interrupt */
                Set_Default_Handler  USART_3_4_5_6_Handler	            /* 29  USART3/4/5/6 and LPUART1 global interrupt */
                Set_Default_Handler  CEC_Handler	                    /* 30  CEC global interrupt */
                Set_Default_Handler  AES_RNG_Handler	                /* 31  AES and RNG global interrupts  */


                .end
