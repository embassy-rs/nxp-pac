#[doc = "INPUTMUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux0 {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux0 {}
unsafe impl Sync for Inputmux0 {}
impl Inputmux0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Inputmux Register for SCT0 Input."]
    #[inline(always)]
    pub const fn sct0_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sct0Inmux, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer0cap0(self) -> crate::common::Reg<regs::Ctimer0cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer0cap1(self) -> crate::common::Reg<regs::Ctimer0cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer0cap2(self) -> crate::common::Reg<regs::Ctimer0cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer0cap3(self) -> crate::common::Reg<regs::Ctimer0cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::common::Reg<regs::Timer0trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer1cap0(self) -> crate::common::Reg<regs::Ctimer1cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer1cap1(self) -> crate::common::Reg<regs::Ctimer1cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer1cap2(self) -> crate::common::Reg<regs::Ctimer1cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer1cap3(self) -> crate::common::Reg<regs::Ctimer1cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::common::Reg<regs::Timer1trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer2cap0(self) -> crate::common::Reg<regs::Ctimer2cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer2cap1(self) -> crate::common::Reg<regs::Ctimer2cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer2cap2(self) -> crate::common::Reg<regs::Ctimer2cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer2cap3(self) -> crate::common::Reg<regs::Ctimer2cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::common::Reg<regs::Timer2trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Inputmux Register for SMARTDMA Arch B Inputs."]
    #[inline(always)]
    pub const fn smartdmaarchb_inmux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SmartdmaarchbInmux, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "Pin Interrupt Select."]
    #[inline(always)]
    pub const fn pintsel(self, n: usize) -> crate::common::Reg<regs::Pintsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Selection for Frequency Measurement Reference Clock."]
    #[inline(always)]
    pub const fn freqmeas_ref(self) -> crate::common::Reg<regs::FreqmeasRef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for Frequency Measurement Target Clock."]
    #[inline(always)]
    pub const fn freqmeas_tar(self) -> crate::common::Reg<regs::FreqmeasTar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer3cap0(self) -> crate::common::Reg<regs::Ctimer3cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer3cap1(self) -> crate::common::Reg<regs::Ctimer3cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer3cap2(self) -> crate::common::Reg<regs::Ctimer3cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer3cap3(self) -> crate::common::Reg<regs::Ctimer3cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::common::Reg<regs::Timer3trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer4cap0(self) -> crate::common::Reg<regs::Ctimer4cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer4cap1(self) -> crate::common::Reg<regs::Ctimer4cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer4cap2(self) -> crate::common::Reg<regs::Ctimer4cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer4cap3(self) -> crate::common::Reg<regs::Ctimer4cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::common::Reg<regs::Timer4trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "CMP0 Input Connections."]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::common::Reg<regs::Cmp0Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger Input Connections."]
    #[inline(always)]
    pub const fn adc0_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Adc0Trig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger Input Connections."]
    #[inline(always)]
    pub const fn adc1_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Adc1Trig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::common::Reg<regs::Dac0Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "DAC1 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac1_trig(self) -> crate::common::Reg<regs::Dac1Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "DAC2 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac2_trig(self) -> crate::common::Reg<regs::Dac2Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_trig(self) -> crate::common::Reg<regs::Qdc0Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "QDC0 Input Connections."]
    #[inline(always)]
    pub const fn qdc0_home(self) -> crate::common::Reg<regs::Qdc0Home, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "QDC0 Input Connections."]
    #[inline(always)]
    pub const fn qdc0_index(self) -> crate::common::Reg<regs::Qdc0Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "QDC0 Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phaseb(self) -> crate::common::Reg<regs::Qdc0Phaseb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "QDC0 Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phasea(self) -> crate::common::Reg<regs::Qdc0Phasea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_trig(self) -> crate::common::Reg<regs::Qdc1Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "QDC1 Input Connections."]
    #[inline(always)]
    pub const fn qdc1_home(self) -> crate::common::Reg<regs::Qdc1Home, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "QDC1 Input Connections."]
    #[inline(always)]
    pub const fn qdc1_index(self) -> crate::common::Reg<regs::Qdc1Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "QDC1 Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phaseb(self) -> crate::common::Reg<regs::Qdc1Phaseb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "QDC1 Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phasea(self) -> crate::common::Reg<regs::Qdc1Phasea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "PWM0 External Synchronization."]
    #[inline(always)]
    pub const fn flex_pwm0_sm_extsync(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm0SmExtsync, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm_exta(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm0SmExta, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 External Force Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_extforce(
        self,
    ) -> crate::common::Reg<regs::FlexPwm0Extforce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "PWM0 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_fault(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm0Fault, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 External Synchronization."]
    #[inline(always)]
    pub const fn flex_pwm1_sm_extsync(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm1SmExtsync, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 Input EXTA Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm_exta(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm1SmExta, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 External Force Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_extforce(
        self,
    ) -> crate::common::Reg<regs::FlexPwm1Extforce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "PWM1 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_fault(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm1Fault, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 External Clock Trigger."]
    #[inline(always)]
    pub const fn pwm0_ext_clk(self) -> crate::common::Reg<regs::Pwm0ExtClk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "PWM1 External Clock Trigger."]
    #[inline(always)]
    pub const fn pwm1_ext_clk(self) -> crate::common::Reg<regs::Pwm1ExtClk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig0(self) -> crate::common::Reg<regs::EvtgTrig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig1(self) -> crate::common::Reg<regs::EvtgTrig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig2(self) -> crate::common::Reg<regs::EvtgTrig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig3(self) -> crate::common::Reg<regs::EvtgTrig3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig4(self) -> crate::common::Reg<regs::EvtgTrig4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig5(self) -> crate::common::Reg<regs::EvtgTrig5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig6(self) -> crate::common::Reg<regs::EvtgTrig6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig7(self) -> crate::common::Reg<regs::EvtgTrig7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig8(self) -> crate::common::Reg<regs::EvtgTrig8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig9(self) -> crate::common::Reg<regs::EvtgTrig9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig10(self) -> crate::common::Reg<regs::EvtgTrig10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig11(self) -> crate::common::Reg<regs::EvtgTrig11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig12(self) -> crate::common::Reg<regs::EvtgTrig12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig13(self) -> crate::common::Reg<regs::EvtgTrig13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig14(self) -> crate::common::Reg<regs::EvtgTrig14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig15(self) -> crate::common::Reg<regs::EvtgTrig15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "USB-FS Trigger Input Connections."]
    #[inline(always)]
    pub const fn usbfs_trig(self) -> crate::common::Reg<regs::UsbfsTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "TSI Trigger Input Connections."]
    #[inline(always)]
    pub const fn tsi_trig(self) -> crate::common::Reg<regs::TsiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig0(self) -> crate::common::Reg<regs::ExtTrig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig1(self) -> crate::common::Reg<regs::ExtTrig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig2(self) -> crate::common::Reg<regs::ExtTrig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig3(self) -> crate::common::Reg<regs::ExtTrig3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig4(self) -> crate::common::Reg<regs::ExtTrig4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig5(self) -> crate::common::Reg<regs::ExtTrig5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig6(self) -> crate::common::Reg<regs::ExtTrig6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
    #[inline(always)]
    pub const fn ext_trig7(self) -> crate::common::Reg<regs::ExtTrig7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "CMP1 Input Connections."]
    #[inline(always)]
    pub const fn cmp1_trig(self) -> crate::common::Reg<regs::Cmp1Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "CMP2 Input Connections."]
    #[inline(always)]
    pub const fn cmp2_trig(self) -> crate::common::Reg<regs::Cmp2Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch0(
        self,
    ) -> crate::common::Reg<regs::SincFilterCh0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch1(
        self,
    ) -> crate::common::Reg<regs::SincFilterCh1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch2(
        self,
    ) -> crate::common::Reg<regs::SincFilterCh2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch3(
        self,
    ) -> crate::common::Reg<regs::SincFilterCh3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch4(
        self,
    ) -> crate::common::Reg<regs::SincFilterCh4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "OPAMP Trigger Input Connections."]
    #[inline(always)]
    pub const fn opamp0_trig(self) -> crate::common::Reg<regs::Opamp0Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "OPAMP Trigger Input Connections."]
    #[inline(always)]
    pub const fn opamp1_trig(self) -> crate::common::Reg<regs::Opamp1Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "OPAMP Trigger Input Connections."]
    #[inline(always)]
    pub const fn opamp2_trig(self) -> crate::common::Reg<regs::Opamp2Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "LP_FLEXCOMM0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm0_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm0Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm1_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm1Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM2 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm2_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm2Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM3 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm3_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm3Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LP_FLEXCOMM4 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm4_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm4Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "LP_FLEXCOMM5 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm5_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm5Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "LP_FLEXCOMM6 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm6_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm6Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "LP_FLEXCOMM7 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm7_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm7Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "LP_FLEXCOMM8 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm8_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm8Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM9 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm9_trig(
        self,
    ) -> crate::common::Reg<regs::Flexcomm9Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig0(self) -> crate::common::Reg<regs::FlexioTrig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig1(self) -> crate::common::Reg<regs::FlexioTrig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e4usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig2(self) -> crate::common::Reg<regs::FlexioTrig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e8usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig3(self) -> crate::common::Reg<regs::FlexioTrig3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ecusize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig4(self) -> crate::common::Reg<regs::FlexioTrig4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig5(self) -> crate::common::Reg<regs::FlexioTrig5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f4usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig6(self) -> crate::common::Reg<regs::FlexioTrig6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f8usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig7(self) -> crate::common::Reg<regs::FlexioTrig7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06fcusize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_tog(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable0Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_tog(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable1Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable2Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable2Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_tog(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable2Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3_set(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable3Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3_clr(
        self,
    ) -> crate::common::Reg<regs::Dma0ReqEnable3Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0738usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_tog(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable0Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_tog(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable1Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x079cusize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable2Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable2Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_tog(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable2Tog, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07acusize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3_set(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable3Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3_clr(
        self,
    ) -> crate::common::Reg<regs::Dma1ReqEnable3Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
