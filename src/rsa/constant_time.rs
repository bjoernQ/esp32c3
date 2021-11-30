#[doc = "Register `CONSTANT_TIME` reader"]
pub struct R(crate::R<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANT_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANT_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANT_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANT_TIME` writer"]
pub struct W(crate::W<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANT_TIME_SPEC>;
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
impl From<crate::W<CONSTANT_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANT_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANT_TIME` reader - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub struct CONSTANT_TIME_R(crate::FieldReader<bool, bool>);
impl CONSTANT_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONSTANT_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTANT_TIME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSTANT_TIME` writer - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
pub struct CONSTANT_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTANT_TIME_W<'a> {
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
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    pub fn constant_time(&self) -> CONSTANT_TIME_R {
        CONSTANT_TIME_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure this bit to 0 for acceleration. 0: with acceleration, 1: without acceleration(defalut)."]
    #[inline(always)]
    pub fn constant_time(&mut self) -> CONSTANT_TIME_W {
        CONSTANT_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA constant time option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constant_time](index.html) module"]
pub struct CONSTANT_TIME_SPEC;
impl crate::RegisterSpec for CONSTANT_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constant_time::R](R) reader structure"]
impl crate::Readable for CONSTANT_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constant_time::W](W) writer structure"]
impl crate::Writable for CONSTANT_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONSTANT_TIME to value 0x01"]
impl crate::Resettable for CONSTANT_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
