#[doc = "FLASH_NMPA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashNmpa {
    ptr: *mut u8,
}
unsafe impl Send for FlashNmpa {}
unsafe impl Sync for FlashNmpa {}
impl FlashNmpa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPO0 register 0 description"]
    #[inline(always)]
    pub const fn gpo0_0(self) -> crate::common::Reg<regs::Gpo00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_array0(self) -> crate::common::Reg<regs::Gpo0Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPO0 register 1 description"]
    #[inline(always)]
    pub const fn gpo0_1(self) -> crate::common::Reg<regs::Gpo01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_array1(self) -> crate::common::Reg<regs::Gpo0Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPO0 register 2 description"]
    #[inline(always)]
    pub const fn gpo0_2(self) -> crate::common::Reg<regs::Gpo02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_array2(self) -> crate::common::Reg<regs::Gpo0Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPO0 register 3 description"]
    #[inline(always)]
    pub const fn gpo0_3(self) -> crate::common::Reg<regs::Gpo03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPO0 array description"]
    #[inline(always)]
    pub const fn gpo0_array3(self) -> crate::common::Reg<regs::Gpo0Array3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPO1 register 0 description"]
    #[inline(always)]
    pub const fn gpo1_0(self) -> crate::common::Reg<regs::Gpo10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_array0(self) -> crate::common::Reg<regs::Gpo1Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPO1 register 1 description"]
    #[inline(always)]
    pub const fn gpo1_1(self) -> crate::common::Reg<regs::Gpo11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_array1(self) -> crate::common::Reg<regs::Gpo1Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPO1 register 2 description"]
    #[inline(always)]
    pub const fn gpo1_2(self) -> crate::common::Reg<regs::Gpo12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_array2(self) -> crate::common::Reg<regs::Gpo1Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPO1 register 3 description"]
    #[inline(always)]
    pub const fn gpo1_3(self) -> crate::common::Reg<regs::Gpo13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPO1 array description"]
    #[inline(always)]
    pub const fn gpo1_array3(self) -> crate::common::Reg<regs::Gpo1Array3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPO2 register 0 description"]
    #[inline(always)]
    pub const fn gpo2_0(self) -> crate::common::Reg<regs::Gpo20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_array0(self) -> crate::common::Reg<regs::Gpo2Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GPO2 register 1 description"]
    #[inline(always)]
    pub const fn gpo2_1(self) -> crate::common::Reg<regs::Gpo21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_array1(self) -> crate::common::Reg<regs::Gpo2Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "GPO2 register 2 description"]
    #[inline(always)]
    pub const fn gpo2_2(self) -> crate::common::Reg<regs::Gpo22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_array2(self) -> crate::common::Reg<regs::Gpo2Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPO2 register 3 description"]
    #[inline(always)]
    pub const fn gpo2_3(self) -> crate::common::Reg<regs::Gpo23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPO2 array description"]
    #[inline(always)]
    pub const fn gpo2_array3(self) -> crate::common::Reg<regs::Gpo2Array3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPO3 register 0 description"]
    #[inline(always)]
    pub const fn gpo3_0(self) -> crate::common::Reg<regs::Gpo30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_array0(self) -> crate::common::Reg<regs::Gpo3Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPO3 register 1 description"]
    #[inline(always)]
    pub const fn gpo3_1(self) -> crate::common::Reg<regs::Gpo31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_array1(self) -> crate::common::Reg<regs::Gpo3Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPO3 register 2 description"]
    #[inline(always)]
    pub const fn gpo3_2(self) -> crate::common::Reg<regs::Gpo32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_array2(self) -> crate::common::Reg<regs::Gpo3Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPO3 register 3 description"]
    #[inline(always)]
    pub const fn gpo3_3(self) -> crate::common::Reg<regs::Gpo33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GPO3 array description"]
    #[inline(always)]
    pub const fn gpo3_array3(self) -> crate::common::Reg<regs::Gpo3Array3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "checksum of the GPO data in words 0"]
    #[inline(always)]
    pub const fn gpo_checksum_0(self) -> crate::common::Reg<regs::GpoChecksum0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_array0(
        self,
    ) -> crate::common::Reg<regs::GpoChecksumArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 1"]
    #[inline(always)]
    pub const fn gpo_checksum_1(self) -> crate::common::Reg<regs::GpoChecksum1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_array1(
        self,
    ) -> crate::common::Reg<regs::GpoChecksumArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 2"]
    #[inline(always)]
    pub const fn gpo_checksum_2(self) -> crate::common::Reg<regs::GpoChecksum2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_array2(
        self,
    ) -> crate::common::Reg<regs::GpoChecksumArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "checksum of the GPO data in words 3"]
    #[inline(always)]
    pub const fn gpo_checksum_3(self) -> crate::common::Reg<regs::GpoChecksum3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub const fn gpo_checksum_array3(
        self,
    ) -> crate::common::Reg<regs::GpoChecksumArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_0(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchId0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_array0(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchIdArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_1(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchId1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_array1(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchIdArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_2(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchId2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_array2(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchIdArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_3(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchId3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_batch_id_array3(
        self,
    ) -> crate::common::Reg<regs::FinalTestBatchIdArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn device_type(self) -> crate::common::Reg<regs::DeviceType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_program_version(
        self,
    ) -> crate::common::Reg<regs::FinalTestProgramVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_date(
        self,
    ) -> crate::common::Reg<regs::FinalTestDate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn final_test_time(
        self,
    ) -> crate::common::Reg<regs::FinalTestTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_0(self) -> crate::common::Reg<regs::Uuid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_array0(self) -> crate::common::Reg<regs::UuidArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_1(self) -> crate::common::Reg<regs::Uuid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_array1(self) -> crate::common::Reg<regs::UuidArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_2(self) -> crate::common::Reg<regs::Uuid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_array2(self) -> crate::common::Reg<regs::UuidArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_3(self) -> crate::common::Reg<regs::Uuid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn uuid_array3(self) -> crate::common::Reg<regs::UuidArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test1_program_version(
        self,
    ) -> crate::common::Reg<regs::WaferTest1ProgramVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test1_date(
        self,
    ) -> crate::common::Reg<regs::WaferTest1Date, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test1_time(
        self,
    ) -> crate::common::Reg<regs::WaferTest1Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test2_program_version(
        self,
    ) -> crate::common::Reg<regs::WaferTest2ProgramVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test2_date(
        self,
    ) -> crate::common::Reg<regs::WaferTest2Date, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn wafer_test2_time(
        self,
    ) -> crate::common::Reg<regs::WaferTest2Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn usbcfg(self) -> crate::common::Reg<regs::Usbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn periphencfg(self) -> crate::common::Reg<regs::Periphencfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ramsizecfg(self) -> crate::common::Reg<regs::Ramsizecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn flashsizecfg(self) -> crate::common::Reg<regs::Flashsizecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ringo_0(self) -> crate::common::Reg<regs::Ringo0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ringo_1(self) -> crate::common::Reg<regs::Ringo1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ringo_2(self) -> crate::common::Reg<regs::Ringo2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn fro_192mhz(self) -> crate::common::Reg<regs::Fro192mhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn xo_32mhz(self) -> crate::common::Reg<regs::Xo32mhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn xo_32khz(self) -> crate::common::Reg<regs::Xo32khz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn fro_1mhz(self) -> crate::common::Reg<regs::Fro1mhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileHigh0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_array0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileHighArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileHigh1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_high_array1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileHighArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileLow0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_array0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileLowArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileLow1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_low_array1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileLowArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileMedium0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_array0(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileMediumArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileMedium1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dcdc_power_profile_medium_array1(
        self,
    ) -> crate::common::Reg<regs::DcdcPowerProfileMediumArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn bod(self) -> crate::common::Reg<regs::Bod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ldo_ao(self) -> crate::common::Reg<regs::LdoAo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn sdio_delay(self) -> crate::common::Reg<regs::SdioDelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_0(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbient0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_array0(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbientArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_1(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbient1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_array1(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbientArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_2(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbient2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_array2(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbientArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_3(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbient3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_ambient_array3(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveAmbientArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_0(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTemp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_array0(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTempArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_1(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTemp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_array1(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTempArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_2(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTemp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_array2(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTempArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_3(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTemp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub const fn aux_bias_curve_temp_array3(
        self,
    ) -> crate::common::Reg<regs::AuxBiasCurveTempArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn temp_sens_vbe1vbe8_ref_1(
        self,
    ) -> crate::common::Reg<regs::TempSensVbe1vbe8Ref1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn temp_sens_vbe1vbe8_ref_2(
        self,
    ) -> crate::common::Reg<regs::TempSensVbe1vbe8Ref2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn temp_sens_slope(
        self,
    ) -> crate::common::Reg<regs::TempSensSlope, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn temp_sens_offset(
        self,
    ) -> crate::common::Reg<regs::TempSensOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_array0(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_ringo(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0Ringo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_array1(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_delays_lsb(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0DelaysLsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_array2(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_0_delays_msb(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor0DelaysMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_array0(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1Array0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_ringo(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1Ringo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_array1(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1Array1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_delays_lsb(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1DelaysLsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_array2(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1Array2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn pvt_monitor_1_delays_msb(
        self,
    ) -> crate::common::Reg<regs::PvtMonitor1DelaysMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn nxp_device_private_key(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NxpDevicePrivateKey, crate::common::RW> {
        assert!(n < 13usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_0(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NxpDeviceCertificate0, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_1(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NxpDeviceCertificate1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_2(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NxpDeviceCertificate2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
    #[inline(always)]
    pub const fn nxp_device_certificate_3(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::NxpDeviceCertificate3, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    #[inline(always)]
    pub const fn sha256_digest(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sha256Digest, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ecid_backup_0(self) -> crate::common::Reg<regs::EcidBackup0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_array0(
        self,
    ) -> crate::common::Reg<regs::EcidBackupArray0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ecid_backup_1(self) -> crate::common::Reg<regs::EcidBackup1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_array1(
        self,
    ) -> crate::common::Reg<regs::EcidBackupArray1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ecid_backup_2(self) -> crate::common::Reg<regs::EcidBackup2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_array2(
        self,
    ) -> crate::common::Reg<regs::EcidBackupArray2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ecid_backup_3(self) -> crate::common::Reg<regs::EcidBackup3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub const fn ecid_backup_array3(
        self,
    ) -> crate::common::Reg<regs::EcidBackupArray3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Checksum of the whole page"]
    #[inline(always)]
    pub const fn checksum(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn dis_rom_hiding(self) -> crate::common::Reg<regs::DisRomHiding, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cacusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn puf_sram(self) -> crate::common::Reg<regs::PufSram, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cbcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
