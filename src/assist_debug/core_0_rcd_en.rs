#[doc = "Register `CORE_0_RCD_EN` reader"]
pub struct R(crate::R<CORE_0_RCD_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_RCD_EN` writer"]
pub struct W(crate::W<CORE_0_RCD_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_RCD_EN_SPEC>;
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
impl From<crate::W<CORE_0_RCD_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_RCD_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_RCD_RECORDEN` reader - reg_core_0_rcd_recorden"]
pub struct CORE_0_RCD_RECORDEN_R(crate::FieldReader<bool, bool>);
impl CORE_0_RCD_RECORDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_RCD_RECORDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_RCD_RECORDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_RCD_RECORDEN` writer - reg_core_0_rcd_recorden"]
pub struct CORE_0_RCD_RECORDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_RCD_RECORDEN_W<'a> {
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
#[doc = "Field `CORE_0_RCD_PDEBUGEN` reader - reg_core_0_rcd_pdebugen"]
pub struct CORE_0_RCD_PDEBUGEN_R(crate::FieldReader<bool, bool>);
impl CORE_0_RCD_PDEBUGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_RCD_PDEBUGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_RCD_PDEBUGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_RCD_PDEBUGEN` writer - reg_core_0_rcd_pdebugen"]
pub struct CORE_0_RCD_PDEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_RCD_PDEBUGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reg_core_0_rcd_recorden"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> CORE_0_RCD_RECORDEN_R {
        CORE_0_RCD_RECORDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reg_core_0_rcd_pdebugen"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> CORE_0_RCD_PDEBUGEN_R {
        CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_core_0_rcd_recorden"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&mut self) -> CORE_0_RCD_RECORDEN_W {
        CORE_0_RCD_RECORDEN_W { w: self }
    }
    #[doc = "Bit 1 - reg_core_0_rcd_pdebugen"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&mut self) -> CORE_0_RCD_PDEBUGEN_W {
        CORE_0_RCD_PDEBUGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_en](index.html) module"]
pub struct CORE_0_RCD_EN_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_en::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_rcd_en::W](W) writer structure"]
impl crate::Writable for CORE_0_RCD_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_RCD_EN to value 0"]
impl crate::Resettable for CORE_0_RCD_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
