#[doc = "Register `INT_ST_RTC` reader"]
pub struct R(crate::R<INT_ST_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ST` reader - sleep wakeup interrupt state"]
pub struct SLP_WAKEUP_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_ST` reader - sleep reject interrupt state"]
pub struct SLP_REJECT_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_WDT_INT_ST` reader - RTC WDT interrupt state"]
pub struct RTC_WDT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_WDT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_WDT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WDT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_BROWN_OUT_INT_ST` reader - brown out interrupt state"]
pub struct RTC_BROWN_OUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_BROWN_OUT_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_BROWN_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_BROWN_OUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MAIN_TIMER_INT_ST` reader - RTC main timer interrupt state"]
pub struct RTC_MAIN_TIMER_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_MAIN_TIMER_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MAIN_TIMER_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MAIN_TIMER_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SWD_INT_ST` reader - super watch dog interrupt state"]
pub struct RTC_SWD_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_SWD_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SWD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SWD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_XTAL32K_DEAD_INT_ST` reader - xtal32k dead detection interrupt state"]
pub struct RTC_XTAL32K_DEAD_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_XTAL32K_DEAD_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_XTAL32K_DEAD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_XTAL32K_DEAD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_GLITCH_DET_INT_ST` reader - glitch_det_interrupt state"]
pub struct RTC_GLITCH_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_GLITCH_DET_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_GLITCH_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_GLITCH_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_BBPLL_CAL_INT_ST` reader - bbpll cal end interrupt state"]
pub struct RTC_BBPLL_CAL_INT_ST_R(crate::FieldReader<bool, bool>);
impl RTC_BBPLL_CAL_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_BBPLL_CAL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_BBPLL_CAL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - sleep wakeup interrupt state"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&self) -> SLP_WAKEUP_INT_ST_R {
        SLP_WAKEUP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - sleep reject interrupt state"]
    #[inline(always)]
    pub fn slp_reject_int_st(&self) -> SLP_REJECT_INT_ST_R {
        SLP_REJECT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC WDT interrupt state"]
    #[inline(always)]
    pub fn rtc_wdt_int_st(&self) -> RTC_WDT_INT_ST_R {
        RTC_WDT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - brown out interrupt state"]
    #[inline(always)]
    pub fn rtc_brown_out_int_st(&self) -> RTC_BROWN_OUT_INT_ST_R {
        RTC_BROWN_OUT_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC main timer interrupt state"]
    #[inline(always)]
    pub fn rtc_main_timer_int_st(&self) -> RTC_MAIN_TIMER_INT_ST_R {
        RTC_MAIN_TIMER_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - super watch dog interrupt state"]
    #[inline(always)]
    pub fn rtc_swd_int_st(&self) -> RTC_SWD_INT_ST_R {
        RTC_SWD_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - xtal32k dead detection interrupt state"]
    #[inline(always)]
    pub fn rtc_xtal32k_dead_int_st(&self) -> RTC_XTAL32K_DEAD_INT_ST_R {
        RTC_XTAL32K_DEAD_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 19 - glitch_det_interrupt state"]
    #[inline(always)]
    pub fn rtc_glitch_det_int_st(&self) -> RTC_GLITCH_DET_INT_ST_R {
        RTC_GLITCH_DET_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - bbpll cal end interrupt state"]
    #[inline(always)]
    pub fn rtc_bbpll_cal_int_st(&self) -> RTC_BBPLL_CAL_INT_ST_R {
        RTC_BBPLL_CAL_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st_rtc](index.html) module"]
pub struct INT_ST_RTC_SPEC;
impl crate::RegisterSpec for INT_ST_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st_rtc::R](R) reader structure"]
impl crate::Readable for INT_ST_RTC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST_RTC to value 0"]
impl crate::Resettable for INT_ST_RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
