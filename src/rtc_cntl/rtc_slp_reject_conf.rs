#[doc = "Register `RTC_SLP_REJECT_CONF` reader"]
pub struct R(crate::R<RTC_SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SLP_REJECT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SLP_REJECT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SLP_REJECT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SLP_REJECT_CONF` writer"]
pub struct W(crate::W<RTC_SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SLP_REJECT_CONF_SPEC>;
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
impl From<crate::W<RTC_SLP_REJECT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SLP_REJECT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SLEEP_REJECT_ENA` reader - sleep reject enable"]
pub struct RTC_SLEEP_REJECT_ENA_R(crate::FieldReader<u32, u32>);
impl RTC_SLEEP_REJECT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RTC_SLEEP_REJECT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SLEEP_REJECT_ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SLEEP_REJECT_ENA` writer - sleep reject enable"]
pub struct RTC_SLEEP_REJECT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SLEEP_REJECT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 12)) | ((value as u32 & 0x0003_ffff) << 12);
        self.w
    }
}
#[doc = "Field `LIGHT_SLP_REJECT_EN` reader - enable reject for light sleep"]
pub struct LIGHT_SLP_REJECT_EN_R(crate::FieldReader<bool, bool>);
impl LIGHT_SLP_REJECT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIGHT_SLP_REJECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIGHT_SLP_REJECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIGHT_SLP_REJECT_EN` writer - enable reject for light sleep"]
pub struct LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIGHT_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DEEP_SLP_REJECT_EN` reader - enable reject for deep sleep"]
pub struct DEEP_SLP_REJECT_EN_R(crate::FieldReader<bool, bool>);
impl DEEP_SLP_REJECT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEEP_SLP_REJECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLP_REJECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEP_SLP_REJECT_EN` writer - enable reject for deep sleep"]
pub struct DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLP_REJECT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:29 - sleep reject enable"]
    #[inline(always)]
    pub fn rtc_sleep_reject_ena(&self) -> RTC_SLEEP_REJECT_ENA_R {
        RTC_SLEEP_REJECT_ENA_R::new(((self.bits >> 12) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 30 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:29 - sleep reject enable"]
    #[inline(always)]
    pub fn rtc_sleep_reject_ena(&mut self) -> RTC_SLEEP_REJECT_ENA_W {
        RTC_SLEEP_REJECT_ENA_W { w: self }
    }
    #[doc = "Bit 30 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W {
        LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 31 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W {
        DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_slp_reject_conf](index.html) module"]
pub struct RTC_SLP_REJECT_CONF_SPEC;
impl crate::RegisterSpec for RTC_SLP_REJECT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_slp_reject_conf::R](R) reader structure"]
impl crate::Readable for RTC_SLP_REJECT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_slp_reject_conf::W](W) writer structure"]
impl crate::Writable for RTC_SLP_REJECT_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SLP_REJECT_CONF to value 0"]
impl crate::Resettable for RTC_SLP_REJECT_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
