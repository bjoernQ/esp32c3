#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_END_INT_CLR` writer - The clear bit for SPI_MEM_PER_END_INT interrupt."]
pub struct PER_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_END_INT_CLR_W<'a> {
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
#[doc = "Field `PES_END_INT_CLR` writer - The clear bit for SPI_MEM_PES_END_INT interrupt."]
pub struct PES_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_END_INT_CLR_W<'a> {
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
#[doc = "Field `WPE_END_INT_CLR` writer - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
pub struct WPE_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WPE_END_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub struct SLV_ST_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ST_END_INT_CLR_W<'a> {
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
#[doc = "Field `MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub struct MST_ST_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_ST_END_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end_int_clr(&mut self) -> PER_END_INT_CLR_W {
        PER_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - The clear bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end_int_clr(&mut self) -> PES_END_INT_CLR_W {
        PES_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - The clear bit for SPI_MEM_WPE_END_INT interrupt."]
    #[inline(always)]
    pub fn wpe_end_int_clr(&mut self) -> WPE_END_INT_CLR_W {
        WPE_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_clr(&mut self) -> SLV_ST_END_INT_CLR_W {
        SLV_ST_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_clr(&mut self) -> MST_ST_END_INT_CLR_W {
        MST_ST_END_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
