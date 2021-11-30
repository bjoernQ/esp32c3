#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` reader - backup_bus_pms_constrain_uart"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UART_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_UART_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_UART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_UART_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART` writer - backup_bus_pms_constrain_uart"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UART_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_UART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` reader - backup_bus_pms_constrain_g0spi_1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1` writer - backup_bus_pms_constrain_g0spi_1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` reader - backup_bus_pms_constrain_g0spi_0"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0` writer - backup_bus_pms_constrain_g0spi_0"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` reader - backup_bus_pms_constrain_gpio"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_GPIO_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_GPIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_GPIO` writer - backup_bus_pms_constrain_gpio"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` reader - backup_bus_pms_constrain_fe2"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_FE2_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_FE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_FE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE2` writer - backup_bus_pms_constrain_fe2"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_FE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_FE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` reader - backup_bus_pms_constrain_fe"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_FE_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_FE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_FE` writer - backup_bus_pms_constrain_fe"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_FE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_FE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMER` reader - backup_bus_pms_constrain_timer"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMER_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_TIMER` writer - backup_bus_pms_constrain_timer"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` reader - backup_bus_pms_constrain_rtc"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RTC_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_RTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTC` writer - backup_bus_pms_constrain_rtc"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` reader - backup_bus_pms_constrain_io_mux"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_IO_MUX` writer - backup_bus_pms_constrain_io_mux"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WDG` reader - backup_bus_pms_constrain_wdg"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WDG_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_WDG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_WDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_WDG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_WDG` writer - backup_bus_pms_constrain_wdg"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_WDG_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_WDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` reader - backup_bus_pms_constrain_misc"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_MISC_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_MISC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_MISC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_MISC` writer - backup_bus_pms_constrain_misc"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_MISC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` reader - backup_bus_pms_constrain_i2c"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2C_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_I2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_I2C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_I2C` writer - backup_bus_pms_constrain_i2c"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_I2C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` reader - backup_bus_pms_constrain_uart1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UART1_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_CONSTRAIN_UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_CONSTRAIN_UART1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_UART1` writer - backup_bus_pms_constrain_uart1"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_CONSTRAIN_UART1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_uart"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_g0spi_1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_g0spi_0"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_0(&self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_gpio"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_gpio(&self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_R {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_fe2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe2(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_fe"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe(&self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_R {
        BACKUP_BUS_PMS_CONSTRAIN_FE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_timer"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timer(&self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMER_R {
        BACKUP_BUS_PMS_CONSTRAIN_TIMER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_rtc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_io_mux"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_io_mux(&self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - backup_bus_pms_constrain_wdg"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wdg(&self) -> BACKUP_BUS_PMS_CONSTRAIN_WDG_R {
        BACKUP_BUS_PMS_CONSTRAIN_WDG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - backup_bus_pms_constrain_misc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_misc(&self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_R {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_i2c"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c(&self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_R {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_uart1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart1(&self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_R {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - backup_bus_pms_constrain_uart"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART_W {
        BACKUP_BUS_PMS_CONSTRAIN_UART_W { w: self }
    }
    #[doc = "Bits 2:3 - backup_bus_pms_constrain_g0spi_1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_1_W { w: self }
    }
    #[doc = "Bits 4:5 - backup_bus_pms_constrain_g0spi_0"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_g0spi_0(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W {
        BACKUP_BUS_PMS_CONSTRAIN_G0SPI_0_W { w: self }
    }
    #[doc = "Bits 6:7 - backup_bus_pms_constrain_gpio"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_gpio(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_GPIO_W {
        BACKUP_BUS_PMS_CONSTRAIN_GPIO_W { w: self }
    }
    #[doc = "Bits 8:9 - backup_bus_pms_constrain_fe2"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe2(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE2_W {
        BACKUP_BUS_PMS_CONSTRAIN_FE2_W { w: self }
    }
    #[doc = "Bits 10:11 - backup_bus_pms_constrain_fe"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_fe(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_FE_W {
        BACKUP_BUS_PMS_CONSTRAIN_FE_W { w: self }
    }
    #[doc = "Bits 12:13 - backup_bus_pms_constrain_timer"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_timer(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_TIMER_W {
        BACKUP_BUS_PMS_CONSTRAIN_TIMER_W { w: self }
    }
    #[doc = "Bits 14:15 - backup_bus_pms_constrain_rtc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_RTC_W {
        BACKUP_BUS_PMS_CONSTRAIN_RTC_W { w: self }
    }
    #[doc = "Bits 16:17 - backup_bus_pms_constrain_io_mux"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_io_mux(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W {
        BACKUP_BUS_PMS_CONSTRAIN_IO_MUX_W { w: self }
    }
    #[doc = "Bits 18:19 - backup_bus_pms_constrain_wdg"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_wdg(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_WDG_W {
        BACKUP_BUS_PMS_CONSTRAIN_WDG_W { w: self }
    }
    #[doc = "Bits 24:25 - backup_bus_pms_constrain_misc"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_misc(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_MISC_W {
        BACKUP_BUS_PMS_CONSTRAIN_MISC_W { w: self }
    }
    #[doc = "Bits 26:27 - backup_bus_pms_constrain_i2c"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_i2c(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_I2C_W {
        BACKUP_BUS_PMS_CONSTRAIN_I2C_W { w: self }
    }
    #[doc = "Bits 30:31 - backup_bus_pms_constrain_uart1"]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_uart1(&mut self) -> BACKUP_BUS_PMS_CONSTRAIN_UART1_W {
        BACKUP_BUS_PMS_CONSTRAIN_UART1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_constrain_1](index.html) module"]
pub struct BACKUP_BUS_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_1 to value 0xcf0f_ffff"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xcf0f_ffff
    }
}
