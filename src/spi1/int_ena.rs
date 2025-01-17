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
#[doc = "Field `PER_END_INT_ENA` reader - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub struct PER_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PER_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_END_INT_ENA` writer - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub struct PER_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_END_INT_ENA_W<'a> {
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
#[doc = "Field `PES_END_INT_ENA` reader - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub struct PES_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PES_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_END_INT_ENA` writer - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub struct PES_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_END_INT_ENA_W<'a> {
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
#[doc = "Field `WPE_END_INT_ENA` reader - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub struct WPE_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl WPE_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPE_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPE_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPE_END_INT_ENA` writer - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
pub struct WPE_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WPE_END_INT_ENA_W<'a> {
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
#[doc = "Field `SLV_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub struct SLV_ST_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SLV_ST_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ST_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ST_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub struct SLV_ST_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ST_END_INT_ENA_W<'a> {
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
#[doc = "Field `MST_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub struct MST_ST_END_INT_ENA_R(crate::FieldReader<bool, bool>);
impl MST_ST_END_INT_ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_ST_END_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_ST_END_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub struct MST_ST_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_ST_END_INT_ENA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_ena(&self) -> PER_END_INT_ENA_R {
        PER_END_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_ena(&self) -> PES_END_INT_ENA_R {
        PES_END_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_ena(&self) -> WPE_END_INT_ENA_R {
        WPE_END_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_ena(&self) -> SLV_ST_END_INT_ENA_R {
        SLV_ST_END_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_ena(&self) -> MST_ST_END_INT_ENA_R {
        MST_ST_END_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_ena(&mut self) -> PER_END_INT_ENA_W {
        PER_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_ena(&mut self) -> PES_END_INT_ENA_W {
        PES_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_ena(&mut self) -> WPE_END_INT_ENA_W {
        WPE_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_ena(&mut self) -> SLV_ST_END_INT_ENA_W {
        SLV_ST_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_ena(&mut self) -> MST_ST_END_INT_ENA_W {
        MST_ST_END_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
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
