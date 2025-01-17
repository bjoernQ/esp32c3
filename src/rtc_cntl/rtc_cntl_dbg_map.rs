#[doc = "Register `RTC_CNTL_DBG_MAP` reader"]
pub struct R(crate::R<RTC_CNTL_DBG_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_DBG_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_DBG_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_DBG_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_DBG_MAP` writer"]
pub struct W(crate::W<RTC_CNTL_DBG_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_DBG_MAP_SPEC>;
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
impl From<crate::W<RTC_CNTL_DBG_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_DBG_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_GPIO_PIN5_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN5_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN5_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN5_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN5_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN5_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN5_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN5_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN4_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN4_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN4_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN4_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN4_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN4_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN4_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN4_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN3_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN3_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN3_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN3_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN3_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN3_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN3_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN3_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN2_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN2_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN2_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN2_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN2_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN2_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN2_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN2_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN1_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN1_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN1_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN1_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN1_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN1_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN1_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN0_MUX_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN0_MUX_SEL_R(crate::FieldReader<bool, bool>);
impl RTC_GPIO_PIN0_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GPIO_PIN0_MUX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN0_MUX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN0_MUX_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN0_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN0_MUX_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN5_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN5_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN5_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN5_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN5_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN5_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN5_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN5_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN4_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN4_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN4_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN4_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN4_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN4_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN4_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN4_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN3_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN3_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN3_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN3_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN3_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN3_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN3_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN3_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN2_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN2_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN2_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN2_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN2_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN2_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN2_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN2_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN1_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN1_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN1_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN1_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN1_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN1_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN1_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN1_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RTC_GPIO_PIN0_FUN_SEL` reader - use for debug"]
pub struct RTC_GPIO_PIN0_FUN_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_GPIO_PIN0_FUN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_GPIO_PIN0_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GPIO_PIN0_FUN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GPIO_PIN0_FUN_SEL` writer - use for debug"]
pub struct RTC_GPIO_PIN0_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN0_FUN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_mux_sel(&self) -> RTC_GPIO_PIN5_MUX_SEL_R {
        RTC_GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_mux_sel(&self) -> RTC_GPIO_PIN4_MUX_SEL_R {
        RTC_GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_mux_sel(&self) -> RTC_GPIO_PIN3_MUX_SEL_R {
        RTC_GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_mux_sel(&self) -> RTC_GPIO_PIN2_MUX_SEL_R {
        RTC_GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_mux_sel(&self) -> RTC_GPIO_PIN1_MUX_SEL_R {
        RTC_GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_mux_sel(&self) -> RTC_GPIO_PIN0_MUX_SEL_R {
        RTC_GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_fun_sel(&self) -> RTC_GPIO_PIN5_FUN_SEL_R {
        RTC_GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_fun_sel(&self) -> RTC_GPIO_PIN4_FUN_SEL_R {
        RTC_GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_fun_sel(&self) -> RTC_GPIO_PIN3_FUN_SEL_R {
        RTC_GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_fun_sel(&self) -> RTC_GPIO_PIN2_FUN_SEL_R {
        RTC_GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_fun_sel(&self) -> RTC_GPIO_PIN1_FUN_SEL_R {
        RTC_GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_fun_sel(&self) -> RTC_GPIO_PIN0_FUN_SEL_R {
        RTC_GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_mux_sel(&mut self) -> RTC_GPIO_PIN5_MUX_SEL_W {
        RTC_GPIO_PIN5_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 3 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_mux_sel(&mut self) -> RTC_GPIO_PIN4_MUX_SEL_W {
        RTC_GPIO_PIN4_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 4 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_mux_sel(&mut self) -> RTC_GPIO_PIN3_MUX_SEL_W {
        RTC_GPIO_PIN3_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 5 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_mux_sel(&mut self) -> RTC_GPIO_PIN2_MUX_SEL_W {
        RTC_GPIO_PIN2_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 6 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_mux_sel(&mut self) -> RTC_GPIO_PIN1_MUX_SEL_W {
        RTC_GPIO_PIN1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 7 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_mux_sel(&mut self) -> RTC_GPIO_PIN0_MUX_SEL_W {
        RTC_GPIO_PIN0_MUX_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_fun_sel(&mut self) -> RTC_GPIO_PIN5_FUN_SEL_W {
        RTC_GPIO_PIN5_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin4_fun_sel(&mut self) -> RTC_GPIO_PIN4_FUN_SEL_W {
        RTC_GPIO_PIN4_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin3_fun_sel(&mut self) -> RTC_GPIO_PIN3_FUN_SEL_W {
        RTC_GPIO_PIN3_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin2_fun_sel(&mut self) -> RTC_GPIO_PIN2_FUN_SEL_W {
        RTC_GPIO_PIN2_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin1_fun_sel(&mut self) -> RTC_GPIO_PIN1_FUN_SEL_W {
        RTC_GPIO_PIN1_FUN_SEL_W { w: self }
    }
    #[doc = "Bits 28:31 - use for debug"]
    #[inline(always)]
    pub fn rtc_gpio_pin0_fun_sel(&mut self) -> RTC_GPIO_PIN0_FUN_SEL_W {
        RTC_GPIO_PIN0_FUN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_map](index.html) module"]
pub struct RTC_CNTL_DBG_MAP_SPEC;
impl crate::RegisterSpec for RTC_CNTL_DBG_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_dbg_map::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_map::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_DBG_MAP to value 0"]
impl crate::Resettable for RTC_CNTL_DBG_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
