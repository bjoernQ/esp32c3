#[doc = "Register `DMA_ENABLE` reader"]
pub struct R(crate::R<DMA_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ENABLE` writer"]
pub struct W(crate::W<DMA_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ENABLE_SPEC>;
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
impl From<crate::W<DMA_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_ENABLE` reader - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
pub struct DMA_ENABLE_R(crate::FieldReader<bool, bool>);
impl DMA_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ENABLE` writer - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0: typical AES working mode, 1'b1: DMA-AES working mode."]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA-AES working mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enable](index.html) module"]
pub struct DMA_ENABLE_SPEC;
impl crate::RegisterSpec for DMA_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_enable::R](R) reader structure"]
impl crate::Readable for DMA_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_enable::W](W) writer structure"]
impl crate::Writable for DMA_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ENABLE to value 0"]
impl crate::Resettable for DMA_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
