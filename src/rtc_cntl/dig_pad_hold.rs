#[doc = "Register `DIG_PAD_HOLD` reader"]
pub struct R(crate::R<DIG_PAD_HOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_PAD_HOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_PAD_HOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_PAD_HOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_PAD_HOLD` writer"]
pub struct W(crate::W<DIG_PAD_HOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_PAD_HOLD_SPEC>;
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
impl From<crate::W<DIG_PAD_HOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_PAD_HOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_PAD_HOLD` reader - the configure of digital pad"]
pub struct DIG_PAD_HOLD_R(crate::FieldReader<u32, u32>);
impl DIG_PAD_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIG_PAD_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_PAD_HOLD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_PAD_HOLD` writer - the configure of digital pad"]
pub struct DIG_PAD_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_PAD_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - the configure of digital pad"]
    #[inline(always)]
    pub fn dig_pad_hold(&self) -> DIG_PAD_HOLD_R {
        DIG_PAD_HOLD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - the configure of digital pad"]
    #[inline(always)]
    pub fn dig_pad_hold(&mut self) -> DIG_PAD_HOLD_W {
        DIG_PAD_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pad_hold](index.html) module"]
pub struct DIG_PAD_HOLD_SPEC;
impl crate::RegisterSpec for DIG_PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_pad_hold::R](R) reader structure"]
impl crate::Readable for DIG_PAD_HOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_pad_hold::W](W) writer structure"]
impl crate::Writable for DIG_PAD_HOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_PAD_HOLD to value 0"]
impl crate::Resettable for DIG_PAD_HOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
