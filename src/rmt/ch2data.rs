#[doc = "Register `CH2DATA` reader"]
pub struct R(crate::R<CH2DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH2DATA` reader - Reserved."]
pub struct CH2DATA_R(crate::FieldReader<u32, u32>);
impl CH2DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CH2DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn ch2data(&self) -> CH2DATA_R {
        CH2DATA_R::new(self.bits as u32)
    }
}
#[doc = "RMT_CH2DATA_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2data](index.html) module"]
pub struct CH2DATA_SPEC;
impl crate::RegisterSpec for CH2DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2data::R](R) reader structure"]
impl crate::Readable for CH2DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH2DATA to value 0"]
impl crate::Resettable for CH2DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
