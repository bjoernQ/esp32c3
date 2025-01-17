#[doc = "Register `RTC_WDTFEED` writer"]
pub struct W(crate::W<RTC_WDTFEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WDTFEED_SPEC>;
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
impl From<crate::W<RTC_WDTFEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WDTFEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WDT_FEED` writer - sw feed rtc wdt"]
pub struct RTC_WDT_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_WDT_FEED_W<'a> {
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
impl W {
    #[doc = "Bit 31 - sw feed rtc wdt"]
    #[inline(always)]
    pub fn rtc_wdt_feed(&mut self) -> RTC_WDT_FEED_W {
        RTC_WDT_FEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wdtfeed](index.html) module"]
pub struct RTC_WDTFEED_SPEC;
impl crate::RegisterSpec for RTC_WDTFEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_wdtfeed::W](W) writer structure"]
impl crate::Writable for RTC_WDTFEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WDTFEED to value 0"]
impl crate::Resettable for RTC_WDTFEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
