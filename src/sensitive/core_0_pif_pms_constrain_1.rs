#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` reader - core_0_pif_pms_constrain_world_0_uart"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` writer - core_0_pif_pms_constrain_world_0_uart"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` reader - core_0_pif_pms_constrain_world_0_g0spi_1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` writer - core_0_pif_pms_constrain_world_0_g0spi_1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` reader - core_0_pif_pms_constrain_world_0_g0spi_0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` writer - core_0_pif_pms_constrain_world_0_g0spi_0"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` reader - core_0_pif_pms_constrain_world_0_gpio"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` writer - core_0_pif_pms_constrain_world_0_gpio"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` reader - core_0_pif_pms_constrain_world_0_fe2"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` writer - core_0_pif_pms_constrain_world_0_fe2"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` reader - core_0_pif_pms_constrain_world_0_fe"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` writer - core_0_pif_pms_constrain_world_0_fe"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER` reader - core_0_pif_pms_constrain_world_0_timer"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER` writer - core_0_pif_pms_constrain_world_0_timer"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` reader - core_0_pif_pms_constrain_world_0_rtc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` writer - core_0_pif_pms_constrain_world_0_rtc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` reader - core_0_pif_pms_constrain_world_0_io_mux"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` writer - core_0_pif_pms_constrain_world_0_io_mux"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG` reader - core_0_pif_pms_constrain_world_0_wdg"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG` writer - core_0_pif_pms_constrain_world_0_wdg"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` reader - core_0_pif_pms_constrain_world_0_misc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` writer - core_0_pif_pms_constrain_world_0_misc"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` reader - core_0_pif_pms_constrain_world_0_i2c"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` writer - core_0_pif_pms_constrain_world_0_i2c"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` reader - core_0_pif_pms_constrain_world_0_uart1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` writer - core_0_pif_pms_constrain_world_0_uart1"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_uart"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_0_g0spi_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_g0spi_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_gpio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_gpio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_0_fe2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe2(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_fe"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_0_timer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_rtc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rtc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_io_mux"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_0_wdg"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wdg(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_0_misc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_misc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_i2c"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_uart1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - core_0_pif_pms_constrain_world_0_uart"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W { w: self }
    }
    #[doc = "Bits 2:3 - core_0_pif_pms_constrain_world_0_g0spi_1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W { w: self }
    }
    #[doc = "Bits 4:5 - core_0_pif_pms_constrain_world_0_g0spi_0"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W { w: self }
    }
    #[doc = "Bits 6:7 - core_0_pif_pms_constrain_world_0_gpio"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_gpio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W { w: self }
    }
    #[doc = "Bits 8:9 - core_0_pif_pms_constrain_world_0_fe2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W { w: self }
    }
    #[doc = "Bits 10:11 - core_0_pif_pms_constrain_world_0_fe"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W { w: self }
    }
    #[doc = "Bits 12:13 - core_0_pif_pms_constrain_world_0_timer"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMER_W { w: self }
    }
    #[doc = "Bits 14:15 - core_0_pif_pms_constrain_world_0_rtc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rtc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W { w: self }
    }
    #[doc = "Bits 16:17 - core_0_pif_pms_constrain_world_0_io_mux"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W { w: self }
    }
    #[doc = "Bits 18:19 - core_0_pif_pms_constrain_world_0_wdg"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wdg(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WDG_W { w: self }
    }
    #[doc = "Bits 24:25 - core_0_pif_pms_constrain_world_0_misc"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_misc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W { w: self }
    }
    #[doc = "Bits 26:27 - core_0_pif_pms_constrain_world_0_i2c"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W { w: self }
    }
    #[doc = "Bits 30:31 - core_0_pif_pms_constrain_world_0_uart1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_1](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_1 to value 0xcf0f_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xcf0f_ffff
    }
}
