#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_ENA` reader - saradc thres1 low interrupt enable"]
pub struct APB_SARADC_THRES1_LOW_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES1_LOW_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES1_LOW_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES1_LOW_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_ENA` writer - saradc thres1 low interrupt enable"]
pub struct APB_SARADC_THRES1_LOW_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_LOW_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_ENA` reader - saradc thres0 low interrupt enable"]
pub struct APB_SARADC_THRES0_LOW_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES0_LOW_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES0_LOW_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES0_LOW_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_ENA` writer - saradc thres0 low interrupt enable"]
pub struct APB_SARADC_THRES0_LOW_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_LOW_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_ENA` reader - saradc thres1 high interrupt enable"]
pub struct APB_SARADC_THRES1_HIGH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES1_HIGH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES1_HIGH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES1_HIGH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_ENA` writer - saradc thres1 high interrupt enable"]
pub struct APB_SARADC_THRES1_HIGH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES1_HIGH_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_ENA` reader - saradc thres0 high interrupt enable"]
pub struct APB_SARADC_THRES0_HIGH_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_THRES0_HIGH_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_THRES0_HIGH_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_THRES0_HIGH_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_ENA` writer - saradc thres0 high interrupt enable"]
pub struct APB_SARADC_THRES0_HIGH_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_THRES0_HIGH_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` reader - saradc2 done interrupt enable"]
pub struct APB_SARADC2_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC2_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC2_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC2_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` writer - saradc2 done interrupt enable"]
pub struct APB_SARADC2_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC2_DONE_INT_ENA_W<'a> {
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
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` reader - saradc1 done interrupt enable"]
pub struct APB_SARADC1_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APB_SARADC1_DONE_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC1_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC1_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` writer - saradc1 done interrupt enable"]
pub struct APB_SARADC1_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC1_DONE_INT_ENA_W<'a> {
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
    #[doc = "Bit 26 - saradc thres1 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_ena(&self) -> APB_SARADC_THRES1_LOW_INT_ENA_R {
        APB_SARADC_THRES1_LOW_INT_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_ena(&self) -> APB_SARADC_THRES0_LOW_INT_ENA_R {
        APB_SARADC_THRES0_LOW_INT_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_ena(&self) -> APB_SARADC_THRES1_HIGH_INT_ENA_R {
        APB_SARADC_THRES1_HIGH_INT_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_ena(&self) -> APB_SARADC_THRES0_HIGH_INT_ENA_R {
        APB_SARADC_THRES0_HIGH_INT_ENA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&self) -> APB_SARADC2_DONE_INT_ENA_R {
        APB_SARADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&self) -> APB_SARADC1_DONE_INT_ENA_R {
        APB_SARADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - saradc thres1 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_ena(&mut self) -> APB_SARADC_THRES1_LOW_INT_ENA_W {
        APB_SARADC_THRES1_LOW_INT_ENA_W { w: self }
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_ena(&mut self) -> APB_SARADC_THRES0_LOW_INT_ENA_W {
        APB_SARADC_THRES0_LOW_INT_ENA_W { w: self }
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_ena(&mut self) -> APB_SARADC_THRES1_HIGH_INT_ENA_W {
        APB_SARADC_THRES1_HIGH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_ena(&mut self) -> APB_SARADC_THRES0_HIGH_INT_ENA_W {
        APB_SARADC_THRES0_HIGH_INT_ENA_W { w: self }
    }
    #[doc = "Bit 30 - saradc2 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&mut self) -> APB_SARADC2_DONE_INT_ENA_W {
        APB_SARADC2_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Bit 31 - saradc1 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&mut self) -> APB_SARADC1_DONE_INT_ENA_W {
        APB_SARADC1_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc int register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
