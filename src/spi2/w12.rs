#[doc = "Register `W12` reader"]
pub struct R(crate::R<W12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W12` writer"]
pub struct W(crate::W<W12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W12_SPEC>;
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
impl From<crate::W<W12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF12` reader - data buffer"]
pub struct BUF12_R(crate::FieldReader<u32, u32>);
impl BUF12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF12_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF12` writer - data buffer"]
pub struct BUF12_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf12(&self) -> BUF12_R {
        BUF12_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf12(&mut self) -> BUF12_W {
        BUF12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w12](index.html) module"]
pub struct W12_SPEC;
impl crate::RegisterSpec for W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w12::R](R) reader structure"]
impl crate::Readable for W12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w12::W](W) writer structure"]
impl crate::Writable for W12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W12 to value 0"]
impl crate::Resettable for W12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
