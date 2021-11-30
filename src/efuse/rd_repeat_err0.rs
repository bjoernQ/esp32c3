#[doc = "Register `RD_REPEAT_ERR0` reader"]
pub struct R(crate::R<RD_REPEAT_ERR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS_ERR` reader - If any bit in RD_DIS is 1, then it indicates a programming error."]
pub struct RD_DIS_ERR_R(crate::FieldReader<u8, u8>);
impl RD_DIS_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RD_DIS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DIS_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RTC_RAM_BOOT_ERR` reader - If DIS_RTC_RAM_BOOT is 1, then it indicates a programming error."]
pub struct DIS_RTC_RAM_BOOT_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_RTC_RAM_BOOT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RTC_RAM_BOOT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_RTC_RAM_BOOT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_ICACHE_ERR` reader - If DIS_ICACHE is 1, then it indicates a programming error."]
pub struct DIS_ICACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_ICACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_ICACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_ICACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_JTAG_ERR` reader - If DIS_USB_JTAG is 1, then it indicates a programming error."]
pub struct DIS_USB_JTAG_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_USB_JTAG_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_JTAG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_JTAG_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_ICACHE_ERR` reader - If DIS_DOWNLOAD_ICACHE is 1, then it indicates a programming error."]
pub struct DIS_DOWNLOAD_ICACHE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_ICACHE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_ICACHE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_ICACHE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_DEVICE_ERR` reader - If DIS_USB_DEVICE is 1, then it indicates a programming error."]
pub struct DIS_USB_DEVICE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_USB_DEVICE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_DEVICE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_DEVICE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_FORCE_DOWNLOAD_ERR` reader - If DIS_FORCE_DOWNLOAD is 1, then it indicates a programming error."]
pub struct DIS_FORCE_DOWNLOAD_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_FORCE_DOWNLOAD_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_FORCE_DOWNLOAD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_FORCE_DOWNLOAD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED6_ERR` reader - Reserved."]
pub struct RPT4_RESERVED6_ERR_R(crate::FieldReader<bool, bool>);
impl RPT4_RESERVED6_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPT4_RESERVED6_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED6_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_CAN_ERR` reader - If DIS_CAN is 1, then it indicates a programming error."]
pub struct DIS_CAN_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_CAN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_CAN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_CAN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_SEL_ENABLE_ERR` reader - If JTAG_SEL_ENABLE is 1, then it indicates a programming error."]
pub struct JTAG_SEL_ENABLE_ERR_R(crate::FieldReader<bool, bool>);
impl JTAG_SEL_ENABLE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_SEL_ENABLE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_SEL_ENABLE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_DIS_JTAG_ERR` reader - If SOFT_DIS_JTAG is 1, then it indicates a programming error."]
pub struct SOFT_DIS_JTAG_ERR_R(crate::FieldReader<u8, u8>);
impl SOFT_DIS_JTAG_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOFT_DIS_JTAG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_DIS_JTAG_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_PAD_JTAG_ERR` reader - If DIS_PAD_JTAG is 1, then it indicates a programming error."]
pub struct DIS_PAD_JTAG_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_PAD_JTAG_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_PAD_JTAG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_PAD_JTAG_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - If DIS_DOWNLOAD_MANUAL_ENCRYPT is 1, then it indicates a programming error."]
pub struct DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFH_ERR` reader - If any bit in USB_DREFH is 1, then it indicates a programming error."]
pub struct USB_DREFH_ERR_R(crate::FieldReader<u8, u8>);
impl USB_DREFH_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFH_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFH_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFL_ERR` reader - If any bit in USB_DREFL is 1, then it indicates a programming error."]
pub struct USB_DREFL_ERR_R(crate::FieldReader<u8, u8>);
impl USB_DREFL_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFL_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_EXCHG_PINS_ERR` reader - If USB_EXCHG_PINS is 1, then it indicates a programming error."]
pub struct USB_EXCHG_PINS_ERR_R(crate::FieldReader<bool, bool>);
impl USB_EXCHG_PINS_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_EXCHG_PINS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_EXCHG_PINS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_AS_GPIO_ERR` reader - If VDD_SPI_AS_GPIO is 1, then it indicates a programming error."]
pub struct VDD_SPI_AS_GPIO_ERR_R(crate::FieldReader<bool, bool>);
impl VDD_SPI_AS_GPIO_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDD_SPI_AS_GPIO_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_AS_GPIO_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTLC_GPIO_ENABLE_ERR` reader - If any bit in BTLC_GPIO_ENABLE is 1, then it indicates a programming error."]
pub struct BTLC_GPIO_ENABLE_ERR_R(crate::FieldReader<u8, u8>);
impl BTLC_GPIO_ENABLE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BTLC_GPIO_ENABLE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTLC_GPIO_ENABLE_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWERGLITCH_EN_ERR` reader - If POWERGLITCH_EN is 1, then it indicates a programming error."]
pub struct POWERGLITCH_EN_ERR_R(crate::FieldReader<bool, bool>);
impl POWERGLITCH_EN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POWERGLITCH_EN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWERGLITCH_EN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_DSENSE_ERR` reader - If any bit in POWER_GLITCH_DSENSE is 1, then it indicates a programming error."]
pub struct POWER_GLITCH_DSENSE_ERR_R(crate::FieldReader<u8, u8>);
impl POWER_GLITCH_DSENSE_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POWER_GLITCH_DSENSE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_DSENSE_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - If any bit in RD_DIS is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - If DIS_RTC_RAM_BOOT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_rtc_ram_boot_err(&self) -> DIS_RTC_RAM_BOOT_ERR_R {
        DIS_RTC_RAM_BOOT_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If DIS_ICACHE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_icache_err(&self) -> DIS_ICACHE_ERR_R {
        DIS_ICACHE_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If DIS_USB_JTAG is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_jtag_err(&self) -> DIS_USB_JTAG_ERR_R {
        DIS_USB_JTAG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If DIS_DOWNLOAD_ICACHE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_icache_err(&self) -> DIS_DOWNLOAD_ICACHE_ERR_R {
        DIS_DOWNLOAD_ICACHE_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - If DIS_USB_DEVICE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_device_err(&self) -> DIS_USB_DEVICE_ERR_R {
        DIS_USB_DEVICE_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If DIS_FORCE_DOWNLOAD is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> DIS_FORCE_DOWNLOAD_ERR_R {
        DIS_FORCE_DOWNLOAD_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved6_err(&self) -> RPT4_RESERVED6_ERR_R {
        RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If DIS_CAN is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_can_err(&self) -> DIS_CAN_ERR_R {
        DIS_CAN_ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If JTAG_SEL_ENABLE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn jtag_sel_enable_err(&self) -> JTAG_SEL_ENABLE_ERR_R {
        JTAG_SEL_ENABLE_ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - If SOFT_DIS_JTAG is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> SOFT_DIS_JTAG_ERR_R {
        SOFT_DIS_JTAG_ERR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - If DIS_PAD_JTAG is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> DIS_PAD_JTAG_ERR_R {
        DIS_PAD_JTAG_ERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - If DIS_DOWNLOAD_MANUAL_ENCRYPT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - If any bit in USB_DREFH is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_drefh_err(&self) -> USB_DREFH_ERR_R {
        USB_DREFH_ERR_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - If any bit in USB_DREFL is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_drefl_err(&self) -> USB_DREFL_ERR_R {
        USB_DREFL_ERR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 25 - If USB_EXCHG_PINS is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_exchg_pins_err(&self) -> USB_EXCHG_PINS_ERR_R {
        USB_EXCHG_PINS_ERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - If VDD_SPI_AS_GPIO is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_as_gpio_err(&self) -> VDD_SPI_AS_GPIO_ERR_R {
        VDD_SPI_AS_GPIO_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - If any bit in BTLC_GPIO_ENABLE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn btlc_gpio_enable_err(&self) -> BTLC_GPIO_ENABLE_ERR_R {
        BTLC_GPIO_ENABLE_ERR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - If POWERGLITCH_EN is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn powerglitch_en_err(&self) -> POWERGLITCH_EN_ERR_R {
        POWERGLITCH_EN_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - If any bit in POWER_GLITCH_DSENSE is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn power_glitch_dsense_err(&self) -> POWER_GLITCH_DSENSE_ERR_R {
        POWER_GLITCH_DSENSE_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
#[doc = "Programming error record register 0 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err0](index.html) module"]
pub struct RD_REPEAT_ERR0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err0::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR0 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
