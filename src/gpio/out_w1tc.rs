#[doc = "Register `OUT_W1TC` writer"]
pub struct W(crate::W<OUT_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_W1TC_SPEC>;
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
impl From<crate::W<OUT_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_W1TC` writer - GPIO output clear register for GPIO0-25"]
pub struct OUT_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:25 - GPIO output clear register for GPIO0-25"]
    #[inline(always)]
    pub fn out_w1tc(&mut self) -> OUT_W1TC_W {
        OUT_W1TC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_w1tc](index.html) module"]
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_w1tc::W](W) writer structure"]
impl crate::Writable for OUT_W1TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OUT_W1TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}