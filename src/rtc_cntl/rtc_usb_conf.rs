#[doc = "Register `RTC_USB_CONF` reader"]
pub struct R(crate::R<RTC_USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_USB_CONF` writer"]
pub struct W(crate::W<RTC_USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_USB_CONF_SPEC>;
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
impl From<crate::W<RTC_USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - disable io_mux reset"]
pub struct IO_MUX_RESET_DISABLE_R(crate::FieldReader<bool, bool>);
impl IO_MUX_RESET_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_MUX_RESET_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_MUX_RESET_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - disable io_mux reset"]
pub struct IO_MUX_RESET_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MUX_RESET_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - disable io_mux reset"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - disable io_mux reset"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W {
        IO_MUX_RESET_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_usb_conf](index.html) module"]
pub struct RTC_USB_CONF_SPEC;
impl crate::RegisterSpec for RTC_USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_usb_conf::R](R) reader structure"]
impl crate::Readable for RTC_USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_usb_conf::W](W) writer structure"]
impl crate::Writable for RTC_USB_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_USB_CONF to value 0"]
impl crate::Resettable for RTC_USB_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
