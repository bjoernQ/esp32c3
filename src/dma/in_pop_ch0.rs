#[doc = "Register `IN_POP_CH0` reader"]
pub struct R(crate::R<IN_POP_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_POP_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_POP_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_POP_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_POP_CH0` writer"]
pub struct W(crate::W<IN_POP_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_POP_CH0_SPEC>;
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
impl From<crate::W<IN_POP_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_POP_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA_CH0` reader - This register stores the data popping from DMA FIFO."]
pub struct INFIFO_RDATA_CH0_R(crate::FieldReader<u16, u16>);
impl INFIFO_RDATA_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INFIFO_RDATA_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_RDATA_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_POP_CH0` reader - Set this bit to pop data from DMA FIFO."]
pub struct INFIFO_POP_CH0_R(crate::FieldReader<bool, bool>);
impl INFIFO_POP_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFIFO_POP_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFIFO_POP_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFIFO_POP_CH0` writer - Set this bit to pop data from DMA FIFO."]
pub struct INFIFO_POP_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> INFIFO_POP_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This register stores the data popping from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch0(&self) -> INFIFO_RDATA_CH0_R {
        INFIFO_RDATA_CH0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Set this bit to pop data from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_pop_ch0(&self) -> INFIFO_POP_CH0_R {
        INFIFO_POP_CH0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to pop data from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_pop_ch0(&mut self) -> INFIFO_POP_CH0_W {
        INFIFO_POP_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_POP_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_pop_ch0](index.html) module"]
pub struct IN_POP_CH0_SPEC;
impl crate::RegisterSpec for IN_POP_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_pop_ch0::R](R) reader structure"]
impl crate::Readable for IN_POP_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_pop_ch0::W](W) writer structure"]
impl crate::Writable for IN_POP_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_POP_CH0 to value 0x0800"]
impl crate::Resettable for IN_POP_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
