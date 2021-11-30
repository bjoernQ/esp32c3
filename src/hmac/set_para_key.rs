#[doc = "Register `SET_PARA_KEY` writer"]
pub struct W(crate::W<SET_PARA_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_PARA_KEY_SPEC>;
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
impl From<crate::W<SET_PARA_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_PARA_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_SET` writer - Set hmac parameter key."]
pub struct KEY_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - Set hmac parameter key."]
    #[inline(always)]
    pub fn key_set(&mut self) -> KEY_SET_W {
        KEY_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure key.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_para_key](index.html) module"]
pub struct SET_PARA_KEY_SPEC;
impl crate::RegisterSpec for SET_PARA_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_para_key::W](W) writer structure"]
impl crate::Writable for SET_PARA_KEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_PARA_KEY to value 0"]
impl crate::Resettable for SET_PARA_KEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}