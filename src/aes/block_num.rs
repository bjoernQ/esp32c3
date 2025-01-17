#[doc = "Register `BLOCK_NUM` reader"]
pub struct R(crate::R<BLOCK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLOCK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLOCK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLOCK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLOCK_NUM` writer"]
pub struct W(crate::W<BLOCK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLOCK_NUM_SPEC>;
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
impl From<crate::W<BLOCK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLOCK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_NUM` reader - Those bits stores the number of Plaintext/ciphertext block."]
pub struct BLOCK_NUM_R(crate::FieldReader<u32, u32>);
impl BLOCK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BLOCK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_NUM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_NUM` writer - Those bits stores the number of Plaintext/ciphertext block."]
pub struct BLOCK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Those bits stores the number of Plaintext/ciphertext block."]
    #[inline(always)]
    pub fn block_num(&self) -> BLOCK_NUM_R {
        BLOCK_NUM_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the number of Plaintext/ciphertext block."]
    #[inline(always)]
    pub fn block_num(&mut self) -> BLOCK_NUM_W {
        BLOCK_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES block number register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [block_num](index.html) module"]
pub struct BLOCK_NUM_SPEC;
impl crate::RegisterSpec for BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [block_num::R](R) reader structure"]
impl crate::Readable for BLOCK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [block_num::W](W) writer structure"]
impl crate::Writable for BLOCK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLOCK_NUM to value 0"]
impl crate::Resettable for BLOCK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
