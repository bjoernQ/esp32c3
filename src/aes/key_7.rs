#[doc = "Register `KEY_7` reader"]
pub struct R(crate::R<KEY_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_7` writer"]
pub struct W(crate::W<KEY_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_7_SPEC>;
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
impl From<crate::W<KEY_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_7` reader - This bits stores key_7 that is a part of key material."]
pub struct KEY_7_R(crate::FieldReader<u32, u32>);
impl KEY_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KEY_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_7` writer - This bits stores key_7 that is a part of key material."]
pub struct KEY_7_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This bits stores key_7 that is a part of key material."]
    #[inline(always)]
    pub fn key_7(&self) -> KEY_7_R {
        KEY_7_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_7 that is a part of key material."]
    #[inline(always)]
    pub fn key_7(&mut self) -> KEY_7_W {
        KEY_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key material key_7 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_7](index.html) module"]
pub struct KEY_7_SPEC;
impl crate::RegisterSpec for KEY_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_7::R](R) reader structure"]
impl crate::Readable for KEY_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_7::W](W) writer structure"]
impl crate::Writable for KEY_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_7 to value 0"]
impl crate::Resettable for KEY_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
