#[doc = "Register `IN_PRI_CH2` reader"]
pub struct R(crate::R<IN_PRI_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_PRI_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_PRI_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_PRI_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_PRI_CH2` writer"]
pub struct W(crate::W<IN_PRI_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_PRI_CH2_SPEC>;
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
impl From<crate::W<IN_PRI_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_PRI_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_PRI_CH2` reader - The priority of Rx channel 2. The larger of the value, the higher of the priority."]
pub struct RX_PRI_CH2_R(crate::FieldReader<u8, u8>);
impl RX_PRI_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_PRI_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PRI_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PRI_CH2` writer - The priority of Rx channel 2. The larger of the value, the higher of the priority."]
pub struct RX_PRI_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PRI_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The priority of Rx channel 2. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri_ch2(&self) -> RX_PRI_CH2_R {
        RX_PRI_CH2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Rx channel 2. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri_ch2(&mut self) -> RX_PRI_CH2_W {
        RX_PRI_CH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_PRI_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_pri_ch2](index.html) module"]
pub struct IN_PRI_CH2_SPEC;
impl crate::RegisterSpec for IN_PRI_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_pri_ch2::R](R) reader structure"]
impl crate::Readable for IN_PRI_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_pri_ch2::W](W) writer structure"]
impl crate::Writable for IN_PRI_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_PRI_CH2 to value 0"]
impl crate::Resettable for IN_PRI_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
