#[doc = "Register `REG_Q3_WORD0` reader"]
pub struct R(crate::R<REG_Q3_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_Q3_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_Q3_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_Q3_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_Q3_WORD0` writer"]
pub struct W(crate::W<REG_Q3_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_Q3_WORD0_SPEC>;
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
impl From<crate::W<REG_Q3_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_Q3_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q3_WORD0` reader - a"]
pub struct SEND_Q3_WORD0_R(crate::FieldReader<u32, u32>);
impl SEND_Q3_WORD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SEND_Q3_WORD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_Q3_WORD0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_Q3_WORD0` writer - a"]
pub struct SEND_Q3_WORD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_Q3_WORD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_q3_word0(&self) -> SEND_Q3_WORD0_R {
        SEND_Q3_WORD0_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_q3_word0(&mut self) -> SEND_Q3_WORD0_W {
        SEND_Q3_WORD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_q3_word0](index.html) module"]
pub struct REG_Q3_WORD0_SPEC;
impl crate::RegisterSpec for REG_Q3_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_q3_word0::R](R) reader structure"]
impl crate::Readable for REG_Q3_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_q3_word0::W](W) writer structure"]
impl crate::Writable for REG_Q3_WORD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_Q3_WORD0 to value 0"]
impl crate::Resettable for REG_Q3_WORD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
