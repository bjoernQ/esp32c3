#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDC_DATE` reader - reg_ledc_date."]
pub struct LEDC_DATE_R(crate::FieldReader<u32, u32>);
impl LEDC_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LEDC_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_DATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_DATE` writer - reg_ledc_date."]
pub struct LEDC_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - reg_ledc_date."]
    #[inline(always)]
    pub fn ledc_date(&self) -> LEDC_DATE_R {
        LEDC_DATE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_ledc_date."]
    #[inline(always)]
    pub fn ledc_date(&mut self) -> LEDC_DATE_W {
        LEDC_DATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_DATE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATE to value 0x1906_1700"]
impl crate::Resettable for DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1906_1700
    }
}
