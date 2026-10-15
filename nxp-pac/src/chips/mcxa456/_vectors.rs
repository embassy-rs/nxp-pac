unsafe extern "C" {
    fn CMC();
    fn DMA0_CH0();
    fn DMA0_CH1();
    fn DMA0_CH2();
    fn DMA0_CH3();
    fn DMA0_CH4();
    fn DMA0_CH5();
    fn DMA0_CH6();
    fn DMA0_CH7();
    fn ERM0_SINGLE_BIT();
    fn ERM0_MULTI_BIT();
    fn FMU0();
    fn GLIKEY0();
    fn MBC0();
    fn SCG0();
    fn SPC0();
    fn TDET();
    fn WUU0();
    fn CAN0();
    fn CAN1();
    fn FLEXIO();
    fn I3C0();
    fn I3C1();
    fn LPI2C0();
    fn LPI2C1();
    fn LPSPI0();
    fn LPSPI1();
    fn LPSPI2();
    fn LPUART0();
    fn LPUART1();
    fn LPUART2();
    fn LPUART3();
    fn LPUART4();
    fn CDOG0();
    fn CTIMER0();
    fn CTIMER1();
    fn CTIMER2();
    fn CTIMER3();
    fn CTIMER4();
    fn FREQME0();
    fn LPTMR0();
    fn OS_EVENT();
    fn WAKETIMER0();
    fn UTICK0();
    fn WWDT0();
    fn WWDT1();
    fn ADC0();
    fn ADC1();
    fn CMP0();
    fn DAC0();
    fn DAC1();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn GPIO5();
    fn LPI2C2();
    fn LPI2C3();
    fn I3C2();
    fn LPUART5();
    fn LPSPI3();
    fn LPSPI4();
    fn LPSPI5();
    fn LPI2C4();
    fn I3C3();
    fn USB1_HS();
    fn USB1_HS_PHY();
    fn FLEXSPI0();
    fn SMARTDMA();
    fn CDOG1();
    fn PKC();
    fn SGI();
    fn TRNG0();
    fn SECURE_ERR();
    fn SEC_HYPERVISOR_CALL();
    fn RTC0();
    fn GDET();
    fn EWM0();
    fn TSI_END_OF_SCAN();
    fn TSI_OUT_OF_SCAN();
    fn GPIO0_1();
    fn GPIO1_1();
    fn GPIO2_1();
    fn GPIO3_1();
    fn GPIO4_1();
    fn GPIO5_1();
    fn ITRC0();
    fn DMA0_CH8();
    fn DMA0_CH9();
    fn DMA0_CH10();
    fn DMA0_CH11();
    fn DMA1_CH0();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 146] = [
    Vector { _reserved: 0 },
    Vector { _handler: CMC },
    Vector { _handler: DMA0_CH0 },
    Vector { _handler: DMA0_CH1 },
    Vector { _handler: DMA0_CH2 },
    Vector { _handler: DMA0_CH3 },
    Vector { _handler: DMA0_CH4 },
    Vector { _handler: DMA0_CH5 },
    Vector { _handler: DMA0_CH6 },
    Vector { _handler: DMA0_CH7 },
    Vector {
        _handler: ERM0_SINGLE_BIT,
    },
    Vector {
        _handler: ERM0_MULTI_BIT,
    },
    Vector { _handler: FMU0 },
    Vector { _handler: GLIKEY0 },
    Vector { _handler: MBC0 },
    Vector { _handler: SCG0 },
    Vector { _handler: SPC0 },
    Vector { _handler: TDET },
    Vector { _handler: WUU0 },
    Vector { _handler: CAN0 },
    Vector { _handler: CAN1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FLEXIO },
    Vector { _handler: I3C0 },
    Vector { _handler: I3C1 },
    Vector { _handler: LPI2C0 },
    Vector { _handler: LPI2C1 },
    Vector { _handler: LPSPI0 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _handler: LPUART0 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPUART2 },
    Vector { _handler: LPUART3 },
    Vector { _handler: LPUART4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CDOG0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER3 },
    Vector { _handler: CTIMER4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FREQME0 },
    Vector { _handler: LPTMR0 },
    Vector { _reserved: 0 },
    Vector { _handler: OS_EVENT },
    Vector {
        _handler: WAKETIMER0,
    },
    Vector { _handler: UTICK0 },
    Vector { _handler: WWDT0 },
    Vector { _handler: WWDT1 },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: CMP0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DAC0 },
    Vector { _handler: DAC1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector { _handler: GPIO5 },
    Vector { _handler: LPI2C2 },
    Vector { _handler: LPI2C3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I3C2 },
    Vector { _handler: LPUART5 },
    Vector { _reserved: 0 },
    Vector { _handler: LPSPI3 },
    Vector { _handler: LPSPI4 },
    Vector { _handler: LPSPI5 },
    Vector { _handler: LPI2C4 },
    Vector { _handler: I3C3 },
    Vector { _reserved: 0 },
    Vector { _handler: USB1_HS },
    Vector {
        _handler: USB1_HS_PHY,
    },
    Vector { _reserved: 0 },
    Vector { _handler: FLEXSPI0 },
    Vector { _reserved: 0 },
    Vector { _handler: SMARTDMA },
    Vector { _handler: CDOG1 },
    Vector { _handler: PKC },
    Vector { _handler: SGI },
    Vector { _reserved: 0 },
    Vector { _handler: TRNG0 },
    Vector {
        _handler: SECURE_ERR,
    },
    Vector {
        _handler: SEC_HYPERVISOR_CALL,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GDET },
    Vector { _handler: EWM0 },
    Vector {
        _handler: TSI_END_OF_SCAN,
    },
    Vector {
        _handler: TSI_OUT_OF_SCAN,
    },
    Vector { _handler: GPIO0_1 },
    Vector { _handler: GPIO1_1 },
    Vector { _handler: GPIO2_1 },
    Vector { _handler: GPIO3_1 },
    Vector { _handler: GPIO4_1 },
    Vector { _handler: GPIO5_1 },
    Vector { _reserved: 0 },
    Vector { _handler: ITRC0 },
    Vector { _handler: DMA0_CH8 },
    Vector { _handler: DMA0_CH9 },
    Vector {
        _handler: DMA0_CH10,
    },
    Vector {
        _handler: DMA0_CH11,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA1_CH0 },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
];
