#[doc = "Register `RTC_TIME_HIGH1` reader"]
pub struct R(crate::R<RTC_TIME_HIGH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_HIGH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_HIGH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_HIGH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_TIMER_VALUE1_HIGH` reader - RTC timer high 16 bits"]
pub struct RTC_TIMER_VALUE1_HIGH_R(crate::FieldReader<u16, u16>);
impl RTC_TIMER_VALUE1_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_TIMER_VALUE1_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIMER_VALUE1_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC timer high 16 bits"]
    #[inline(always)]
    pub fn rtc_timer_value1_high(&self) -> RTC_TIMER_VALUE1_HIGH_R {
        RTC_TIMER_VALUE1_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_high1](index.html) module"]
pub struct RTC_TIME_HIGH1_SPEC;
impl crate::RegisterSpec for RTC_TIME_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_high1::R](R) reader structure"]
impl crate::Readable for RTC_TIME_HIGH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_TIME_HIGH1 to value 0"]
impl crate::Resettable for RTC_TIME_HIGH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
