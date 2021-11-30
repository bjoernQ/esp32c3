#[doc = "Register `M_PRIME` reader"]
pub struct R(crate::R<M_PRIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_PRIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_PRIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_PRIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M_PRIME` writer"]
pub struct W(crate::W<M_PRIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M_PRIME_SPEC>;
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
impl From<crate::W<M_PRIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M_PRIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_PRIME` reader - Those bits stores m'"]
pub struct M_PRIME_R(crate::FieldReader<u32, u32>);
impl M_PRIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        M_PRIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M_PRIME_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M_PRIME` writer - Those bits stores m'"]
pub struct M_PRIME_W<'a> {
    w: &'a mut W,
}
impl<'a> M_PRIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Those bits stores m'"]
    #[inline(always)]
    pub fn m_prime(&self) -> M_PRIME_R {
        M_PRIME_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores m'"]
    #[inline(always)]
    pub fn m_prime(&mut self) -> M_PRIME_W {
        M_PRIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSA M_prime register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_prime](index.html) module"]
pub struct M_PRIME_SPEC;
impl crate::RegisterSpec for M_PRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_prime::R](R) reader structure"]
impl crate::Readable for M_PRIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m_prime::W](W) writer structure"]
impl crate::Writable for M_PRIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M_PRIME to value 0"]
impl crate::Resettable for M_PRIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
