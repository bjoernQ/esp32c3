#[doc = "Register `RTC_SLOW_CLK_CONF` reader"]
pub struct R(crate::R<RTC_SLOW_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SLOW_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SLOW_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SLOW_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SLOW_CLK_CONF` writer"]
pub struct W(crate::W<RTC_SLOW_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SLOW_CLK_CONF_SPEC>;
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
impl From<crate::W<RTC_SLOW_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SLOW_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ANA_CLK_DIV_VLD` reader - used to sync div bus. clear vld before set reg_rtc_ana_clk_div"]
pub struct RTC_ANA_CLK_DIV_VLD_R(crate::FieldReader<bool, bool>);
impl RTC_ANA_CLK_DIV_VLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ANA_CLK_DIV_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ANA_CLK_DIV_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ANA_CLK_DIV_VLD` writer - used to sync div bus. clear vld before set reg_rtc_ana_clk_div"]
pub struct RTC_ANA_CLK_DIV_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ANA_CLK_DIV_VLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RTC_ANA_CLK_DIV` reader - the clk divider num of RTC_CLK"]
pub struct RTC_ANA_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl RTC_ANA_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_ANA_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ANA_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ANA_CLK_DIV` writer - the clk divider num of RTC_CLK"]
pub struct RTC_ANA_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ANA_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 23)) | ((value as u32 & 0xff) << 23);
        self.w
    }
}
#[doc = "Field `RTC_SLOW_CLK_NEXT_EDGE` reader - flag rtc_slow_clk_next_edge"]
pub struct RTC_SLOW_CLK_NEXT_EDGE_R(crate::FieldReader<bool, bool>);
impl RTC_SLOW_CLK_NEXT_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SLOW_CLK_NEXT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SLOW_CLK_NEXT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SLOW_CLK_NEXT_EDGE` writer - flag rtc_slow_clk_next_edge"]
pub struct RTC_SLOW_CLK_NEXT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SLOW_CLK_NEXT_EDGE_W<'a> {
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
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div"]
    #[inline(always)]
    pub fn rtc_ana_clk_div_vld(&self) -> RTC_ANA_CLK_DIV_VLD_R {
        RTC_ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:30 - the clk divider num of RTC_CLK"]
    #[inline(always)]
    pub fn rtc_ana_clk_div(&self) -> RTC_ANA_CLK_DIV_R {
        RTC_ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - flag rtc_slow_clk_next_edge"]
    #[inline(always)]
    pub fn rtc_slow_clk_next_edge(&self) -> RTC_SLOW_CLK_NEXT_EDGE_R {
        RTC_SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div"]
    #[inline(always)]
    pub fn rtc_ana_clk_div_vld(&mut self) -> RTC_ANA_CLK_DIV_VLD_W {
        RTC_ANA_CLK_DIV_VLD_W { w: self }
    }
    #[doc = "Bits 23:30 - the clk divider num of RTC_CLK"]
    #[inline(always)]
    pub fn rtc_ana_clk_div(&mut self) -> RTC_ANA_CLK_DIV_W {
        RTC_ANA_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 31 - flag rtc_slow_clk_next_edge"]
    #[inline(always)]
    pub fn rtc_slow_clk_next_edge(&mut self) -> RTC_SLOW_CLK_NEXT_EDGE_W {
        RTC_SLOW_CLK_NEXT_EDGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_slow_clk_conf](index.html) module"]
pub struct RTC_SLOW_CLK_CONF_SPEC;
impl crate::RegisterSpec for RTC_SLOW_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_slow_clk_conf::R](R) reader structure"]
impl crate::Readable for RTC_SLOW_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_slow_clk_conf::W](W) writer structure"]
impl crate::Writable for RTC_SLOW_CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SLOW_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for RTC_SLOW_CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0000
    }
}
