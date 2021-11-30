#[doc = "Register `AHB_TEST` reader"]
pub struct R(crate::R<AHB_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_TEST` writer"]
pub struct W(crate::W<AHB_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_TEST_SPEC>;
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
impl From<crate::W<AHB_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB_TESTMODE` reader - reserved"]
pub struct AHB_TESTMODE_R(crate::FieldReader<u8, u8>);
impl AHB_TESTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_TESTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_TESTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_TESTMODE` writer - reserved"]
pub struct AHB_TESTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_TESTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `AHB_TESTADDR` reader - reserved"]
pub struct AHB_TESTADDR_R(crate::FieldReader<u8, u8>);
impl AHB_TESTADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_TESTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_TESTADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_TESTADDR` writer - reserved"]
pub struct AHB_TESTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_TESTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - reserved"]
    #[inline(always)]
    pub fn ahb_testmode(&self) -> AHB_TESTMODE_R {
        AHB_TESTMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - reserved"]
    #[inline(always)]
    pub fn ahb_testaddr(&self) -> AHB_TESTADDR_R {
        AHB_TESTADDR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - reserved"]
    #[inline(always)]
    pub fn ahb_testmode(&mut self) -> AHB_TESTMODE_W {
        AHB_TESTMODE_W { w: self }
    }
    #[doc = "Bits 4:5 - reserved"]
    #[inline(always)]
    pub fn ahb_testaddr(&mut self) -> AHB_TESTADDR_W {
        AHB_TESTADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_AHB_TEST_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_test](index.html) module"]
pub struct AHB_TEST_SPEC;
impl crate::RegisterSpec for AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_test::R](R) reader structure"]
impl crate::Readable for AHB_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_test::W](W) writer structure"]
impl crate::Writable for AHB_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_TEST to value 0"]
impl crate::Resettable for AHB_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
