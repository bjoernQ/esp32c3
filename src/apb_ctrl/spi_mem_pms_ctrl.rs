#[doc = "Register `SPI_MEM_PMS_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_PMS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_PMS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_PMS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_PMS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_PMS_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_PMS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_PMS_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_PMS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_PMS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_REJECT_INT` reader - reg_spi_mem_reject_int"]
pub struct SPI_MEM_REJECT_INT_R(crate::FieldReader<bool, bool>);
impl SPI_MEM_REJECT_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_MEM_REJECT_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MEM_REJECT_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_MEM_REJECT_CLR` writer - reg_spi_mem_reject_clr"]
pub struct SPI_MEM_REJECT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MEM_REJECT_CLR_W<'a> {
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
#[doc = "Field `SPI_MEM_REJECT_CDE` reader - reg_spi_mem_reject_cde"]
pub struct SPI_MEM_REJECT_CDE_R(crate::FieldReader<u8, u8>);
impl SPI_MEM_REJECT_CDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_MEM_REJECT_CDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MEM_REJECT_CDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - reg_spi_mem_reject_int"]
    #[inline(always)]
    pub fn spi_mem_reject_int(&self) -> SPI_MEM_REJECT_INT_R {
        SPI_MEM_REJECT_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - reg_spi_mem_reject_cde"]
    #[inline(always)]
    pub fn spi_mem_reject_cde(&self) -> SPI_MEM_REJECT_CDE_R {
        SPI_MEM_REJECT_CDE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - reg_spi_mem_reject_clr"]
    #[inline(always)]
    pub fn spi_mem_reject_clr(&mut self) -> SPI_MEM_REJECT_CLR_W {
        SPI_MEM_REJECT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_pms_ctrl](index.html) module"]
pub struct SPI_MEM_PMS_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_PMS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_pms_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_pms_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_PMS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_PMS_CTRL to value 0"]
impl crate::Resettable for SPI_MEM_PMS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
