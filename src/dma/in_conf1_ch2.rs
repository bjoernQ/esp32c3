#[doc = "Register `IN_CONF1_CH2` reader"]
pub struct R(crate::R<IN_CONF1_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF1_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF1_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF1_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF1_CH2` writer"]
pub struct W(crate::W<IN_CONF1_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF1_CH2_SPEC>;
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
impl From<crate::W<IN_CONF1_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF1_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_CHECK_OWNER_CH2` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct IN_CHECK_OWNER_CH2_R(crate::FieldReader<bool, bool>);
impl IN_CHECK_OWNER_CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_CHECK_OWNER_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_CHECK_OWNER_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_CHECK_OWNER_CH2` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub struct IN_CHECK_OWNER_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_CHECK_OWNER_CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch2(&self) -> IN_CHECK_OWNER_CH2_R {
        IN_CHECK_OWNER_CH2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner_ch2(&mut self) -> IN_CHECK_OWNER_CH2_W {
        IN_CHECK_OWNER_CH2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF1_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf1_ch2](index.html) module"]
pub struct IN_CONF1_CH2_SPEC;
impl crate::RegisterSpec for IN_CONF1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf1_ch2::R](R) reader structure"]
impl crate::Readable for IN_CONF1_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf1_ch2::W](W) writer structure"]
impl crate::Writable for IN_CONF1_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF1_CH2 to value 0"]
impl crate::Resettable for IN_CONF1_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
