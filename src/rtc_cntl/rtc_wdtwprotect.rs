#[doc = "Register `RTC_WDTWPROTECT` reader"]
pub struct R(crate::R<RTC_WDTWPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WDTWPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WDTWPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WDTWPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WDTWPROTECT` writer"]
pub struct W(crate::W<RTC_WDTWPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WDTWPROTECT_SPEC>;
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
impl From<crate::W<RTC_WDTWPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WDTWPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_WKEY` reader - the key of rtc wdt"]
pub struct WDT_WKEY_R(crate::FieldReader<u32, u32>);
impl WDT_WKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WDT_WKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_WKEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_WKEY` writer - the key of rtc wdt"]
pub struct WDT_WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_WKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - the key of rtc wdt"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - the key of rtc wdt"]
    #[inline(always)]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W {
        WDT_WKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wdtwprotect](index.html) module"]
pub struct RTC_WDTWPROTECT_SPEC;
impl crate::RegisterSpec for RTC_WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wdtwprotect::R](R) reader structure"]
impl crate::Readable for RTC_WDTWPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wdtwprotect::W](W) writer structure"]
impl crate::Writable for RTC_WDTWPROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WDTWPROTECT to value 0"]
impl crate::Resettable for RTC_WDTWPROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
