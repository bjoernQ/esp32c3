#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_START_FORCE` reader - select software enable saradc sample"]
pub struct SARADC_START_FORCE_R(crate::FieldReader<bool, bool>);
impl SARADC_START_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_START_FORCE` writer - select software enable saradc sample"]
pub struct SARADC_START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SARADC_START` reader - software enable saradc sample"]
pub struct SARADC_START_R(crate::FieldReader<bool, bool>);
impl SARADC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_START` writer - software enable saradc sample"]
pub struct SARADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SARADC_SAR_CLK_GATED` reader - SAR clock gated"]
pub struct SARADC_SAR_CLK_GATED_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR_CLK_GATED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR_CLK_GATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_CLK_GATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_CLK_GATED` writer - SAR clock gated"]
pub struct SARADC_SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_CLK_GATED_W<'a> {
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
#[doc = "Field `SARADC_SAR_CLK_DIV` reader - SAR clock divider"]
pub struct SARADC_SAR_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SARADC_SAR_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAR_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_CLK_DIV` writer - SAR clock divider"]
pub struct SARADC_SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | ((value as u32 & 0xff) << 7);
        self.w
    }
}
#[doc = "Field `SARADC_SAR_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SARADC_SAR_PATT_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_SAR_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub struct SARADC_SAR_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub struct SARADC_SAR_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SARADC_SAR_PATT_P_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARADC_SAR_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_SAR_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_SAR_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub struct SARADC_SAR_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_SAR_PATT_P_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SARADC_XPD_SAR_FORCE` reader - force option to xpd sar blocks"]
pub struct SARADC_XPD_SAR_FORCE_R(crate::FieldReader<u8, u8>);
impl SARADC_XPD_SAR_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_XPD_SAR_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_XPD_SAR_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_XPD_SAR_FORCE` writer - force option to xpd sar blocks"]
pub struct SARADC_XPD_SAR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_XPD_SAR_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` reader - wait arbit signal stable after sar_done"]
pub struct SARADC_WAIT_ARB_CYCLE_R(crate::FieldReader<u8, u8>);
impl SARADC_WAIT_ARB_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SARADC_WAIT_ARB_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SARADC_WAIT_ARB_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARADC_WAIT_ARB_CYCLE` writer - wait arbit signal stable after sar_done"]
pub struct SARADC_WAIT_ARB_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARADC_WAIT_ARB_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - select software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start_force(&self) -> SARADC_START_FORCE_R {
        SARADC_START_FORCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start(&self) -> SARADC_START_R {
        SARADC_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SAR clock gated"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&self) -> SARADC_SAR_CLK_GATED_R {
        SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&self) -> SARADC_SAR_CLK_DIV_R {
        SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar_patt_len(&self) -> SARADC_SAR_PATT_LEN_R {
        SARADC_SAR_PATT_LEN_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar_patt_p_clear(&self) -> SARADC_SAR_PATT_P_CLEAR_R {
        SARADC_SAR_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    pub fn saradc_xpd_sar_force(&self) -> SARADC_XPD_SAR_FORCE_R {
        SARADC_XPD_SAR_FORCE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn saradc_wait_arb_cycle(&self) -> SARADC_WAIT_ARB_CYCLE_R {
        SARADC_WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - select software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start_force(&mut self) -> SARADC_START_FORCE_W {
        SARADC_START_FORCE_W { w: self }
    }
    #[doc = "Bit 1 - software enable saradc sample"]
    #[inline(always)]
    pub fn saradc_start(&mut self) -> SARADC_START_W {
        SARADC_START_W { w: self }
    }
    #[doc = "Bit 6 - SAR clock gated"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&mut self) -> SARADC_SAR_CLK_GATED_W {
        SARADC_SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&mut self) -> SARADC_SAR_CLK_DIV_W {
        SARADC_SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 15:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar_patt_len(&mut self) -> SARADC_SAR_PATT_LEN_W {
        SARADC_SAR_PATT_LEN_W { w: self }
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar_patt_p_clear(&mut self) -> SARADC_SAR_PATT_P_CLEAR_W {
        SARADC_SAR_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    pub fn saradc_xpd_sar_force(&mut self) -> SARADC_XPD_SAR_FORCE_W {
        SARADC_XPD_SAR_FORCE_W { w: self }
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn saradc_wait_arb_cycle(&mut self) -> SARADC_WAIT_ARB_CYCLE_W {
        SARADC_WAIT_ARB_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x4003_8240"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4003_8240
    }
}
